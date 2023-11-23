#![allow(unused)]
use log::{error, info, warn};
use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time;
use std::vec::Vec;

mod connection;
mod game;
mod packet;
mod pipe;
pub mod player;

/// The general pipe system will be the following :
///
///     .------------ User1 <-----------.
///     |                               |
///     |                               |
///     v                               v
/// MainThread ---------------------> Game
///     ^                               ^
///     |                               |
///     |                               |
///     `------------- User2 <----------'
///
///

/// This structure save the handler and the pipes for game threads
struct LocalGame {
    handle: thread::JoinHandle<()>,
    tocken: u16,
    sender: mpsc::Sender<pipe::ServerMessage>,
}

/// This structure save the handler for user threads
struct LocalConnection {
    handle: thread::JoinHandle<()>,
    tocken: u16,
}

pub struct Server {
    target: String,
    connections: Vec<LocalConnection>,
    games: Vec<LocalGame>,
    connection_tocken: u16,
    room_tocken: u16,
    sender: mpsc::Sender<pipe::ServerMessage>,
    receiver: mpsc::Receiver<pipe::ServerMessage>,
}

impl Server {
    /// Constants defining max number of running games and active connected users
    const MAX_USERS: usize = 50;
    const MAX_GAMES: usize = 5;

    pub fn new() -> Server {
        let (send, recv) = mpsc::channel();

        Server {
            target: "Server".to_string(),
            connections: Vec::with_capacity(Server::MAX_USERS),
            games: Vec::with_capacity(Server::MAX_GAMES),
            connection_tocken: 1,
            room_tocken: 1,
            sender: send,
            receiver: recv,
        }
    }

    /// First handler of incomming connexions, is responsible to lauch the thread and build the local user structure
    fn first_handler(&self, stream: TcpStream, &tocken: &u16) -> thread::JoinHandle<()> {
        let sender = self.sender.clone();
        thread::spawn(move || {
            let mut c = connection::Connection::new(stream, tocken, sender);
            c.manager();
        })
    }

    fn handle_connection_pipe_message(&mut self, message: pipe::ServerMessage) {
        match message.flag {
            pipe::ServerMessageFlag::Create => {
                let (sender, receiver) = mpsc::channel();

                let mut game = game::Game::new(receiver, self.room_tocken);
                // game.add_player(message.sender);

                self.games.push(LocalGame {
                    handle: thread::spawn(move || game.manager()),
                    tocken: self.room_tocken,
                    sender: sender.clone(),
                });

                match sender.send(message) {
                    Ok(_) => {}
                    Err(e) => {
                        error!(target: self.target.as_str(), "room {} pipe disconnected after creation",self.room_tocken)
                    }
                }

                self.room_tocken += 1;
            }
            pipe::ServerMessageFlag::Join => {
                for g in self.games.iter() {
                    if g.tocken == message.room_tocken {
                        match g.sender.send(message) {
                            Ok(_) => {}
                            Err(e) => {
                                error!(target: self.target.as_str(), "room {} pipe disconnected",self.room_tocken)
                            }
                        }
                        return;
                    }
                }
                // TODO: Add error message in the communication protocol for this case !
                // Or at least send an general error message back to the client !
                warn!(target: self.target.as_str(), "Unable to locate the game {}", message.room_tocken);
            }
        }
    }

    fn handle_connection_pipe(&mut self) {
        loop {
            match self.receiver.try_recv() {
                Ok(m) => self.handle_connection_pipe_message(m),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("pipe sender dropped"), // should never happened
            }
        }
    }

    fn update_connections_status(&mut self) {
        let mut i: usize = 0;
        while i < self.connections.len() {
            if self.connections[i].handle.is_finished() {
                self.connections.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Launch the server
    pub fn launch_server(&mut self) -> std::io::Result<()> {
        let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info));
        info!(target: self.target.as_str(), "starting ...");

        let listener = TcpListener::bind("0.0.0.0:8888")?;
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");

        info!(target: self.target.as_str(), "started successfully");

        for stream in listener.incoming() {
            self.update_connections_status();
            self.handle_connection_pipe();
            match stream {
                Ok(mut stream) => {
                    match stream.peer_addr() {
                        Ok(addr) => {
                            info!(target: self.target.as_str(), "new incomming connection from {}", addr)
                        }
                        Err(e) => {
                            // This case is really so that the main thread is the most reliable possible
                            warn!(target: self.target.as_str(), "client unreachable : {e}");
                            continue;
                        }
                    };
                    if self.connections.len() < Server::MAX_USERS {
                        self.connections.push(LocalConnection {
                            handle: self.first_handler(stream, &self.connection_tocken),
                            tocken: self.connection_tocken,
                        });
                        self.connection_tocken += 1;
                    } else {
                        match packet::Packet::error_message(self.connection_tocken)
                            .send_packet(&mut stream)
                        {
                            Ok(_) => warn!(target: self.target.as_str(), "no thread available"),
                            Err(e) => {
                                warn!(target: self.target.as_str(), "couldn't disconnect client : {e}")
                            }
                        }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    thread::sleep(time::Duration::from_millis(10));
                    continue;
                }
                Err(error) => {
                    warn!(target: self.target.as_str(), "unexpected error : {:?}", error);
                }
            }
        }
        Ok(())
    }
}

/// Log interface
struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[ {} ] {:>30} -- {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

mod network;
#[cfg(test)]
mod tests {
    use super::*;
    use std::*;
    #[test]
    fn test_server_client_comm() {
        let _ = thread::spawn(|| {
            let mut server = Server::new();
            server.launch_server().unwrap();
        });

        thread::sleep(time::Duration::from_millis(100));

        let client1 = thread::spawn(|| {
            let mut client = network::Network::connect(10., 10., 1000, 1000);
            assert_eq!(client.create_room().unwrap(), 1_u16);
            thread::sleep(time::Duration::from_millis(1000));
            client.lock_room();
            loop {
                match client.get_status() {
                    network::Status::InLockRoom(r) => break,
                    _ => continue,
                }
            }
            client.launch_game().unwrap();
            loop {
                match client.get_status() {
                    network::Status::InGame => break,
                    _ => continue,
                }
            }
            let mut buffer = [1_u8; packet::MAX_DATA_SIZE];
            // assert!(client.recv(&mut buffer));
            // thread::sleep(time::Duration::from_millis(10000));
        });

        thread::sleep(time::Duration::from_millis(20));

        let client2 = thread::spawn(|| {
            let mut client = network::Network::connect(10., 10., 1000, 1000);
            assert_eq!(client.create_room().unwrap(), 2_u16);
            thread::sleep(time::Duration::from_millis(200));
        });

        thread::sleep(time::Duration::from_millis(10));

        let client3 = thread::spawn(|| {
            let mut client = network::Network::connect(10., 10., 1000, 1000);
            client.join_room(1).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
            loop {
                match client.get_status() {
                    network::Status::InLockRoom(_) => {}
                    network::Status::InGame => break,
                    _ => continue,
                }
            }
            let mut buffer = [1_u8; packet::MAX_DATA_SIZE];
            client.send(&buffer);
            thread::sleep(time::Duration::from_millis(100));
            // assert!(client.recv(&mut buffer));
            thread::sleep(time::Duration::from_millis(1000));
        });

        client1.join().unwrap();
        client2.join().unwrap();
        client3.join().unwrap();
        panic!("Make this test fail");
    }
}
