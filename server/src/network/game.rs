use super::pipe;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time;

struct Player {
    sender: mpsc::Sender<pipe::GameMessage>,
    receiver: mpsc::Receiver<pipe::GameMessage>,

    rank: u8,
}

pub struct Game {
    // Receiver for the main thread (join request)
    main_receiver: mpsc::Receiver<pipe::ServerMessage>,

    players: Vec<Player>,
}

impl Game {
    pub fn new(receiver: mpsc::Receiver<pipe::ServerMessage>) -> Game {
        Game {
            main_receiver: receiver,
            players: Vec::new(),
        }
    }

    fn check_for_new_players(&mut self) {
        match self.main_receiver.try_recv() {
            Ok(message) => {
                self.add_player(message.sender)
            },
            Err(e) if (e == TryRecvError::Empty) => return,
            Err(_) => panic!("Pipe with the server broke unexpectedly"),
        }
    }

    fn should_game_launch(&self) -> bool {
        // NOTE : we can always ensure that the first player we receive is the master device
        match self.players[0].receiver.try_recv() {
            Ok(_) => return true,
            Err(e) if e == TryRecvError::Empty => return false,
            Err(_) => panic!("Pipe with the master client broke unexpectedly"),
        }
    }

    fn assign_rank(&mut self) {
        // we just assign them in the order they connected, maybe later some randomness would be cool
        let mut index = 0;
        while index < self.players.len() {
            self.players[index].rank = index as u8;
            self.players[index].sender.send(pipe::GameMessage::lock_message(index as u8));
            index += 1;
        }
    }

    fn unlock_game(&self) {
        for p in self.players.iter() {
            p.sender.send(pipe::GameMessage::launch_message());
        }

        // Here we will put the interface code with the client
    }

    fn launch_game(&self) {
        match self.players[0].receiver.recv() {
            Ok(_) => self.unlock_game(),
            Err(_) => panic!("Pipe with the master client broke unexpectedly"),
        }
    }

    pub fn manager(&mut self) {
        let mut is_game_on: bool = false;

        while !is_game_on {
            self.check_for_new_players();
            if self.should_game_launch() {
                // We go first through a phase of locked game where we send a message to all the players of where they are positioned
                self.assign_rank();
                self.launch_game();
                
                is_game_on = false;
            }

            thread::sleep(time::Duration::from_millis(10));
        }
    }

    fn add_player(&mut self, p_sender: mpsc::Sender<pipe::GameMessage>) {
        let (sender, receiver) = mpsc::channel();
        p_sender.send(pipe::GameMessage::init_message(sender));
        self.players.push(
            Player {
                sender: p_sender,
                receiver: receiver,
                rank: 0,
            }
        );
    }
}