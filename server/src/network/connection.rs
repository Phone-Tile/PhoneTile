use crate::network::packet;
use super::pipe;

use std::net::TcpStream;
use std::time::Duration;
use std::io::{Write, Read};
use std::sync::mpsc;

enum Status {
    Created,
    Initialized,
}

pub struct Connection {
    status: Status,

    session_tocken: u16,
    stream: TcpStream,
    // Sender for the main thread (game creation / join request)
    main_sender: mpsc::Sender<pipe::ServerMessage>,

    // Sender for the game thread (game oriented communication)
    game_sender: Option<mpsc::Sender<()>>,

    // Sender/Receiver pair for the game thread to send us data
    my_recv: mpsc::Receiver<()>,
    my_sender: mpsc::Sender<()>,
}

impl Connection {
    pub fn new(stream: TcpStream, tocken: u16, main_sender: mpsc::Sender<pipe::ServerMessage>) -> Self {
        let _ = stream.set_read_timeout(Some(Duration::new(5,0)));
        let (sender, receiver) = mpsc::channel();

        Connection {
            status: Status::Created,
            session_tocken: tocken,
            stream: stream,
            main_sender: main_sender,
            game_sender: None,
            my_recv: receiver,
            my_sender: sender
        }
    }

    pub fn manager(&mut self) -> Result<(), &'static str> {
        let mut buffer = [0 as u8; 2048];

       let _ = self.init_handshake()?;
        
        println!("[\033[32m INFO \033[97m] {:?} : Handshake done !", self.session_tocken);

        loop {
            // two possible things : either we create a game, either we connect to one !
            match self.stream.read(&mut buffer) {
                Ok(size) => {
                    match packet::Packet::unpack(buffer, size) {
                        Ok(packet) => {
                            match packet.get_flag() {
                                packet::Flag::Create => {},
                                packet::Flag::Join => {},
                                _ => panic!("Received an unexpected message"),
                            }
                        }
                        Err(e) => panic!("{e}"),
                    }
                }
                Err(_) => panic!("Client disconnected"),
            }
        }
    }
    
    /// Initial handshake
    fn init_handshake(&mut self) -> Result<(), &'static str> {
        let mut buffer = [0 as u8; 2048];

        let init_message: [u8;packet::HEADER_SIZE] = [packet::Version::V0 as u8,packet::Flag::Init as u8,0,0,(self.session_tocken >> 8) as u8, (self.session_tocken & 0x0f) as u8,0,0];

        match self.stream.read(&mut buffer) {
            Ok(n) => {
                let _: packet::Packet = packet::Packet::unpack(buffer, n)?;
                match self.stream.write(&init_message) {
                    Ok(_) => {
                        self.status = Status::Initialized;
                        Ok(())
                    }
                    Err(_) => return Err("Unable to send data"),
                }
            },
            Err(_) => return Err("Unable to receive data"),
        }
    }
}