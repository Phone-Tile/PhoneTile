use std::convert::Into;
use std::hash::BuildHasher;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::{self, SystemTime};

use log::info;

pub const HEADER_SIZE: usize = 8;
pub const MAX_DATA_SIZE: usize = 2040;
pub const BUFFER_SIZE: usize = HEADER_SIZE + MAX_DATA_SIZE;

#[derive(Clone, Copy)]
pub enum Flag {
    Error,
    Init,
    Create,
    Join,
    Lock,
    Launch,
    Transmit,
    Unknown,
}

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

impl From<u8> for Flag {
    fn from(orig: u8) -> Self {
        match orig {
            0 => Flag::Error,
            1 => Flag::Init,
            2 => Flag::Create,
            3 => Flag::Join,
            4 => Flag::Lock,
            5 => Flag::Launch,
            6 => Flag::Transmit,
            _ => Flag::Unknown,
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

#[derive(Debug, Clone)]
pub struct Packet {
    version: Version,
    flag: u8,
    sync: u8,
    size: usize,
    pub session: u16,
    pub room: u16,
    pub data: [u8; MAX_DATA_SIZE],

    processed_time: SystemTime,
}

impl Packet {
    pub fn new(flag: Flag, sync: u8, session: u16, room: u16, data: [u8; MAX_DATA_SIZE]) -> Packet {
        Packet {
            version: Version::V0,
            flag: flag as u8,
            sync,
            size: data.len(),
            session,
            room,
            data,
            processed_time: SystemTime::now(),
        }
    }

    fn unpack_u16(data: &[u8]) -> u16 {
        ((data[0] as u16) << 8) + (data[1] as u16)
    }

    fn pack_u16(int: u16, slice: &mut [u8]) {
        slice[0] = u8::try_from(int >> 8).unwrap();
        slice[1] = u8::try_from(int & (0x00ff_u16)).unwrap();
    }

    fn process_v0_packet(packet: &[u8; BUFFER_SIZE]) -> Result<Self, Error> {
        // if size != packet[3] as usize + HEADER_SIZE {
        //     return Err("non-standard packet : packet size doesn't fit header info");
        // }
        let mut data = [0_u8; MAX_DATA_SIZE];
        data.clone_from_slice(&packet[8..BUFFER_SIZE]);

        Ok(Packet {
            version: Version::V0,
            flag: packet[1],
            sync: packet[2],
            size: packet[3] as usize,
            session: Packet::unpack_u16(&packet[4..6]),
            room: Packet::unpack_u16(&packet[6..8]),
            data,

            processed_time: SystemTime::now(),
        })
    }

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
        packet[1] = self.flag;
        packet[2] = self.sync;
        packet[3] = self.size as u8;
        Self::pack_u16(self.session, &mut packet[4..6]);
        Self::pack_u16(self.room, &mut packet[6..8]);
        packet[8..].clone_from_slice(self.data.as_slice());
    }

    /// Get packet flag
    pub fn get_flag(&self) -> Flag {
        self.flag.into()
    }

    /// Produce a log in stdout
    pub fn log_packet(&self) {
        info!(target: "Packet", "Procesed at {:?} :", self.processed_time);
        info!(target: "Packet", "\t Version : {}", self.version as u8);
        info!(target: "Packet", "\t Size : {:?}", self.size + HEADER_SIZE);
        info!(target: "Packet", "\t Session : {:?}", self.session);
        info!(target: "Packet", "\t Room : {:?}", self.room);
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

    pub fn error_message(session_token: u16) -> Packet {
        Self::new(Flag::Error, 0, session_token, 0, [0_u8; MAX_DATA_SIZE])
    }
}
