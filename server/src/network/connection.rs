use super::packet::{HEADER_SIZE, MAX_DATA_SIZE};
use super::pipe::{self, GameMessage};
use crate::network::packet;

use log::{error, info, warn};
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::sync::mpsc::{self, TryRecvError};
use std::time;
use std::{error, thread};

macro_rules! send_packet {
    ( $packet:expr, $s:expr ) => {
        {
            match $packet.send_packet(&mut $s.stream) {
                Ok(_) => {},
                Err(e) => {
                    warn!(target: $s.target.as_str(), "client disconnected : {}", e);
                    exit(0);
                }
            }
        }
    };
}

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

    // Receiver for the game thread to send us data
    my_recv: Option<mpsc::Receiver<GameMessage>>,
}

impl Connection {
    pub fn new(
        stream: TcpStream,
        tocken: u16,
        main_sender: mpsc::Sender<pipe::ServerMessage>,
    ) -> Self {
        let mut target: String = "".to_string();
        match stream.peer_addr() {
            Ok(addr) => target = format!("Client {tocken} ({})", addr),
            Err(e) => {
                error!(target: format!("Client {tocken} ()").as_str(), "client disconnected : {e}");
            }
        }

        Connection {
            status: Status::Created,
            target,
            session_tocken: tocken,
            room_tocken: 0,
            stream,
            main_sender,
            game_sender: None,
            my_recv: None,
        }
    }

