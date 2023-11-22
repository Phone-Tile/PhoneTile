use std::time::SystemTime;
use std::convert::Into;
use std::convert::TryFrom;

pub const HEADER_SIZE: usize = 8;
pub const MAX_DATA_SIZE: usize = 2040;
pub const BUFFER_SIZE: usize = HEADER_SIZE+MAX_DATA_SIZE;

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
    pub fn new(flag: u8, sync: u8, session: u16, room: u16, data: [u8; MAX_DATA_SIZE]) -> Packet {
        Packet {
            version: Version::V0,
            flag,
            sync,
            size: data.len(),
            session,
            room,
            data,
            processed_time: SystemTime::now()
        }
    }
    
    fn unpack_u16(data: &[u8]) -> u16 {
        ((data[0] as u16) << 8) + (data[1] as u16)
    }

    fn pack_u16(int: u16, slice: &mut [u8]) {
        slice[0] = u8::try_from(int>>8).unwrap();
        slice[1] = u8::try_from(int & (0x00ff_u16)).unwrap();
    }

    fn process_v0_packet(packet: &[u8; BUFFER_SIZE]) -> Result<Packet, &'static str> {
        // if size != packet[3] as usize + HEADER_SIZE {
        //     return Err("non-standard packet : packet size doesn't fit header info");
        // }
        let mut data = [0_u8; MAX_DATA_SIZE];
        data.clone_from_slice(&packet[8..MAX_DATA_SIZE+HEADER_SIZE]);

        Ok(Packet {
            version: Version::V0,
            flag: packet[1],
            sync: packet[2],
            size: packet[3] as usize,
            session: Packet::unpack_u16(&packet[4..6]),
            room: Packet::unpack_u16(&packet[6..8]),
            data,

            processed_time: SystemTime::now()
        })
    }

    /// Create a packet from raw data
    pub fn unpack(packet: &[u8; BUFFER_SIZE]) -> Result<Self, &'static str> {
        match packet[0].into() {
            Version::V0 => Self::process_v0_packet(packet),
            _ => Err("non-standard packet : not recognized version"),
        }
    }

    /// Pack in buffer the packet
    pub fn pack(&self, packet: &mut [u8; MAX_DATA_SIZE+HEADER_SIZE]) {
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
    pub fn log_packet(self) {
        println!("[ \033[32m INFO \033[97m ] Packet processed at {:?} :", self.processed_time);
        println!("\t Version : {:?}", self.version as u8);
        println!("\t Size : {:?}", self.size + HEADER_SIZE);
        println!("\t Session : {:?}", self.session);
        println!("\t Room : {:?}", self.room);
    }
}