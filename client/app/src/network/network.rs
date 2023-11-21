enum Status {
    CONNECTED,
    DISCONNECTED,
    IN_ROOM,
    GAME_SELECT,
    IN_LOCK_GAME(u8),
    IN_GAME
}

pub struct id {
    id : &[u8]
}

// SERVER TO CLIENT

pub fn get_status() -> Status {
    Status::CONNECTED
}

/// while host in room, receive new players
pub fn get_waiting_list() {}

/// when game starts, ask for data
pub fn get_design_data() {}

/// while in game
pub fn receive() {}


// CLIENT TO SERVER

pub fn connect() {}

pub fn create_room() -> Vec<u8> {} // return ID

pub fn join_room() {} // takes id: Vec<u8>

pub fn game_select() {}

pub fn lock_game() {}

pub fn launch_game() {}

// while in game, send data
pub fn send(data: &[u8]) {}
