use std::vec;

use super::connection;
use super::packet;

struct Player {
    connection: connection::Connection,
}

impl Player {
    pub fn send(data: &[u8; packet::MAX_DATA_SIZE]) {
        todo!();
    }

    pub fn recv(buffer: [u8; packet::MAX_DATA_SIZE]) {
        todo!();
    }

    pub fn to_local_coordinates(x: f32, y: f32) -> (f32, f32) {
        todo!();
    }

    pub fn from_local_coordinates(x: f32, y: f32) -> (f32, f32) {
        todo!();
    }
}