    pub fn manager(&mut self) {
        match self.init_handshake() {
            Ok(_) => {}
            Err(e) => {
                error!(target: self.target.as_str(), "unabled to initiate handshake : {e}");
                exit(0);
            }
        }
        info!(target: self.target.as_str(), "Handshake done");

        'room: loop {
            let lock = self.handle_room_joining_message();
            info!(target: self.target.as_str(), "Join room {}", self.room_tocken);
            'game: loop {
                match self.wait_lock(&lock) {
                    Ok(rank) => {
                        self.send_rank(rank);
                        match self.wait_launch(&lock) {
                            Ok(_) => {}
                            Err(e) => {
                                error!(target: self.target.as_str(), "{e}");
                                send_packet!(
                                    packet::Packet::error_message(self.session_tocken),
                                    self
                                );
                                continue 'room;
                            }
                        }
                        match self.main_loop() {
                            Ok(_) => {}
                            Err(e) => {
                                error!(target: self.target.as_str(), "{e}");
                                send_packet!(
                                    packet::Packet::error_message(self.session_tocken),
                                    self
                                );
                                continue 'room;
                            }
                        }
                    }
                    Err(e) => {
                        error!(target: self.target.as_str(), "{e}");
                        continue 'room;
                    }
                }
            }
        }
    }

    fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            // try receive from the client
            if let Some(packet) = packet::Packet::try_recv_packet(&mut self.stream) {
                match &self.game_sender {
                    Some(sender) => match sender.send(pipe::GameMessage::data_message(packet.data))
                    {
                        Ok(_) => {}
                        Err(_) => {
                            send_packet!(packet::Packet::error_message(self.session_tocken), self);
                            break;
                        }
                    },
                    None => panic!("No sender !"),
                }
            }

            // try receive from the game
            let error = &self.my_recv.as_ref().unwrap().try_recv(); // is will
            match error {
                Ok(message) => {
                    let packet = packet::Packet::new(
                        packet::Flag::Transmit,
                        0,
                        self.session_tocken,
                        self.room_tocken,
                        message.data.unwrap(), // should never be None
                    );
                    send_packet!(packet, self);
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    return Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken"))
                }
            };
            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }

    fn wait_launch(&mut self, lock: &Lock) -> Result<(), Error> {
        match lock {
            Lock::Enabled => {
                // listen to stream
                match packet::Packet::recv_packet(&mut self.stream) {
                    Ok(_) => {}, // TODO : add packet sanity check
                    Err(_) => {
                        warn!(target: self.target.as_str(), "client disconnected");
                        exit(0);
                    },
                }
                match &self.game_sender {
                    Some(sender) => {
                        match sender.send(pipe::GameMessage::launch_message()) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(Error::new(
                                    ErrorKind::BrokenPipe,
                                    "pipe with game broken",
                                ))
                            }
                        }
                        match self.my_recv.as_ref().unwrap().recv() {
                            Ok(_) => {
                                let packet = packet::Packet::new(
                                    packet::Flag::Launch,
                                    0,
                                    self.session_tocken,
                                    self.room_tocken,
                                    [0_u8; packet::MAX_DATA_SIZE],
                                );
                                send_packet!(packet, self);
                            }
                            Err(e) => {
                                return Err(Error::new(
                                    ErrorKind::BrokenPipe,
                                    "pipe with game broken",
                                ))
                            }
                        }
                    }
                    None => panic!("No sender !"), // this should never happened
                }
            }
            Lock::Disabled => {
                // listen to game_receiver for lock message
                match self.my_recv.as_ref().unwrap().recv() {
                    Ok(_) => {
                        let packet = packet::Packet::new(
                            packet::Flag::Launch,
                            0,
                            self.session_tocken,
                            self.room_tocken,
                            [0_u8; packet::MAX_DATA_SIZE],
                        );
                        send_packet!(packet, self);
                    }
                    Err(e) => {
                        return Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken"))
                    }
                }
            }
        }
        Ok(())
    }

    fn send_rank(&mut self, rank: u8) {
        let mut tbl = [0_u8; packet::MAX_DATA_SIZE];
        tbl[0] = rank;
        let packet = packet::Packet::new(
            packet::Flag::Lock,
            0,
            self.session_tocken,
            self.room_tocken,
            tbl,
        );
        send_packet!(packet, self);
    }

    fn wait_lock(&mut self, lock: &Lock) -> Result<u8, Error> {
        match lock {
            Lock::Enabled => {
                // listen to stream
                match packet::Packet::recv_packet(&mut self.stream) {
                    Ok(_) => {}, // TODO : add packet sanity check
                    Err(_) => {
                        warn!(target: self.target.as_str(), "client disconnected");
                        exit(0);
                    },
                }
                let sender = self.game_sender.as_ref().unwrap();
                match sender.send(pipe::GameMessage::lock_message(0)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken"))
                    }
                }
                match self.my_recv.as_ref().unwrap().recv() {
                    Ok(m) => Ok(m.rank.unwrap()), // should never be None
                    Err(e) => Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken")),
                }
            }
            Lock::Disabled => {
                // listen to game_receiver for lock message
                match self.my_recv.as_ref().unwrap().recv() {
                    Ok(m) => Ok(m.rank.unwrap()), // should never be None
                    Err(e) => Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken")),
                }
            }
        }
    }

    fn handle_room_joining_message(&mut self) -> Lock {
        // two possible things : either we create a game, either we connect to one !
        match packet::Packet::recv_packet(&mut self.stream) {
            Ok(packet) => {
                let flag = packet.get_flag();
                let room_tocken = packet.room;
                let (sender, receiver) = mpsc::channel();
                match flag {
                    packet::Flag::Create => {
                        self.main_sender
                            .send(pipe::ServerMessage::new(
                                self.session_tocken,
                                pipe::ServerMessageFlag::Create,
                                0,
                                sender,
                            ))
                            .unwrap(); // should never happened
                        self.my_recv = Some(receiver);
                        match self.my_recv.as_ref().unwrap().recv() {
                            Ok(message) => {
                                self.room_tocken = message.room_tocken;
                                self.game_sender = message.sender;
                                let packet = packet::Packet::new(
                                    packet::Flag::Create,
                                    0,
                                    self.session_tocken,
                                    self.room_tocken,
                                    [0_u8; packet::MAX_DATA_SIZE],
                                );
                                send_packet!(packet, self);
                                Lock::Enabled
                            }
                            Err(_) => exit(0),
                        }
                    }
                    packet::Flag::Join => {
                        self.main_sender
                            .send(pipe::ServerMessage::new(
                                self.session_tocken,
                                pipe::ServerMessageFlag::Join,
                                room_tocken,
                                sender,
                            ))
                            .unwrap();
                        self.my_recv = Some(receiver);
                        match self.my_recv.as_ref().unwrap().recv() {
                            Ok(message) => {
                                self.room_tocken = message.room_tocken;
                                self.game_sender = message.sender;
                                let packet = packet::Packet::new(
                                    packet::Flag::Create,
                                    0,
                                    self.session_tocken,
                                    self.room_tocken,
                                    [0_u8; packet::MAX_DATA_SIZE],
                                );
                                send_packet!(packet, self);
                                Lock::Disabled
                            }
                            Err(_) => exit(0),
                        }
                    }
                    _ => {
                        error!(target: self.target.as_str(), "An unexpected packet was received");
                        exit(0);
                    }
                }
            }
            Err(e) => {
                warn!(target: self.target.as_str(), "{e}");
                exit(0);
            }
        }
    }

    /// Initial handshake
    fn init_handshake(&mut self) -> Result<(), Error> {
        match packet::Packet::recv_packet(&mut self.stream) {
            Ok(packet) => {
                send_packet!(packet, self);
                self.status = Status::Initialized;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
