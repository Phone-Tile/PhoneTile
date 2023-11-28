use super::packet;
use super::pipe;
use std::io::Error;
use std::io::ErrorKind;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

//////////////////////////////////////////////
///
///
/// Player structure
///
///
//////////////////////////////////////////////

pub struct Player {
    pub sender: mpsc::Sender<pipe::GameMessage>,
    pub receiver: mpsc::Receiver<pipe::GameMessage>,

    pub rank: u8,
    pub top_left_x: f32,
    pub top_left_y: f32,
    pub physical_height: f32,
    pub physical_width: f32,
    pub window_height: u32,
    pub window_width: u32,
}

impl Player {
    /// Send data to the associated client
    pub fn send(&mut self, raw_data: &[u8]) -> Result<(), Error> {
        let mut data = [0_u8; packet::MAX_DATA_SIZE];
        let size = raw_data.len();
        if (size > packet::MAX_DATA_SIZE) {
            panic!("Trying to send too much data");
        }
        if (size > 0) {
            data[..size].copy_from_slice(raw_data);
        }
        match self
            .sender
            .send(pipe::GameMessage::data_message(data, size))
        {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::NotConnected, "client not connected")),
        }
    }

    /// Receive data from the associated client
    /// Return how much data was actually received
    /// If no data was received, the function return 0
    pub fn recv(&mut self, buffer: &mut [u8; packet::MAX_DATA_SIZE]) -> Result<usize, Error> {
        match self.receiver.try_recv() {
            Ok(m) => {
                buffer.copy_from_slice(&m.data.unwrap());
                Ok(m.size)
            }
            Err(TryRecvError::Empty) => Ok(0),
            Err(TryRecvError::Disconnected) => {
                Err(Error::new(ErrorKind::NotConnected, "client not connected"))
            }
        }
    }

    /// Convert physical coordinates to screen coordinates
    pub fn to_local_coordinates(&self, x: f32, y: f32) -> (u32, u32) {
        let mut res_x = x;
        let mut res_y = y;
        res_x -= self.top_left_x;
        res_y -= self.top_left_y;
        res_x *= self.window_width as f32 / self.physical_width;
        res_y *= self.window_height as f32 / self.physical_height;

        (res_x.floor() as u32, res_y.floor() as u32)
    }

    /// Convert screen coordinates to physical coordinates, not usefull yet so big ratio
    pub fn from_local_coordinates(x: u32, y: u32) -> (f32, f32) {
        todo!()
    }
}
