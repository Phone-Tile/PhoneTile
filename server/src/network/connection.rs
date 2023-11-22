use crate::network::packet;
use super::packet::{MAX_DATA_SIZE, HEADER_SIZE};
use super::pipe::{self, GameMessage};

use std::net::TcpStream;
use std::process::exit;
use std::io::{Write, Read, ErrorKind};
use std::sync::mpsc::{self, TryRecvError};
use log::{info, error};
use std::thread;
use std::time;

#[derive(Clone)]
enum Lock {
    Enabled,
    Disabled,
}

enum Status {
    Created,
    Initialized,
}

pub struct Connection {
    status: Status,
    target: String,

    session_tocken: u16,
    room_tocken: u16,
    stream: TcpStream,
    // Sender for the main thread (game creation / join request)
    main_sender: mpsc::Sender<pipe::ServerMessage>,

    // Sender for the game thread (game oriented communication)
    game_sender: Option<mpsc::Sender<GameMessage>>,

    // Sender/Receiver pair for the game thread to send us data
    my_recv: mpsc::Receiver<GameMessage>,
    my_sender: mpsc::Sender<GameMessage>,
}

impl Connection {
    fn block_read_exact(&mut self, buf: &mut [u8]) {
        loop {
            match self.stream.read_exact(buf) {
                Ok(_) => return,
                Err(e) if e.kind() == ErrorKind::WouldBlock => {},
                Err(e) => panic!("{e}"),
            }
            thread::sleep(time::Duration::from_millis(30));
        }
    }

    pub fn new(stream: TcpStream, tocken: u16, main_sender: mpsc::Sender<pipe::ServerMessage>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let target: String = format!("Client {}", tocken);

        Connection {
            status: Status::Created,
            target,
            session_tocken: tocken,
            room_tocken: 0,
            stream,
            main_sender,
            game_sender: None,
            my_recv: receiver,
            my_sender: sender,
        }
    }

    pub fn manager(&mut self) {
        self.init_handshake().unwrap();
        
        info!(target: self.target.as_str(), "Handshake done");
        let lock = self.handle_room_joining_message();
        info!(target: self.target.as_str(), "Join room {}", self.room_tocken);
        let rank = self.wait_lock(lock.clone());
        self.send_rank(rank);
        self.wait_launch(lock);
        self.main_loop();
    }

    fn main_loop(&mut self) {
        loop {
            let mut buffer = [0_u8; packet::BUFFER_SIZE];

            // try receive from the client
            match self.stream.read_exact(&mut buffer) {
                Ok(_) => {
                    let packet = packet::Packet::unpack(&buffer).unwrap();
                    match &self.game_sender {
                        Some(sender) => sender.send(pipe::GameMessage::data_message(packet.data)).unwrap(),
                        None => panic!("No sender !"),
                    } 
                },
                Err(e) if e.kind() == ErrorKind::WouldBlock => {},
                Err(e) => panic!("{e}"),
            };

            // try receive from the game
            let error = self.my_recv.try_recv();
            match error {
                Ok(message) => {
                    let packet = packet::Packet::new(packet::Flag::Transmit as u8, 0, self.session_tocken, self.room_tocken, message.data.unwrap());
                    packet.pack(&mut buffer);
                    self.stream.write_all(&buffer).unwrap();
                }
                Err(TryRecvError::Empty) => {},
                Err(TryRecvError::Disconnected) => panic!("Client {} disconnected !", self.session_tocken),
            };
            thread::sleep(time::Duration::from_millis(10));
        };
    }

    fn wait_launch(&mut self, lock: Lock) {
        match lock {
            Lock::Enabled => {
                // listen to stream
                let mut buffer = [0_u8; MAX_DATA_SIZE+HEADER_SIZE];
                self.block_read_exact(&mut buffer);
                match &self.game_sender {
                    Some(sender) => {
                        sender.send(pipe::GameMessage::launch_message()).unwrap();
                        match self.my_recv.recv() {
                            Ok(_) => {
                                let mut buffer = [0_u8; MAX_DATA_SIZE+HEADER_SIZE];
                                packet::Packet::new(packet::Flag::Launch as u8, 0, self.session_tocken, self.room_tocken, [0_u8; packet::MAX_DATA_SIZE]).pack(&mut buffer);
                                self.stream.write_all(&buffer).unwrap();
                            },
                            Err(e) => panic!("{e}"),
                        }
                    },
                    None => panic!("No sender !"),
                }
            },
            Lock::Disabled => {
                // listen to game_receiver for lock message
                match self.my_recv.recv() {
                    Ok(_) => {
                        let mut buffer = [0_u8; MAX_DATA_SIZE+HEADER_SIZE];
                        packet::Packet::new(packet::Flag::Launch as u8, 0, self.session_tocken, self.room_tocken, [0_u8; packet::MAX_DATA_SIZE]).pack(&mut buffer);
                        self.stream.write_all(&buffer).unwrap();
                    },
                    Err(e) => panic!("{e}"),
                }
            },
        }
    }

