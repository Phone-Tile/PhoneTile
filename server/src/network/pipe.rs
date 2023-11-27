use std::sync::mpsc;

use super::packet::{self, BUFFER_SIZE};

//////////////////////////////////////////////
///
///
/// Message destinated for the server
///
///
//////////////////////////////////////////////

pub enum ServerMessageFlag {
    Create,
    Join,
}

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

//////////////////////////////////////////////
///
///
/// Messages between clients and rooms
///
///
//////////////////////////////////////////////

pub enum GameMessageFlag {
    Init,
    Lock,
    Launch,
    Data,

    Disconnected,
    Error,
}

pub struct GameMessage {
    pub flag: GameMessageFlag,

    pub room_token: u16,
    pub sender: Option<mpsc::Sender<GameMessage>>,
    pub rank: Option<u16>,
    pub size: usize,
    pub data: Option<[u8; packet::MAX_DATA_SIZE]>,
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
