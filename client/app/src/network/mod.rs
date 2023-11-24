use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::{thread, time};

pub mod packet;

/// All of those functions are completely non-blocking

#[derive(Clone)]
pub enum Status {
    Connected,
    Disconnected,
    InRoom,
    SelectedGame,
    InLockRoom(u8),
    InGame,
}

pub struct Network {
    stream: TcpStream,
    session_token: u16,
    room_token: u16,
    status: Status,
}

impl Network {
    fn init_handshake(&mut self) -> Result<(), Error> {
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE + packet::HEADER_SIZE];
        packet::Packet::new(packet::Flag::Init, 0, 0, 0, [0_u8; packet::MAX_DATA_SIZE])
            .send_packet(&mut self.stream)
            .unwrap();

        match packet::Packet::recv_packet(&mut self.stream) {
            Ok(packet) => {
                self.session_token = packet.session;
            }
            Err(_) => panic!("Not well formed packet"),
        }
        Ok(())
    }

    /// Connect to the server, you must do this action BEFORE ANYTHING ELSE
    pub fn connect(
        physical_height: f32,
        physical_width: f32,
        window_height: u32,
        window_width: u32,
    ) -> Self {
        match TcpStream::connect("127.0.0.1:8888") {
            Ok(stream) => {
                let mut network = Network {
                    stream,
                    session_token: 0,
                    room_token: 0,
                    status: Status::Connected,
                };
                network.init_handshake().unwrap();
                network
            }
            Err(_) => panic!("Unabled to connect to server !"),
        }
    }

    /// Send data to the server ; this action can only be done in game
    /// If you use this function outisde of a game, this will simply discard the message
    pub fn send(&mut self, data: &[u8; packet::MAX_DATA_SIZE]) {
        let mut buffer = [0_u8; packet::BUFFER_SIZE];
        packet::Packet::new(packet::Flag::Transmit, 0, self.session_token, 0, *data)
            .send_packet(&mut self.stream)
            .unwrap();
    }

    /// Receive data from the server ; this action can only be done in game
    /// It return the amount of data read
    pub fn recv(&mut self, buffer: &mut [u8; packet::MAX_DATA_SIZE]) -> bool {
        match packet::Packet::recv_packet(&mut self.stream) {
            Ok(packet) => {
                buffer.copy_from_slice(&packet.data);
                true
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => false,
            Err(e) => panic!("{e}"),
        }
    }

    /// Create a room and send back the ID of the room in order for the other
    /// to connect themselves to it
    pub fn create_room(&mut self) -> Result<u16, Error> {
        let packet_room_creation = packet::Packet::new(
            packet::Flag::Create,
            0,
            self.session_token,
            0,
            [0_u8; packet::MAX_DATA_SIZE],
        );
        packet_room_creation.send_packet(&mut self.stream)?;

        match packet::Packet::recv_packet(&mut self.stream) {
            Ok(packet) => {
                self.room_token = packet.room;
                self.status = Status::InRoom;
                Ok(packet.room)
            }
            Err(_) => panic!("Not well formed packet"),
        }
    }

    /// Join a room with the given room ID
    pub fn join_room(&mut self, room_token: u16) -> Result<(), Error> {
        packet::Packet::new(
            packet::Flag::Join,
            0,
            self.session_token,
            room_token,
            [0_u8; packet::MAX_DATA_SIZE],
        )
        .send_packet(&mut self.stream)?;

        match packet::Packet::recv_packet(&mut self.stream) {
            Ok(packet) => {
                self.room_token = packet.room;
                self.status = Status::InRoom;
            }
            Err(_) => panic!("Not well formed packet"),
        }

        Ok(())
    }

    /// Get the current status of the network
    pub fn get_status(&mut self) -> Status {
        match self.status {
            Status::SelectedGame => match packet::Packet::try_recv_packet(&mut self.stream) {
                Some(packet) => {
                    self.status = Status::InLockRoom(packet.data[0]);
                    self.status.clone()
                }
                None => self.status.clone(),
            },
            Status::InLockRoom(_) => {
                let mut buffer = [0_u8; packet::MAX_DATA_SIZE + packet::HEADER_SIZE];
                match self.stream.read_exact(&mut buffer) {
                    Ok(_) => {
                        self.status = Status::InGame;
                        self.status.clone()
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => self.status.clone(),
                    Err(e) => panic!("{e}"),
                }
            }
            _ => self.status.clone(),
        }
    }

    /// Select game, doesn't work yet ...
    pub fn game_select(&mut self) {
        self.status = Status::SelectedGame;
    }

    /// Lock the room, so that no more user can join the room
    /// The position of each user is given from this point when the get_status is triggered
    /// THIS FUNCTION WILL WORK ONLY IF create_room HAS BEEN CALLED BEFORE THAT
    pub fn lock_room(&mut self) -> Result<(), Error> {
        packet::Packet::new(
            packet::Flag::Lock,
            0,
            self.session_token,
            self.room_token,
            [0_u8; packet::MAX_DATA_SIZE],
        )
        .send_packet(&mut self.stream)
    }

    /// Launch the actual game
    /// THIS FUNCTION WILL WORK ONLY IF create_room HAS BEEN CALLED BEFORE THAT
    pub fn launch_game(&mut self) -> Result<(), Error> {
        match packet::Packet::new(
            packet::Flag::Launch,
            0,
            self.session_token,
            self.room_token,
            [0_u8; packet::MAX_DATA_SIZE],
        )
        .send_packet(&mut self.stream)
        {
            Ok(_) => {
                self.status = Status::InGame;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
