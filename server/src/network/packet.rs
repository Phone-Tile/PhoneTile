use std::time::SystemTime;
use std::convert::Into;

const BUFFER_SIZE: usize = 2048;
pub const HEADER_SIZE: usize = 8;
pub const MAX_DATA_SIZE: usize = 2040;

pub enum Flag {
    Error,
    Init,
    Create,
    Join,
    Lock,
    Launch,
    Unknown,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Version {
    V0 = 0x0,
    Unknown = 0xf,
}

impl From<u8> for Version {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => return Version::V0,
            _ => return Version::Unknown,
        };
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
            _ => Flag::Unknown,
        }
    }
}

impl Into<u8> for Version {
    fn into(self) -> u8 {
        match self {
            Version::V0 => return 0x00,
            Version::Unknown => return 0xff,
        };
    }
}

pub struct Packet {
    version: Version,
    flag: u8,
    sync: u8,
    size: usize,
    session: u16,
    room: u16,
    data: Vec<u8>,

    processed_time: SystemTime,
}

impl Packet {
    pub fn new(flag: u8, sync: u8, session: u16, room: u16, data: Vec<u8>) -> Packet {
        Packet {
            version: Version::V0,
            flag: flag,
            sync: sync,
            size: data.len(),
            session: session,
            room: room,
            data: data,
            processed_time: SystemTime::now()
        }
    }
    
    fn unpack_u16(data: &[u8]) -> u16 {
        (data[0] as u16) << 8 + (data[1] as u16)
    }

    fn pack_u16(int: u16, slice: &mut [u8]) {
        slice[0] = (int>>8) as u8;
        slice[1] = (int & (0x00ff as u16)) as u8;
    }

    fn process_v0_packet(packet: [u8; BUFFER_SIZE], size: usize) -> Result<Packet, &'static str> {
        // if size != packet[3] as usize + HEADER_SIZE {
        //     return Err("non-standard packet : packet size doesn't fit header info");
        // }

        Ok(Packet {
            version: Version::V0,
            flag: packet[1],
            sync: packet[2],
            size: packet[3] as usize,
            session: Packet::unpack_u16(&packet[4..6]),
            room: Packet::unpack_u16(&packet[6..8]),
            data: packet[8..size].to_vec(),

            processed_time: SystemTime::now()
        })
    }

    /// Create a packet from raw data
    pub fn unpack(packet: [u8; BUFFER_SIZE], size: usize) -> Result<Self, &'static str> {
        if size < HEADER_SIZE {
            return Err("non-standard packet : too small");
        }
        match packet[0].into() {
            Version::V0 => return Self::process_v0_packet(packet, size),
            _ => return Err("non-standard packet : not recognized version"),
        }
    }

    pub fn pack(&self, mut packet: [u8; HEADER_SIZE+MAX_DATA_SIZE]) {
        packet[0] = self.version.into();
        packet[1] = self.flag;
        packet[2] = self.sync;
        packet[3] = self.size as u8;
        Self::pack_u16(self.session, &mut packet[4..6]);
        Self::pack_u16(self.room, &mut packet[6..8]);
        packet[8..].clone_from_slice(self.data.as_slice());
    }

    /// Get packet flag
    pub fn get_flag(self) -> Flag {
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