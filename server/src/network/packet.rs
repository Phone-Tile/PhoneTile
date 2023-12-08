use std::convert::Into;
use std::convert::TryFrom;
use std::fmt::Display;
use std::hash::BuildHasher;
use std::io::{Error, ErrorKind, Read, Write};

use super::mock_net::TcpStream;

use std::thread;
use std::time::{self, SystemTime};

use log::info;

//////////////////////////////////////////////
///
///
/// Constants
///
///
//////////////////////////////////////////////

pub const HEADER_SIZE: usize = 12;
pub const MAX_DATA_SIZE: usize = 2036;
pub const BUFFER_SIZE: usize = HEADER_SIZE + MAX_DATA_SIZE;

//////////////////////////////////////////////
///
///
/// ProtocolError
///
///
//////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProtocolError {
    ServerDown,
    GameCrashed,
    RoomClosed,
    InvalidPacket,
    InvalidRequest,
    Unknown,
}

impl From<u8> for ProtocolError {
    fn from(mut orig: u8) -> Self {
        orig -= 0x80_u8;
        match orig {
            1 => Self::ServerDown,
            2 => Self::GameCrashed,
            3 => Self::RoomClosed,
            4 => Self::InvalidPacket,
            5 => Self::InvalidRequest,
            _ => Self::Unknown,
        }
    }
}

impl From<ProtocolError> for u8 {
    fn from(orig: ProtocolError) -> Self {
        match orig {
            ProtocolError::ServerDown => 1 | 0x80_u8,
            ProtocolError::GameCrashed => 2 | 0x80_u8,
            ProtocolError::RoomClosed => 3 | 0x80_u8,
            ProtocolError::InvalidPacket => 4 | 0x80_u8,
            ProtocolError::InvalidRequest => 5 | 0x80_u8,
            ProtocolError::Unknown => 0xff_u8,
        }
    }
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::ServerDown => write!(f, "Serve Down"),
            ProtocolError::GameCrashed => write!(f, "Game Crashed"),
            ProtocolError::RoomClosed => write!(f, "Room Closed"),
            ProtocolError::InvalidPacket => write!(f, "Invalid Packet"),
            ProtocolError::InvalidRequest => write!(f, "Invalid Request"),
            ProtocolError::Unknown => write!(f, "Unknown"),
        }
    }
}

//////////////////////////////////////////////
///
///
/// Flag
///
///
//////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flag {
    Error(ProtocolError),
    Init,
    CreateRoom,
    JoinRoom,
    Lock,
    Launch,
    Transmit,
    Unknown,
}

impl From<u8> for Flag {
    fn from(orig: u8) -> Self {
        if orig & 0x80_u8 == 0x80_u8 {
            Flag::Error(orig.into())
        } else {
            match orig {
                1 => Flag::Init,
                2 => Flag::CreateRoom,
                3 => Flag::JoinRoom,
                4 => Flag::Lock,
                5 => Flag::Launch,
                6 => Flag::Transmit,
                _ => Flag::Unknown,
            }
        }
    }
}

impl From<Flag> for u8 {
    fn from(orig: Flag) -> Self {
        match orig {
            Flag::Error(e) => e.into(),
            Flag::Init => 1,
            Flag::CreateRoom => 2,
            Flag::JoinRoom => 3,
            Flag::Lock => 4,
            Flag::Launch => 5,
            Flag::Transmit => 6,
            Flag::Unknown => 0xff_u8,
        }
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flag::Init => write!(f, "Init"),
            Flag::CreateRoom => write!(f, "Create room"),
            Flag::JoinRoom => write!(f, "Join room"),
            Flag::Lock => write!(f, "Lock"),
            Flag::Launch => write!(f, "Launch"),
            Flag::Transmit => write!(f, "Transmit"),
            Flag::Error(e) => write!(f, "Error : {}", e),
            Flag::Unknown => write!(f, "Unknown"),
        }
    }
}

//////////////////////////////////////////////
///
///
/// Version
///
///
//////////////////////////////////////////////

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Version {
    V0 = 0x0,
    Unknown = 0xf,
}

impl From<u8> for Version {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => Version::V0,
            _ => Version::Unknown,
        }
    }
}

