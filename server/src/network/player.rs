use super::connection;
use super::packet;

struct Player {
    connection: connection::Connection,
}

impl Player {
    /// Send data to the associated client
    pub fn send(data: &[u8; packet::MAX_DATA_SIZE]) {
        todo!()
    }

    /// Receive data from the associated client
    /// Return how much data was actually received
    /// If no data was received, the function return 0
    pub fn recv(buffer: &mut [u8; packet::MAX_DATA_SIZE]) -> usize {
        todo!()
    }

    /// Convert physical coordinates to screen coordinates
    pub fn to_local_coordinates(x: f32, y: f32) -> (f32, f32) {
        todo!()
    }

    /// Convert screen coordinates to physical coordinates
    pub fn from_local_coordinates(x: f32, y: f32) -> (f32, f32) {
        todo!()
    }
}