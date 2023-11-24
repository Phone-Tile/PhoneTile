use std::sync::mpsc;

use super::packet;

pub enum ServerMessageFlag {
    Create,
    Join,
}

pub struct ServerMessage {
    pub session_token: u16,
    pub flag: ServerMessageFlag,
    pub room_token: u16,
    pub sender: mpsc::Sender<GameMessage>,
}

impl ServerMessage {
    pub fn new(
        usr_token: u16,
        flag: ServerMessageFlag,
        room_token: u16,
        sender: mpsc::Sender<GameMessage>,
    ) -> ServerMessage {
        ServerMessage {
            session_token: usr_token,
            flag,
            room_token,
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

    pub room_token: u16,
    pub sender: Option<mpsc::Sender<GameMessage>>,
    pub rank: Option<u8>,
    pub data: Option<[u8; packet::MAX_DATA_SIZE]>,
}

impl GameMessage {
    pub fn init_message(sender: mpsc::Sender<GameMessage>, room_token: u16) -> Self {
        GameMessage {
            flag: GameMessageFlag::Init,
            room_token,
            sender: Some(sender),
            rank: None,
            data: None,
        }
    }

    pub fn lock_message(rank: u8) -> Self {
        GameMessage {
            flag: GameMessageFlag::Lock,
            room_token: 0,
            sender: None,
            rank: Some(rank),
            data: None,
        }
    }

    pub fn launch_message() -> Self {
        GameMessage {
            flag: GameMessageFlag::Launch,
            room_token: 0,
            sender: None,
            rank: None,
            data: None,
        }
    }

    pub fn data_message(data: [u8; packet::MAX_DATA_SIZE]) -> Self {
        GameMessage {
            flag: GameMessageFlag::Data,
            room_token: 0,
            sender: None,
            rank: None,
            data: Some(data),
        }
    }

    pub fn error_message() -> Self {
        GameMessage {
            flag: GameMessageFlag::Error,
            room_token: 0,
            sender: None,
            rank: None,
            data: None,
        }
    }
}
