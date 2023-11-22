use super::packet;
use super::pipe;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

pub struct Player {
    pub sender: mpsc::Sender<pipe::GameMessage>,
    pub receiver: mpsc::Receiver<pipe::GameMessage>,

    pub rank: u8,
}

impl Player {
    /// Send data to the associated client
    pub fn send(&mut self, data: &[u8; packet::MAX_DATA_SIZE]) {
        self.sender.send(pipe::GameMessage::data_message(data.clone())).unwrap();
    }

    /// Receive data from the associated client
    /// Return how much data was actually received
    /// If no data was received, the function return 0
    pub fn recv(&mut self, buffer: &mut [u8; packet::MAX_DATA_SIZE]) -> bool {
        match self.receiver.try_recv() {
            Ok(m) => {
                buffer.copy_from_slice(&m.data.unwrap());
                true
            },
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => panic!("Client {} disconnected", self.rank),
        }
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