    fn send_rank(&mut self, rank: u8) {
        let mut buffer = [0_u8; MAX_DATA_SIZE+HEADER_SIZE];
        let mut tbl = [0_u8; packet::MAX_DATA_SIZE];
        tbl[0] = rank;
        packet::Packet::new(packet::Flag::Lock as u8, 0, self.session_tocken, self.room_tocken, tbl).pack(&mut buffer);

        self.stream.write_all(&buffer).unwrap();
    }

    fn wait_lock(&mut self, lock: Lock) -> u8 {
        match lock {
            Lock::Enabled => {
                // listen to stream
                let mut buffer = [0_u8; MAX_DATA_SIZE+HEADER_SIZE];
                self.block_read_exact(&mut buffer);
                match &self.game_sender {
                    Some(sender) => {
                        sender.send(pipe::GameMessage::lock_message(0)).unwrap();
                        match self.my_recv.recv() {
                            Ok(m) => m.rank.unwrap(),
                            Err(e) => panic!("{e}"),
                        }
                    },
                    None => panic!("No sender !"),
                }
            },
            Lock::Disabled => {
                // listen to game_receiver for lock message
                match self.my_recv.recv() {
                    Ok(m) => m.rank.unwrap(),
                    Err(e) => panic!("{e}"),
                }
            },
        }
    }

    fn handle_room_joining_message(&mut self) -> Lock {
        let mut buffer = [0_u8; MAX_DATA_SIZE+HEADER_SIZE];

        // two possible things : either we create a game, either we connect to one !
        // self.stream.read_exact(&mut buffer).unwrap();
        self.block_read_exact(&mut buffer);
        match packet::Packet::unpack(&buffer) {
            Ok(packet) => {
                let flag = packet.get_flag();
                let room_tocken = packet.room;
                match flag {
                    packet::Flag::Create => {
                        self.main_sender.send(pipe::ServerMessage::new(self.session_tocken, pipe::ServerMessageFlag::Create, 0, self.my_sender.clone())).unwrap();
                        match self.my_recv.recv() {
                            Ok(message) => {
                                self.room_tocken = message.room_tocken;
                                self.game_sender = message.sender;
                                packet::Packet::new(packet::Flag::Create as u8, 0, self.session_tocken, self.room_tocken, [0_u8; packet::MAX_DATA_SIZE]).pack(&mut buffer);
                                self.stream.write_all(&buffer).unwrap();
                                Lock::Enabled
                            },
                            Err(_) => exit(0),
                        }
                    },
                    packet::Flag::Join => {
                        self.main_sender.send(pipe::ServerMessage::new(self.session_tocken, pipe::ServerMessageFlag::Join, room_tocken, self.my_sender.clone())).unwrap();
                        match self.my_recv.recv() {
                            Ok(message) => {
                                self.room_tocken = message.room_tocken;
                                self.game_sender = message.sender;
                                packet::Packet::new(packet::Flag::Create as u8, 0, self.session_tocken, self.room_tocken, [0_u8; packet::MAX_DATA_SIZE]).pack(&mut buffer);
                                self.stream.write_all(&buffer).unwrap();
                                Lock::Disabled
                            },
                            Err(_) => exit(0),
                        }
                    },
                    _ => {
                        error!(target: self.target.as_str(), "An unexpected packet was received");
                        exit(0);
                    },
                }
            }
            Err(e) => panic!("{e}"),
        }
    }
    
    /// Initial handshake
    fn init_handshake(&mut self) -> Result<(), &'static str> {
        let mut buffer = [0_u8; 2048];

        packet::Packet::new(packet::Flag::Init as u8, 0, self.session_tocken, 0, [0_u8; MAX_DATA_SIZE]).pack(&mut buffer);

        match self.stream.read_exact(&mut buffer) {
            Ok(())=> {
                packet::Packet::unpack(&buffer).unwrap();
                match self.stream.write_all(&buffer) {
                    Ok(_) => {
                        self.status = Status::Initialized;
                        Ok(())
                    }
                    Err(_) => Err("Unable to send data"),
                }
            },
            Err(_) => Err("Unable to receive data"),
        }
    }
}