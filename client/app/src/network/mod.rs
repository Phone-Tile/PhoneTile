use std::net::TcpStream;
use std::io::{Write, Read, ErrorKind};
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
    session_tocken: u16,
    room_tocken: u16,
    status: Status,
}

impl Network {
    fn init_handshake(&mut self) -> Result<(), std::io::Error> {
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE+packet::HEADER_SIZE];
        packet::Packet::new(packet::Flag::Init as u8, 0, 0, 0, [0_u8; packet::MAX_DATA_SIZE]).pack(&mut buffer);
        self.stream.write_all(&buffer).unwrap();

        self.block_read_exact(&mut buffer);
        match packet::Packet::unpack(&buffer) {
            Ok(packet) => {
                self.session_tocken = packet.session;
            },
            Err(_) => panic!("Not well formed packet"),
        }
        Ok(())
    }
    
    /// Connect to the server, you must do this action BEFORE ANYTHING ELSE
    pub fn connect(
        physical_height: f32,
        physical_width: f32,
        window_height: u32,
        window_width: u32) -> Self {
        match TcpStream::connect("10.0.2.2:8888") {
            Ok(stream) => {
                stream.set_nonblocking(true);
                let mut network = Network {
                    stream,
                    session_tocken: 0,
                    room_tocken: 0,
                    status: Status::Connected,
                };
                network.init_handshake().unwrap();
                network
            },
            Err(_) => panic!("Unabled to connect to server !"),
        }
    }

    /// Send data to the server ; this action can only be done in game
    /// If you use this function outisde of a game, this will simply discard the message
    pub fn send(&mut self, data: &[u8; packet::MAX_DATA_SIZE]) {
        let mut buffer = [0_u8; packet::BUFFER_SIZE];
        packet::Packet::new(packet::Flag::Transmit as u8, 0, self.session_tocken, 0, data.clone()).pack(&mut buffer);
        self.stream.write_all(&buffer).unwrap();
    }
    
    /// Receive data from the server ; this action can only be done in game
    /// It return the amount of data read
    pub fn recv(&mut self, buffer: &mut [u8; packet::MAX_DATA_SIZE]) -> bool {
        let mut internal_buffer = [0_u8; packet::BUFFER_SIZE];
        match self.stream.read_exact(&mut internal_buffer) {
            Ok(_) => {
                buffer.copy_from_slice(&packet::Packet::unpack(&internal_buffer).unwrap().data);
                true
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => false,
            Err(e) => panic!("{e}"),
        }
    }

    /// Create a room and send back the ID of the room in order for the other
    /// to connect themselves to it
    pub fn create_room(&mut self) -> Result<u16, std::io::Error> {
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE+packet::HEADER_SIZE];
        let packet_room_creation = packet::Packet::new(packet::Flag::Create as u8, 0, self.session_tocken, 0, [0_u8; packet::MAX_DATA_SIZE]);
        packet_room_creation.pack(&mut buffer);

        self.stream.write_all(&buffer)?;
        self.block_read_exact(&mut buffer);
        match packet::Packet::unpack(&buffer) {
            Ok(packet) => {
                self.room_tocken = packet.room;
                self.status = Status::InRoom;
                Ok(packet.room)
            },
            Err(_) => panic!("Not well formed packet"),
        }
    }

    /// Join a room with the given room ID
    pub fn join_room(&mut self, room_tocken: u16) -> Result<(), std::io::Error> {
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE+packet::HEADER_SIZE];
        let packet_room_creation = packet::Packet::new(packet::Flag::Join as u8, 0, self.session_tocken, room_tocken, [0_u8; packet::MAX_DATA_SIZE]);
        packet_room_creation.pack(&mut buffer);

        self.stream.write_all(&buffer)?;
        self.block_read_exact(&mut buffer);
        match packet::Packet::unpack(&buffer) {
            Ok(packet) => {
                self.room_tocken = packet.room;
                self.status = Status::InRoom;
            },
            Err(_) => panic!("Not well formed packet"),
        }

        Ok(())
    }

    /// Get the current status of the network
    pub fn get_status(&mut self) -> Status {
        match self.status {
            Status::SelectedGame => {
                let mut buffer = [0_u8; packet::MAX_DATA_SIZE+packet::HEADER_SIZE];
                match self.stream.read_exact(&mut buffer) {
                    Ok(_) => {
                        let packet = packet::Packet::unpack(&buffer).unwrap();
                        self.status = Status::InLockRoom(packet.data[0]);
                        self.status.clone()
                    },
                    Err(e) if e.kind() == ErrorKind::WouldBlock => self.status.clone(),
                    Err(e) => panic!("{e}"),
                }
            },
            Status::InLockRoom(_) => {
                let mut buffer = [0_u8; packet::MAX_DATA_SIZE+packet::HEADER_SIZE];
                match self.stream.read_exact(&mut buffer) {
                    Ok(_) => {
                        self.status = Status::InGame;
                        self.status.clone()
                    },
                    Err(e) if e.kind() == ErrorKind::WouldBlock => self.status.clone(),
                    Err(e) => panic!("{e}"),
                }
            },
            _ => self.status.clone(),
        }
    }

    /// Lock the room, so that no more user can join the room
    /// The position of each user is given from this point when the get_status is triggered
    /// THIS FUNCTION WILL WORK ONLY IF create_room HAS BEEN CALLED BEFORE THAT
    pub fn lock_room(&mut self) -> Result<(), std::io::Error> {
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE+packet::HEADER_SIZE];
        packet::Packet::new(0, 0, self.session_tocken, self.room_tocken, [0_u8; packet::MAX_DATA_SIZE]).pack(&mut buffer);

        self.stream.write_all(&buffer)
    }

    /// Launch the actual game
    /// THIS FUNCTION WILL WORK ONLY IF create_room HAS BEEN CALLED BEFORE THAT
    pub fn launch_game(&mut self) -> Result<(), std::io::Error> {
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE+packet::HEADER_SIZE];
        packet::Packet::new(packet::Flag::Launch as u8, 0, self.session_tocken, self.room_tocken, [0_u8; packet::MAX_DATA_SIZE]).pack(&mut buffer);

        self.status = Status::InGame;

        self.stream.write_all(&buffer)
    }

    pub fn game_select(&mut self) {
        self.status = Status::SelectedGame;
    }

    fn block_read_exact(&mut self, buf: &mut [u8]) {
        loop {
            match self.stream.read_exact(buf) {
                Ok(_) => return,
                Err(e) if e.kind() == ErrorKind::WouldBlock => {},
                Err(e) => panic!("{e}"),
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}