use super::mock_mpsc::mpsc;

use super::packet::{self, BUFFER_SIZE};

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
pub enum Fuzz {
    SessionTocken,
    RoomTocken,
    Flag,
    Sender,
    PhysicalHeight,
    PhysicalWidth,
    WindowHeight,
    WindowWidth,
}

#[cfg(test)]
impl test::AutoGenFuzz<ServerMessage, Fuzz> for ServerMessage {
    fn fuzz_a_packet(packet: ServerMessage, skip_fuzzing: &Vec<Fuzz>) -> Vec<ServerMessage> {
        let mut res = vec![];

        let mut fuzzing = [true; 8];
        for f in skip_fuzzing.iter() {
            fuzzing[f.clone() as usize] = false;
        }

        if fuzzing[Fuzz::SessionTocken as usize] {
            let mut tmp = packet.clone();
            tmp.session_token += 1;
            res.push(tmp);
        }

        if fuzzing[Fuzz::RoomTocken as usize] {
            let mut tmp = packet.clone();
            tmp.room_token += 1;
            res.push(tmp);
        }

        if fuzzing[Fuzz::Sender as usize] {
            let mut tmp = packet.clone();
            let (sender, _) = mpsc::channel();
            tmp.sender = sender;
            res.push(tmp);
        }

        if fuzzing[Fuzz::PhysicalHeight as usize] {
            let mut tmp = packet.clone();
            tmp.physical_height += 1.;
            res.push(tmp);
        }

        if fuzzing[Fuzz::PhysicalWidth as usize] {
            let mut tmp = packet.clone();
            tmp.physical_width += 1.;
            res.push(tmp);
        }

        if fuzzing[Fuzz::WindowHeight as usize] {
            let mut tmp = packet.clone();
            tmp.window_height += 1;
            res.push(tmp);
        }

        if fuzzing[Fuzz::WindowWidth as usize] {
            let mut tmp = packet.clone();
            tmp.window_width += 1;
            res.push(tmp);
        }

        res
    }
}
