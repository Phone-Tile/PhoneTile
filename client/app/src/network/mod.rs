
/// All of those functions are completely non-blocking

const MAX_DATA_SIZE: usize = 2040;


enum Status {
    Connected,
    Disconnected,
    InRoom,
    InLockRoom(u8),
    InGame
}

struct Network {

}

impl Network {
    /// Create a network handler
    /// You need to give the physical dimentions as well as the size of the window in pixels
    /// This is used for map generation and server side convertions
    pub fn new(
        physical_height: f32,
        physical_width: f32,
        window_height: u32,
        window_width: u32)
        -> Network {
        todo!()
    }

    /// Connect to the server, you must do this action BEFORE ANYTHING ELSE
    pub fn connect(self) {
        todo!()
    }

    /// Send data to the server ; this action can only be done in game
    /// If you use this function outisde of a game, this will simply discard the message
    pub fn send(data: &[u8; MAX_DATA_SIZE]) {
        todo!()
    }

    /// Receive data from the server ; this action can only be done in game
    /// It return the amount of data read
    pub fn recv(buffer: &mut [u8; MAX_DATA_SIZE]) -> usize {
        todo!()
    }

    /// Create a room and send back the ID of the room in order for the other
    /// to connect themselves to it
    pub fn create_room(self) -> Result<u16, str> {
        todo!()
    }

    /// Join a room with the given room ID
    pub fn join_room(self, room_id: u16) -> Result<(), str> {
        todo!()
    }

    /// Get the current status of the network
    pub fn get_status(self) -> Status {
        todo!()
    }

    /// Lock the room, so that no more user can join the room
    /// The position of each user is given from this point when the get_status is triggered
    /// THIS FUNCTION WILL WORK ONLY IF create_room HAS BEEN CALLED BEFORE THAT
    pub fn lock_room(self) {
        todo!()
    }


    /// Launch the actual game
    /// THIS FUNCTION WILL WORK ONLY IF create_room HAS BEEN CALLED BEFORE THAT
    pub fn launch_game(self) {
        todo!()
    }
}