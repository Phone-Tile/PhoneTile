use super::client;
use super::packet::{HEADER_SIZE, MAX_DATA_SIZE};
use super::pipe::{self, GameMessage};

use super::mock_net::TcpStream;
use log::{error, info, warn};
use std::io::{Error, ErrorKind, Read, Write};

use crate::network::packet;

use super::mock_mpsc::mpsc::{self, TryRecvError};
use std::time;
use std::{error, thread};

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

#[derive(Debug, PartialEq)]
enum Status {
    Created,
    Initialized,

    Error,
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

    game_sender: Option<mpsc::Sender<GameMessage>>,
    game_recv: mpsc::Receiver<GameMessage>,

    my_sender: mpsc::Sender<GameMessage>,
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

        let (my_sender, my_recv) = mpsc::channel();

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
            game_recv: my_recv,
            my_sender,
        }
    }

    pub fn manager(&mut self) -> Result<(), Error> {
        match self.handshake() {
            Ok(_) => {}
            Err(e) => {
                error!(target: self.target.as_str(), "unabled to initiate handshake : {e}");
                return Err(e);
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

                let reply =
                    packet::Packet::new(packet::Flag::Init, 0, self.session_token, 0, &[], 0);
                self.send_packet(reply);
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
                if packet.session != self.session_token {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "not a valid session-ID, abord connection",
                    ));
                }
                match flag {
                    packet::Flag::CreateRoom => self.join_room_with_create(),
                    packet::Flag::JoinRoom => self.join_room_with_join(room_token),
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
                let sender = self.game_sender.as_mut().unwrap();
                match sender.send(pipe::GameMessage::lock_message(self.game_id.into())) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken"))
                    }
                }
                match self.game_recv.recv() {
                    Ok(m) => Ok(m.rank.unwrap() as u8), // should never be None
                    Err(e) => Err(Error::new(ErrorKind::BrokenPipe, "pipe with game broken")),
                }
            }
            Lock::Disabled => {
                // listen to game_receiver for lock message
                match self.game_recv.recv() {
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
                match self.game_recv.recv() {
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
                match self.game_recv.recv() {
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
                //packet.check_packet_flag(packet::Flag::Transmit)?;
                match &mut self.game_sender {
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
            let error = &self.game_recv.try_recv(); // is will
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
        self.main_sender
            .send(pipe::ServerMessage {
                session_token: self.session_token,
                flag: pipe::ServerMessageFlag::Create,
                room_token: 0,
                sender: self.my_sender.clone(),
                physical_height: self.physical_height,
                physical_width: self.physical_width,
                window_height: self.window_height,
                window_width: self.window_width,
            })
            .unwrap(); // should never happened
        match self.game_recv.recv() {
            Ok(message) => {
                self.room_token = message.room_token;
                self.game_sender = message.sender;
                let packet = packet::Packet::new(
                    packet::Flag::CreateRoom,
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
        self.main_sender
            .send(pipe::ServerMessage {
                session_token: self.session_token,
                flag: pipe::ServerMessageFlag::Join,
                room_token,
                sender: self.my_sender.clone(),
                physical_height: self.physical_height,
                physical_width: self.physical_width,
                window_height: self.window_height,
                window_width: self.window_width,
            })
            .unwrap();
        match self.game_recv.recv() {
            Ok(message) => {
                self.room_token = message.room_token;
                self.game_sender = message.sender;
                let packet = packet::Packet::new(
                    packet::Flag::CreateRoom,
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

#[cfg(test)]
mod tests {
    use crate::network::{mock_mpsc::mpsc, pipe, test::AutoGenFuzz};
    use std::vec;

    use super::{client, packet, Connection};

    use crate::network::mock_net::{TcpListener, TcpStream};

    fn generate_boggus_versions(
        packets: &Vec<[u8; packet::BUFFER_SIZE]>,
    ) -> Vec<[u8; packet::BUFFER_SIZE]> {
        let mut res = vec![];
        for p in packets.iter() {
            let mut tmp = p.clone();
            tmp[0] = packet::Version::Unknown.into();
            res.push(tmp);
        }
        res
    }

    fn generate_boggus_flags(
        packets: &Vec<[u8; packet::BUFFER_SIZE]>,
    ) -> Vec<[u8; packet::BUFFER_SIZE]> {
        let mut res = vec![];
        for p in packets.iter() {}

        res
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Unit tests
    ///
    ///
    //////////////////////////////////////////////

    #[test]
    fn unit_handshake() {
        // normal behaviour
        for i in 0..5 {
            let normal_input = vec![packet::Packet::new(packet::Flag::Init, 0, 0, 0, &[], 0)];
            let normal_output = vec![packet::Packet::new(
                packet::Flag::Init,
                0,
                i as u16,
                0,
                &[],
                0,
            )];

            let stream = TcpStream::new(normal_input.clone(), normal_output.clone());

            let (sender, receiver) = mpsc::channel();

            let mut connection = Connection::new(stream, i, sender);

            connection.handshake().unwrap();
            assert_eq!(connection.status, super::Status::Initialized);

            let fuzzing = packet::Packet::generate_fuzzing(
                &normal_input,
                &vec![vec![
                    packet::Fuzz::Session,
                    packet::Fuzz::Room,
                    packet::Fuzz::Sync,
                    packet::Fuzz::Option,
                    packet::Fuzz::Size,
                ]],
            );

            for exec in fuzzing {
                let stream = TcpStream::new(exec.clone(), normal_output.clone());

                let (sender, receiver) = mpsc::channel();

                let mut connection = Connection::new(stream, i, sender);

                connection.handshake().unwrap_err();
            }
        }
    }

    #[test]
    fn unit_join_room_with_create() {
        // normal behaviour
        for session_token in 0..5 {
            for room_token in 0..5 {
                let normal_input = vec![];
                let normal_output = vec![packet::Packet::new(
                    packet::Flag::CreateRoom,
                    0,
                    session_token,
                    room_token,
                    &[],
                    0,
                )];

                let stream = TcpStream::new(normal_input, normal_output);

                let (game_sender, _) = mpsc::channel_with_checks(vec![], vec![]);
                let local_inputs = vec![pipe::GameMessage::init_message(
                    game_sender.clone(),
                    room_token,
                )];

                let (local_sender, local_recv) =
                    mpsc::channel_with_checks(vec![], local_inputs.clone());
                let main_outputs = vec![pipe::ServerMessage {
                    session_token,
                    flag: pipe::ServerMessageFlag::Create,
                    room_token: 0,
                    sender: local_sender.clone(),
                    physical_height: 1.,
                    physical_width: 2.,
                    window_height: 3,
                    window_width: 4,
                }];

                let (main_sender, _) = mpsc::channel_with_checks(main_outputs, vec![]);

                let mut connection = Connection {
                    status: super::Status::Created,
                    target: "".into(),
                    session_token,
                    room_token: 0,
                    game_id: client::Game::Unknown,
                    stream,
                    physical_height: 1.,
                    physical_width: 2.,
                    window_height: 3,
                    window_width: 4,
                    main_sender,
                    game_sender: None,
                    game_recv: local_recv,
                    my_sender: local_sender,
                };

                connection.join_room_with_create();

                assert_eq!(connection.room_token, room_token);
                assert_eq!(connection.game_sender, Some(game_sender));
            }
        }
    }

    #[test]
    fn unit_join_room_with_join() {}

    #[test]
    fn unit_join_room() {}
}
