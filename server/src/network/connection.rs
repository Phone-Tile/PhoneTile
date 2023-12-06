use super::client;
use super::packet::{HEADER_SIZE, MAX_DATA_SIZE};
use super::pipe::{self, GameMessage};
use crate::network::packet;

use log::{error, info, warn};
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::time;
use std::{error, thread};

/// TODO: I ave to implement the forwading of the game chosen to the room manager !!!!!!!!!!!!!!!!!!!
/// And choose when to do that .........

//////////////////////////////////////////////
///
///
/// Enums
///
///
//////////////////////////////////////////////

#[derive(Clone, Copy)]
enum Lock {
    Enabled,
    Disabled,
}

enum Status {
    Created,
    Initialized,
}

//////////////////////////////////////////////
///
///
/// Connection
///
///
//////////////////////////////////////////////

pub struct Connection {
    status: Status,
    target: String,

    session_token: u16,
    room_token: u16,
    game_id: client::Game,
    stream: TcpStream,

    physical_height: f32,
    physical_width: f32,
    window_height: u32,
    window_width: u32,

    // Sender for the main thread (game creation / join request)
    main_sender: mpsc::Sender<pipe::ServerMessage>,

    // Sender for the game thread (game oriented communication)
    game_sender: Option<mpsc::Sender<GameMessage>>,

    // Receiver for the game thread to send us data
    my_recv: Option<mpsc::Receiver<GameMessage>>,
}

impl Connection {
    //////////////////////////////////////////////
    ///
    ///
    /// Manager
    ///
    ///
    //////////////////////////////////////////////

    pub fn new(
        stream: TcpStream,
        token: u16,
        main_sender: mpsc::Sender<pipe::ServerMessage>,
    ) -> Self {
        let mut target: String = "".to_string();
        match stream.peer_addr() {
            Ok(addr) => target = format!("Client {token} ({})", addr),
            Err(e) => {
                error!(target: format!("Client {token} ()").as_str(), "client disconnected : {e}");
            }
        }

        Connection {
            status: Status::Created,
            target,
            session_token: token,
            room_token: 0,
            game_id: client::Game::Unknown,
            stream,
            physical_height: 0.,
            physical_width: 0.,
            window_height: 0,
            window_width: 0,
            main_sender,
            game_sender: None,
            my_recv: None,
        }
    }

