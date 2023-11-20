use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::vec::Vec;
use std::io::{self, Write};
use std::sync::mpsc::{self, TryRecvError};

mod packet;
mod connection;
mod game;
mod pipe;
mod player;

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
    connections: Vec<LocalConnection>,
    games: Vec<LocalGame>,
    connection_tocken: u16,
    game_tocken: u16,
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
            connections: Vec::with_capacity(Server::MAX_USERS),
            games: Vec::with_capacity(Server::MAX_GAMES),
            connection_tocken: 0,
            game_tocken: 0,
            sender: send,
            receiver: recv,
        }
    }

    /// First handler of incomming connexions, is responsible to lauch the thread and build the local user structure
    fn first_handler(&self, stream: TcpStream, &tocken: &u16) -> thread::JoinHandle<()> {
        let sender = self.sender.clone();
        thread::spawn (move || {
            let mut c = connection::Connection::new(stream, tocken, sender);
            let _ = c.manager();
        })
    }

    fn handle_connection_pipe_message(&mut self, message: pipe::ServerMessage) {
        match message.flag {
            pipe::ServerMessageFlag::CREATE => {
                let (sender, receiver) = mpsc::channel();

                let mut game = game::Game::new(receiver);
                // game.add_player(message.sender);

                self.games.push(
                    LocalGame { handle: thread::spawn(move || {
                        game.manager()
                    }),
                    tocken: self.game_tocken,
                    sender: sender.clone() });

                sender.send(message);
            }
            pipe::ServerMessageFlag::JOIN => {
                for g in self.games.iter() {
                    if g.tocken == message.game_tocken {
                        g.sender.send(message);
                        return;
                    }
                }
                // TODO: Add error message in the communication protocol for this case !
                // Or at least send an general error message back to the client !
                println!("[\033[33m WARNING \033[97m] Client {:?} : Unabled to locate the game {:?}", message.usr_tocken, message.game_tocken);
            }
        }
    }

    fn handle_connection_pipe(&mut self) {
        loop {
            match self.receiver.try_recv() {
                Ok(m) => self.handle_connection_pipe_message(m),
                Err(e) if e == TryRecvError::Empty => break,
                Err(_) => panic!("Something weird happened !"),
            }
        }
    }
    
    fn update_connections_status(&mut self) {
        let mut i: usize = 0;
        while i<self.connections.len() {
            if self.connections[i].handle.is_finished() {
                self.connections.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }
    
    /// Launch the server
    pub fn launch_server(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:8080")?;
        listener.set_nonblocking(true).expect("Cannot set non-blocking");

        const ERROR_MESSAGE: [u8;packet::HEADER_SIZE] = [0, packet::Flag::Error as u8, 0, 0, 0, 0, 0, 0];

        println!("[\033[32m INFO \033[97m] Server started");

        for stream in listener.incoming() {
            self.update_connections_status();
            self.handle_connection_pipe();
            match stream {
                Ok(mut stream) => {
                    println!("[\033[32m INFO \033[97m] New incomming connection");
                    if self.connections.len() < Server::MAX_USERS {
                        self.connections.push(LocalConnection { handle: self.first_handler(stream, &self.connection_tocken), tocken: self.connection_tocken });
                        self.connection_tocken += 1;
                    } else {
                        let _ = stream.write(&ERROR_MESSAGE);
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    thread::sleep(time::Duration::from_millis(10));
                    continue;
                }
                Err(error) => println!("[\033[31m ERROR \033[97m] Problem handling the incomming connection :{:?}", error),
            }
        }
        Ok(())
    }
}

#[test]
fn test_server() {
    let mut s = Server::new();
    let _ = s.launch_server();
}