impl From<Version> for u8 {
    fn from(val: Version) -> Self {
        match val {
            Version::V0 => 0x00,
            Version::Unknown => 0xff,
        }
    }
}

//////////////////////////////////////////////
///
///
/// Packet
///
///
//////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Packet {
    version: Version,
    flag: Flag,
    sync: u8,
    pub size: usize,
    pub option: u16,
    pub session: u16,
    pub room: u16,
    pub data: [u8; MAX_DATA_SIZE],

    processed_time: SystemTime,
}

impl Packet {
    //////////////////////////////////////////////
    ///
    ///
    /// Send/Recv packet
    ///
    ///
    //////////////////////////////////////////////

    pub fn new(
        flag: Flag,
        sync: u8,
        session: u16,
        room: u16,
        raw_data: &[u8],
        option: u16,
    ) -> Packet {
        let mut data = [0_u8; MAX_DATA_SIZE];
        let size = raw_data.len();
        if (size > MAX_DATA_SIZE) {
            panic!("Trying to build a packet too large !");
        }
        if (size > 0) {
            data[..size].copy_from_slice(raw_data);
        }
        Packet {
            version: Version::V0,
            flag,
            sync,
            size,
            option,
            session,
            room,
            data,
            processed_time: SystemTime::now(),
        }
    }

    /// Receive and unpack a packet, it will be blocking until it receives a packet or the pipe is procken
    pub fn recv_packet(stream: &mut TcpStream) -> Result<Self, Error> {
        let mut buffer = [0_u8; BUFFER_SIZE];

        Packet::block_read_exact(stream, &mut buffer)?;
        Packet::unpack(&buffer)
    }

