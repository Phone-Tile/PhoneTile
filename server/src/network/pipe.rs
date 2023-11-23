use std::sync::mpsc;

use super::packet;

pub enum ServerMessageFlag {
    Create,
    Join,
}

pub struct ServerMessage {
    pub session_tocken: u16,
    pub flag: ServerMessageFlag,
    pub room_tocken: u16,
    pub sender: mpsc::Sender<GameMessage>,
}

impl ServerMessage {
    pub fn new(
        usr_tocken: u16,
        flag: ServerMessageFlag,
        room_tocken: u16,
        sender: mpsc::Sender<GameMessage>,
    ) -> ServerMessage {
        ServerMessage {
            session_tocken: usr_tocken,
            flag,
            room_tocken,
            sender,
        }
    }
}

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

    pub room_tocken: u16,
    pub sender: Option<mpsc::Sender<GameMessage>>,
    pub rank: Option<u8>,
    pub data: Option<[u8; packet::MAX_DATA_SIZE]>,
}

impl GameMessage {
    pub fn init_message(sender: mpsc::Sender<GameMessage>, room_tocken: u16) -> Self {
        GameMessage {
            flag: GameMessageFlag::Init,
            room_tocken,
            sender: Some(sender),
            rank: None,
            data: None,
        }
    }

    pub fn lock_message(rank: u8) -> Self {
        GameMessage {
            flag: GameMessageFlag::Lock,
            room_tocken: 0,
            sender: None,
            rank: Some(rank),
            data: None,
        }
    }

    pub fn launch_message() -> Self {
        GameMessage {
            flag: GameMessageFlag::Launch,
            room_tocken: 0,
            sender: None,
            rank: None,
            data: None,
        }
    }

    pub fn data_message(data: [u8; packet::MAX_DATA_SIZE]) -> Self {
        GameMessage {
            flag: GameMessageFlag::Data,
            room_tocken: 0,
            sender: None,
            rank: None,
            data: Some(data),
        }
    }

    pub fn error_message() -> Self {
        GameMessage {
            flag: GameMessageFlag::Error,
            room_tocken: 0,
            sender: None,
            rank: None,
            data: None,
        }
    }
}