    pub fn manager(&mut self) -> Result<(), Error> {
        match self.handshake() {
            Ok(_) => {}
            Err(e) => {
                error!(target: self.target.as_str(), "unabled to initiate handshake : {e}");
                return Ok(());
            }
        }
        info!(target: self.target.as_str(), "Handshake done");

        'room: loop {
            let lock = match self.join_room() {
                Ok(l) => l,
                Err(e)
                    if e.kind() == ErrorKind::InvalidInput
                        || e.kind() == ErrorKind::NotConnected =>
                {
                    warn!(target: self.target.as_str(), "{e}");
                    return Ok(());
                }
                Err(e) => {
                    error!(target: self.target.as_str(), "{e}");
                    self.send_packet(packet::Packet::error_message(self.session_token))?;
                    continue 'room;
                }
            };
            info!(target: self.target.as_str(), "Join room {}", self.room_token);
            'game: loop {
                match self.lock_room(&lock) {
                    Ok(rank) => {
                        self.send_ranks(rank);
                        match self.launch_game(&lock) {
                            Ok(_) => {}
                            Err(e) if e.kind() == ErrorKind::NotConnected => {
                                warn!(target: self.target.as_str(), "{e}");
                                return Err(Error::new(
                                    ErrorKind::NotConnected,
                                    "disconnected while in room",
                                ));
                            }
                            Err(e) => {
                                error!(target: self.target.as_str(), "{e}");
                                self.send_packet(packet::Packet::error_message(
                                    self.session_token,
                                ))?;
                                continue 'room;
                            }
                        }
                        match self.game_loop() {
                            Ok(_) => {}
                            Err(e) => {
                                error!(target: self.target.as_str(), "{e}");
                                self.send_packet(packet::Packet::error_message(
                                    self.session_token,
                                ))?;
                                continue 'room;
                            }
                        }
                    }
                    Err(e) if e.kind() == ErrorKind::NotConnected => {
                        warn!(target: self.target.as_str(), "{e}");
                        return Err(Error::new(
                            ErrorKind::NotConnected,
                            "disconnected while in room",
                        ));
                    }
                    Err(e) => {
                        error!(target: self.target.as_str(), "{e}");
                        continue 'room;
                    }
                }
            }
        }
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Stage functions
    ///
    ///
    //////////////////////////////////////////////

    /// Initial handshake
    fn handshake(&mut self) -> Result<(), Error> {
        match packet::Packet::recv_packet(&mut self.stream) {
            Ok(packet) => {
                packet.check_packet_flag(packet::Flag::Init)?;
                let mut tmp = [0_u8; 4];
                tmp.copy_from_slice(&packet.data[..4]);
                self.physical_height = f32::from_be_bytes(tmp);
                tmp.copy_from_slice(&packet.data[4..8]);
                self.physical_width = f32::from_be_bytes(tmp);
                tmp.copy_from_slice(&packet.data[8..12]);
                self.window_height = u32::from_be_bytes(tmp);
                tmp.copy_from_slice(&packet.data[12..16]);
                self.window_width = u32::from_be_bytes(tmp);

                self.send_packet(packet);
                self.status = Status::Initialized;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn join_room(&mut self) -> Result<Lock, Error> {
        // two possible things : either we create a game, either we connect to one !
        match packet::Packet::recv_packet(&mut self.stream) {
            Ok(packet) => {
                let flag = packet.get_flag();
                let room_token = packet.room;
                match flag {
                    packet::Flag::Create => self.join_room_with_create(),
                    packet::Flag::Join => self.join_room_with_join(room_token),
                    _ => Err(Error::new(
                        ErrorKind::InvalidInput,
                        "an unexpected packet was received",
                    )),
                }
            }
            Err(e) => Err(Error::new(ErrorKind::NotConnected, "client disconnected")),
        }
    }

    fn lock_room(&mut self, lock: &Lock) -> Result<u8, Error> {
        match lock {
            Lock::Enabled => {
                // listen to stream
                match packet::Packet::recv_packet(&mut self.stream) {
                    Ok(packet) => {
                        packet.check_packet_flag(packet::Flag::Lock)?;
                        self.game_id = packet.option.into();
                    }
                    Err(_) => {
                        return Err(Error::new(ErrorKind::NotConnected, "client disconnected"));
                    }
                }
                let sender = self.game_sender.as_ref().unwrap();
                match sender.send(pipe::GameMessage::lock_message(self.game_id.into())) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken"))
                    }
                }
                match self.my_recv.as_ref().unwrap().recv() {
                    Ok(m) => Ok(m.rank.unwrap() as u8), // should never be None
                    Err(e) => Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken")),
                }
            }
            Lock::Disabled => {
                // listen to game_receiver for lock message
                match self.my_recv.as_ref().unwrap().recv() {
                    Ok(m) => Ok(m.rank.unwrap() as u8), // should never be None
                    Err(e) => Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken")),
                }
            }
        }
    }

    fn send_ranks(&mut self, rank: u8) {
        let packet = packet::Packet::new(
            packet::Flag::Lock,
            0,
            self.session_token,
            self.room_token,
            &[],
            rank as u16,
        );
        self.send_packet(packet);
    }

    fn launch_game(&mut self, lock: &Lock) -> Result<(), Error> {
        match lock {
            Lock::Enabled => {
                // listen to stream
                match packet::Packet::recv_packet(&mut self.stream) {
                    Ok(packet) => {
                        packet.check_packet_flag(packet::Flag::Launch)?;
                    }
                    Err(_) => {
                        return Err(Error::new(ErrorKind::NotConnected, "client disconnected"));
                    }
                }
                let sender = self.game_sender.as_mut().expect("No sender !");
                match sender.send(pipe::GameMessage::launch_message()) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken"))
                    }
                }
                match self.my_recv.as_ref().unwrap().recv() {
                    Ok(_) => {
                        let packet = packet::Packet::new(
                            packet::Flag::Launch,
                            0,
                            self.session_token,
                            self.room_token,
                            &[],
                            0,
                        );
                        self.send_packet(packet)?;
                    }
                    Err(e) => {
                        return Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken"))
                    }
                }
            }
            Lock::Disabled => {
                // listen to game_receiver for lock message
                match self.my_recv.as_ref().unwrap().recv() {
                    Ok(_) => {
                        let packet = packet::Packet::new(
                            packet::Flag::Launch,
                            0,
                            self.session_token,
                            self.room_token,
                            &[],
                            0,
                        );
                        self.send_packet(packet)?;
                    }
                    Err(e) => {
                        return Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken"))
                    }
                }
            }
        }
        Ok(())
    }

    fn game_loop(&mut self) -> Result<(), Error> {
        loop {
            // try receive from the client
            if let Some(packet) = packet::Packet::try_recv_packet(&mut self.stream) {
                packet.check_packet_flag(packet::Flag::Transmit)?;
                match &self.game_sender {
                    Some(sender) => match sender
                        .send(pipe::GameMessage::data_message(packet.data, packet.size))
                    {
                        Ok(_) => {}
                        Err(_) => {
                            self.send_packet(packet::Packet::error_message(self.session_token))?;
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
                        self.session_token,
                        self.room_token,
                        &message.data.unwrap(), // should never be None
                        0,
                    );
                    self.send_packet(packet)?;
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

    //////////////////////////////////////////////
    ///
    ///
    /// Helpers
    ///
    ///
    //////////////////////////////////////////////

    fn send_packet(&mut self, packet: packet::Packet) -> Result<(), Error> {
        if let Err(e) = packet.send_packet(&mut self.stream) {
            return Err(Error::new(ErrorKind::NotConnected, "client disconnected"));
        }
        Ok(())
    }

    fn join_room_with_create(&mut self) -> Result<Lock, Error> {
        let (sender, receiver) = mpsc::channel();
        self.main_sender
            .send(pipe::ServerMessage {
                session_token: self.session_token,
                flag: pipe::ServerMessageFlag::Create,
                room_token: 0,
                sender,
                physical_height: self.physical_height,
                physical_width: self.physical_width,
                window_height: self.window_height,
                window_width: self.window_width,
            })
            .unwrap(); // should never happened
        self.my_recv = Some(receiver);
        match self.my_recv.as_ref().unwrap().recv() {
            Ok(message) => {
                self.room_token = message.room_token;
                self.game_sender = message.sender;
                let packet = packet::Packet::new(
                    packet::Flag::Create,
                    0,
                    self.session_token,
                    self.room_token,
                    &[],
                    0,
                );
                self.send_packet(packet);
                Ok(Lock::Enabled)
            }
            Err(_) => Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken")),
        }
    }

    fn join_room_with_join(&mut self, room_token: u16) -> Result<Lock, Error> {
        let (sender, receiver) = mpsc::channel();
        self.main_sender
            .send(pipe::ServerMessage {
                session_token: self.session_token,
                flag: pipe::ServerMessageFlag::Join,
                room_token,
                sender,
                physical_height: self.physical_height,
                physical_width: self.physical_width,
                window_height: self.window_height,
                window_width: self.window_width,
            })
            .unwrap();
        self.my_recv = Some(receiver);
        match self.my_recv.as_ref().unwrap().recv() {
            Ok(message) => {
                self.room_token = message.room_token;
                self.game_sender = message.sender;
                let packet = packet::Packet::new(
                    packet::Flag::Create,
                    0,
                    self.session_token,
                    self.room_token,
                    &[],
                    0,
                );
                self.send_packet(packet);
                Ok(Lock::Disabled)
            }
            Err(_) => Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken")),
        }
    }
}
