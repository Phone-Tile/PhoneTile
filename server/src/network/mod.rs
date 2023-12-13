#![allow(unused)]
use log::{error, info, warn};
use std::io::{self, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time;
use std::vec::Vec;

mod connection;
pub mod packet;
mod pipe;
pub mod player;
mod room;

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

//////////////////////////////////////////////
///
///
/// Local structure for the pipe system, I might move it right now
///
///
//////////////////////////////////////////////

/// This structure save the handler and the pipes for game threads
struct LocalGame {
    handle: thread::JoinHandle<Result<(), Error>>,
    token: u16,
    sender: mpsc::Sender<pipe::ServerMessage>,
}

/// This structure save the handler for user threads
struct LocalConnection {
    handle: thread::JoinHandle<()>,
    token: u16,
}

//////////////////////////////////////////////
///
///
/// Server
///
///
//////////////////////////////////////////////

pub struct Server {
    target: String,
    connections: Vec<LocalConnection>,
    games: Vec<LocalGame>,
    connection_token: u16,
    room_token: u16,
    sender: mpsc::Sender<pipe::ServerMessage>,
    receiver: mpsc::Receiver<pipe::ServerMessage>,
}

impl Server {
    //////////////////////////////////////////////
    ///
    ///
    /// Constants
    ///
    ///
    //////////////////////////////////////////////

    /// Constants defining max number of running games and active connected users
    const MAX_USERS: usize = 50;
    const MAX_GAMES: usize = 5;

    //////////////////////////////////////////////
    ///
    ///
    /// Manager
    ///
    ///
    //////////////////////////////////////////////

    pub fn new() -> Server {
        let (send, recv) = mpsc::channel();

        Server {
            target: "Server".to_string(),
            connections: Vec::with_capacity(Server::MAX_USERS),
            games: Vec::with_capacity(Server::MAX_GAMES),
            connection_token: 1,
            room_token: 1,
            sender: send,
            receiver: recv,
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
                            handle: self.first_handler(stream, &self.connection_token),
                            token: self.connection_token,
                        });
                        self.connection_token += 1;
                    } else {
                        match packet::Packet::error_message(self.connection_token)
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

    //////////////////////////////////////////////
    ///
    ///
    /// Helpers
    ///
    ///
    //////////////////////////////////////////////

    /// First handler of incomming connexions, is responsible to lauch the thread and build the local user structure
    fn first_handler(&self, stream: TcpStream, &token: &u16) -> thread::JoinHandle<()> {
        let sender = self.sender.clone();
        thread::spawn(move || {
            let mut c = connection::Connection::new(stream, token, sender);
            c.manager();
        })
    }

    fn handle_connection_pipe_message(&mut self, message: pipe::ServerMessage) {
        match message.flag {
            pipe::ServerMessageFlag::Create => {
                let (sender, receiver) = mpsc::channel();

                let mut game = room::Room::new(receiver, self.room_token);
                // game.add_player(message.sender);

                self.games.push(LocalGame {
                    handle: thread::spawn(move || game.manager()),
                    token: self.room_token,
                    sender: sender.clone(),
                });

                match sender.send(message) {
                    Ok(_) => {}
                    Err(e) => {
                        error!(target: self.target.as_str(), "room {} pipe disconnected after creation",self.room_token)
                    }
                }

                self.room_token += 1;
            }
            pipe::ServerMessageFlag::Join => {
                for g in self.games.iter() {
                    if g.token == message.room_token {
                        match g.sender.send(message) {
                            Ok(_) => {}
                            Err(e) => {
                                error!(target: self.target.as_str(), "room {} pipe disconnected",self.room_token)
                            }
                        }
                        return;
                    }
                }
                // TODO: Add error message in the communication protocol for this case !
                // Or at least send an general error message back to the client !
                warn!(target: self.target.as_str(), "Unable to locate the game {}", message.room_token);
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
}

//////////////////////////////////////////////
///
///
/// Log system, might go in main for other modules to use it too
///
///
//////////////////////////////////////////////

/// Log interface
struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{:<10}{:>30} -- {}",
                format!("[ {} ]", record.level()).as_str(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

//////////////////////////////////////////////
///
///
/// Tests
///
///
//////////////////////////////////////////////
pub mod client;
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
            let mut client = client::Network::connect(10., 10.12, 1020, 1000).unwrap();
            assert_eq!(client.create_room().unwrap(), 1_u16);
            thread::sleep(time::Duration::from_millis(1000));
            client.lock_room(client::Game::Test);
            loop {
                match client.get_status() {
                    client::Status::InLockRoom(r) => break,
                    _ => continue,
                }
            }
            client.launch_game().unwrap();
            loop {
                match client.get_status() {
                    client::Status::InGame => break,
                    _ => continue,
                }
            }
            let mut buffer = [1_u8; packet::MAX_DATA_SIZE];
            // assert!(client.recv(&mut buffer));
            // thread::sleep(time::Duration::from_millis(10000));
        });

        thread::sleep(time::Duration::from_millis(20));

        let client2 = thread::spawn(|| {
            let mut client = client::Network::connect(10., 10., 1000, 1000).unwrap();
            assert_eq!(client.create_room().unwrap(), 2_u16);
            thread::sleep(time::Duration::from_millis(200));
        });

        thread::sleep(time::Duration::from_millis(10));

        let client3 = thread::spawn(|| {
            let mut client = client::Network::connect(10., 10., 1000, 1000).unwrap();
            client.join_room(1).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
            loop {
                match client.get_status() {
                    client::Status::InLockRoom(_) => {}
                    client::Status::InGame => break,
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
    }
}
