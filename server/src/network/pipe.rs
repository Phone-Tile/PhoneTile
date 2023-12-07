use super::mock_mpsc::mpsc;

use super::packet::{self, BUFFER_SIZE, Flag};

#[cfg(test)]
use super::test;

//////////////////////////////////////////////
///
///
/// Message destinated for the server
///
///
//////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub enum ServerMessageFlag {
    Create,
    Join,
}

impl From<usize> for ServerMessageFlag {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Create,
            _ => Self::Join,
        }
    }
}

#[derive(Clone)]
pub struct ServerMessage {
    pub session_token: u16,
    pub room_token: u16,
    pub flag: ServerMessageFlag,
    pub sender: mpsc::Sender<GameMessage>,
    pub physical_height: f32,
    pub physical_width: f32,
    pub window_height: u32,
    pub window_width: u32,
}

impl PartialEq for ServerMessage {
    fn eq(&self, other: &Self) -> bool {
        self.session_token == other.session_token
            && self.room_token == other.room_token
            && self.flag == other.flag
            && self.physical_height == other.physical_height
            && self.physical_width == other.physical_width
            && self.window_height == other.window_height
            && self.window_width == other.window_width
    }
}

//////////////////////////////////////////////
///
///
/// Messages between clients and rooms
///
///
//////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum GameMessageFlag {
    Init,
    Lock,
    Launch,
    Data,

    Disconnected,
    Error,
}

impl From<usize> for GameMessageFlag {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Init,
            1 => Self::Lock,
            2 => Self::Launch,
            3 => Self::Data,
            4 => Self::Disconnected,
            _ => Self::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameMessage {
    pub flag: GameMessageFlag,

    pub room_token: u16,
    pub sender: Option<mpsc::Sender<GameMessage>>,
    pub rank: Option<u16>,
    pub size: usize,
    pub data: Option<[u8; packet::MAX_DATA_SIZE]>,
}

impl PartialEq for GameMessage {
    fn eq(&self, other: &Self) -> bool {
        self.flag == other.flag
            && self.room_token == other.room_token
            && self.rank == other.rank
            && self.size == other.size
            && self.data == other.data
    }
}

impl GameMessage {
    pub fn init_message(sender: mpsc::Sender<GameMessage>, room_token: u16) -> Self {
        GameMessage {
            flag: GameMessageFlag::Init,
            room_token,
            sender: Some(sender),
            rank: None,
            size: 0,
            data: None,
        }
    }

    pub fn lock_message(rank: u16) -> Self {
        GameMessage {
            flag: GameMessageFlag::Lock,
            room_token: 0,
            sender: None,
            rank: Some(rank),
            size: 0,
            data: None,
        }
    }

    pub fn launch_message() -> Self {
        GameMessage {
            flag: GameMessageFlag::Launch,
            room_token: 0,
            sender: None,
            rank: None,
            size: 0,
            data: None,
        }
    }

    pub fn data_message(data: [u8; packet::MAX_DATA_SIZE], size: usize) -> Self {
        GameMessage {
            flag: GameMessageFlag::Data,
            room_token: 0,
            sender: None,
            rank: None,
            size,
            data: Some(data),
        }
    }

    pub fn error_message() -> Self {
        GameMessage {
            flag: GameMessageFlag::Error,
            room_token: 0,
            sender: None,
            rank: None,
            size: 0,
            data: None,
        }
    }
}

//////////////////////////////////////////////
///
///
/// Tests
///
///
//////////////////////////////////////////////

#[cfg(test)]
#[repr(usize)]
#[derive(Clone)]
pub enum ServerMessageFuzz {
    SessionTocken,
    RoomTocken,
    Flag,
    PhysicalHeight,
    PhysicalWidth,
    WindowHeight,
    WindowWidth,
}

#[cfg(test)]
impl test::AutoGenFuzz<ServerMessage, ServerMessageFuzz> for ServerMessage {
    fn fuzz_a_packet(packet: ServerMessage, skip_fuzzing: &Vec<ServerMessageFuzz>) -> Vec<ServerMessage> {
        let mut res = vec![];

        let mut fuzzing = [true; 8];
        for f in skip_fuzzing.iter() {
            fuzzing[f.clone() as usize] = false;
        }

        if fuzzing[ServerMessageFuzz::SessionTocken as usize] {
            let mut tmp = packet.clone();
            tmp.session_token += 1;
            res.push(tmp);
        }

        if fuzzing[ServerMessageFuzz::RoomTocken as usize] {
            let mut tmp = packet.clone();
            tmp.room_token += 1;
            res.push(tmp);
        }

        if fuzzing[ServerMessageFuzz::Flag as usize] {
            let mut tmp = packet.clone();
            tmp.flag = ((tmp.flag as usize) + 1).into();
            res.push(tmp);
        }

        if fuzzing[ServerMessageFuzz::PhysicalHeight as usize] {
            let mut tmp = packet.clone();
            tmp.physical_height += 1.;
            res.push(tmp);
        }

        if fuzzing[ServerMessageFuzz::PhysicalWidth as usize] {
            let mut tmp = packet.clone();
            tmp.physical_width += 1.;
            res.push(tmp);
        }

        if fuzzing[ServerMessageFuzz::WindowHeight as usize] {
            let mut tmp = packet.clone();
            tmp.window_height += 1;
            res.push(tmp);
        }

        if fuzzing[ServerMessageFuzz::WindowWidth as usize] {
            let mut tmp = packet.clone();
            tmp.window_width += 1;
            res.push(tmp);
        }

        res
    }
}

#[cfg(test)]
#[repr(usize)]
#[derive(Clone)]
pub enum GameMessageFuzz {
    Flag,
    RoomToken,
    Rank,
    Size,
    Data,
}

#[cfg(test)]
impl test::AutoGenFuzz<GameMessage, GameMessageFuzz> for GameMessage {
    fn fuzz_a_packet(packet: GameMessage, skip_fuzzing: &Vec<GameMessageFuzz>) -> Vec<GameMessage> {
        let mut res = vec![];

        let mut fuzzing = [true; 8];
        for f in skip_fuzzing.iter() {
            fuzzing[f.clone() as usize] = false;
        }

        if fuzzing[GameMessageFuzz::Flag as usize] {
            let mut tmp = packet.clone();
            tmp.flag = ((tmp.flag as usize) + 1).into();
            res.push(tmp);
        }

        if fuzzing[GameMessageFuzz::RoomToken as usize] {
            let mut tmp = packet.clone();
            tmp.room_token += 1;
            res.push(tmp);
        }

        if fuzzing[GameMessageFuzz::Rank as usize] {
            match packet.rank {
                Some(i) => {
                    let mut tmp = packet.clone();
                    tmp.rank = Some(i+1);
                    res.push(tmp);
                    tmp = packet.clone();
                    tmp.rank = None;
                    res.push(tmp);
                },
                None => {
                    let mut tmp = packet.clone();
                    tmp.rank = Some(0);
                    res.push(tmp);
                },
            }
        }

        if fuzzing[GameMessageFuzz::Size as usize] {
            let mut tmp = packet.clone();
            tmp.size += 1;
            res.push(tmp);
        }

        if fuzzing[GameMessageFuzz::Data as usize] {
            match packet.data {
                Some(_) => {
                    let mut tmp = packet.clone();
                    tmp.data = None;
                    res.push(tmp);
                },
                None => {
                    let mut tmp = packet.clone();
                    tmp.data = Some([0_u8; packet::MAX_DATA_SIZE]);
                    res.push(tmp);
                },
            }
        }

        res
    }
}