    /// Receive and unpack a packet without blocking
    pub fn try_recv_packet(stream: &mut TcpStream) -> Option<Self> {
        let mut buffer = [0_u8; BUFFER_SIZE];

        match stream.read_exact(&mut buffer) {
            Ok(_) => match Packet::unpack(&buffer) {
                Ok(packet) => Some(packet),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    /// Send the packet
    pub fn send_packet(&self, stream: &mut TcpStream) -> Result<(), Error> {
        let mut buffer = [0_u8; BUFFER_SIZE];
        self.pack(&mut buffer);
        stream.write_all(&buffer)
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Sanity check
    ///
    ///
    //////////////////////////////////////////////

    pub fn check_packet_flag(&self, flag: Flag) -> Result<(), Error> {
        if self.flag == flag {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "received the flag {} while expecting a {} packet",
                    self.flag, flag,
                ),
            ))
        }
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Prebuilt packet
    ///
    ///
    //////////////////////////////////////////////

    pub fn error_message(session_token: u16) -> Packet {
        Self::new(
            Flag::Error(ProtocolError::Unknown),
            0,
            session_token,
            0,
            &[0_u8; MAX_DATA_SIZE],
            0,
        )
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Info functions
    ///
    ///
    //////////////////////////////////////////////

    /// Get packet flag
    pub fn get_flag(&self) -> Flag {
        self.flag
    }

    /// Produce a log in stdout
    pub fn log_packet(&self) {
        info!(target: "Packet", "Procesed at {:?} :", self.processed_time);
        info!(target: "Packet", "\t Version : {}", self.version as u8);
        info!(target: "Packet", "\t Size : {:?}", self.size + HEADER_SIZE);
        info!(target: "Packet", "\t Session : {:?}", self.session);
        info!(target: "Packet", "\t Room : {:?}", self.room);
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Packet processing
    ///
    ///
    //////////////////////////////////////////////

    /// Create a packet from raw data
    fn unpack(packet: &[u8; BUFFER_SIZE]) -> Result<Self, Error> {
        match packet[0].into() {
            Version::V0 => Self::process_v0_packet(packet),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "non-standard packet : not recognized version",
            )),
        }
    }

    /// Pack in buffer the packet
    fn pack(&self, packet: &mut [u8; BUFFER_SIZE]) {
        packet[0] = self.version.into();
        packet[1] = self.flag.into();
        packet[2] = self.sync;
        Self::pack_u16(self.size as u16, &mut packet[4..6]);
        Self::pack_u16(self.option, &mut packet[6..8]);
        Self::pack_u16(self.session, &mut packet[8..10]);
        Self::pack_u16(self.room, &mut packet[10..12]);
        packet[HEADER_SIZE..].clone_from_slice(self.data.as_slice());
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Helpers
    ///
    ///
    //////////////////////////////////////////////

    fn process_v0_packet(packet: &[u8; BUFFER_SIZE]) -> Result<Self, Error> {
        let mut data = [0_u8; MAX_DATA_SIZE];
        let size = Packet::unpack_u16(&packet[4..6]) as usize;

        if size > MAX_DATA_SIZE {
            panic!("Not well formed packet")
        }

        if size > 0 {
            data[..size].clone_from_slice(&packet[HEADER_SIZE..HEADER_SIZE + size]);
        }

        Ok(Packet {
            version: Version::V0,
            flag: packet[1].into(),
            sync: packet[2],
            size,
            option: Packet::unpack_u16(&packet[6..8]),
            session: Packet::unpack_u16(&packet[8..10]),
            room: Packet::unpack_u16(&packet[10..12]),
            data,

            processed_time: SystemTime::now(),
        })
    }

    fn unpack_u16(data: &[u8]) -> u16 {
        ((data[0] as u16) << 8) + (data[1] as u16)
    }

    fn pack_u16(int: u16, slice: &mut [u8]) {
        slice[0] = u8::try_from(int >> 8).unwrap();
        slice[1] = u8::try_from(int & (0x00ff_u16)).unwrap();
    }

    fn block_read_exact(stream: &mut TcpStream, buf: &mut [u8]) -> Result<(), Error> {
        loop {
            match stream.read_exact(buf) {
                Ok(_) => return Ok(()),
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
            thread::sleep(time::Duration::from_millis(30));
        }
    }

    #[cfg(test)]
    pub fn pub_pack(&self) -> [u8; BUFFER_SIZE] {
        let mut buff = [0_u8; BUFFER_SIZE];
        self.pack(&mut buff);
        buff
    }

    #[cfg(test)]
    pub fn pub_unpack(packet: &[u8; BUFFER_SIZE]) -> Result<Packet, Error> {
        Self::unpack(packet)
    }
}

#[cfg(test)]
use crate::network::test::AutoGenFuzz;

#[cfg(test)]
#[repr(usize)]
#[derive(Clone, Copy)]
pub enum PacketFuzz {
    Version,
    Flag,
    Sync,
    Size,
    Data,
    Option,
    Session,
    Room,
}

#[cfg(test)]
impl AutoGenFuzz<Packet, PacketFuzz> for Packet {
    fn fuzz_a_packet(packet: Packet, skip_fuzz: &Vec<PacketFuzz>) -> Vec<Packet> {
        let mut res = vec![];

        let mut fuzzing = [true; 8];
        for f in skip_fuzz.iter() {
            fuzzing[f.clone() as usize] = false;
        }

        if fuzzing[PacketFuzz::Version as usize] {
            let mut tmp = packet.clone();
            tmp.version = (u8::from(tmp.version) + 1).into();
            res.push(tmp);
        }

        // fuzz flag
        if fuzzing[PacketFuzz::Flag as usize] {
            let mut tmp = packet.clone();
            tmp.flag = (u8::from(tmp.flag) + 1).into();
            res.push(tmp);
        }

        // fuzz sync
        if fuzzing[PacketFuzz::Sync as usize] {
            let mut tmp = packet.clone();
            tmp.sync += 1;
            res.push(tmp);
        }

        // fuzz option
        if fuzzing[PacketFuzz::Option as usize] {
            let mut tmp = packet.clone();
            tmp.option += 1;
            res.push(tmp);
        }

        // fuzz session
        if fuzzing[PacketFuzz::Session as usize] {
            let mut tmp = packet.clone();
            tmp.session += 1;
            res.push(tmp);
        }

        // fuzz room
        if fuzzing[PacketFuzz::Room as usize] {
            let mut tmp = packet.clone();
            tmp.room += 1;
            res.push(tmp);
        }

        res
    }
}
