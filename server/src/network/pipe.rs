use std::sync::mpsc;

pub enum ServerMessageFlag {
    CREATE,
    JOIN
}

pub struct ServerMessage {
    pub usr_tocken: u16,
    pub flag: ServerMessageFlag,
    pub game_tocken: u16,
    pub sender: mpsc::Sender<GameMessage>,
}

impl ServerMessage {
    pub fn new(usr_tocken: u16, flag: ServerMessageFlag, game_tocken: u16, sender: mpsc::Sender<GameMessage>) -> ServerMessage {
        ServerMessage {
            usr_tocken: usr_tocken,
            flag: flag,
            game_tocken: game_tocken,
            sender: sender
        }
    }
}

pub enum GameMessageFlag {
    INIT,
    LOCK,
    LAUNCH,

    DISCONNECTED,
}

pub struct GameMessage {
    flag: GameMessageFlag,

    sender: Option<mpsc::Sender<GameMessage>>,
    rank: Option<u8>,
}

impl GameMessage {
    pub fn init_message(sender: mpsc::Sender<GameMessage>) -> Self {
        GameMessage {
            flag: GameMessageFlag::INIT,
            sender: Some(sender),
            rank: None,
        }
    }

    pub fn lock_message(rank: u8) -> Self {
        GameMessage {
            flag: GameMessageFlag::LOCK,
            sender: None,
            rank: Some(rank),
        }
    }

    pub fn launch_message() -> Self {
        GameMessage {
            flag: GameMessageFlag::LAUNCH,
            sender: None,
            rank: None
        }
    }
}