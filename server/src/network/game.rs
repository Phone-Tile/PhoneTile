use log::{info, warn};

use super::player;
use super::{packet, pipe};
use std::process::exit;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time;

pub struct Game {
    tocken: u16,
    target: String,

    // Receiver for the main thread (join request)
    main_receiver: mpsc::Receiver<pipe::ServerMessage>,

    players: Vec<player::Player>,
}

fn test_function(players: &mut Vec<player::Player>) {
    let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
    loop {
        // let mut p1 = &mut players[0];
        // let mut p2 = &mut players[1];
        if players.len() > 1 {
            if players[0].recv(&mut buffer).unwrap() {
                // println!("{buffer:?}");
                players[1].send(&buffer);
            }
            if players[1].recv(&mut buffer).unwrap() {
                players[0].send(&buffer);
            }
        } else {
            let _ = players[0].recv(&mut buffer);
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}

impl Game {
    pub fn new(receiver: mpsc::Receiver<pipe::ServerMessage>, tocken: u16) -> Game {
        let target: String = format!("Room {tocken}");
        Game {
            tocken,
            target,
            main_receiver: receiver,
            players: Vec::new(),
        }
    }

    fn check_for_new_players(&mut self) {
        match self.main_receiver.try_recv() {
            Ok(message) => {
                self.add_player(message.sender);
                info!(target: self.target.as_str(), "Client {} joined the room", message.session_tocken);
            }
            Err(TryRecvError::Empty) => (),
            Err(_) => panic!("Pipe with the server broke unexpectedly"),
        }
    }

    // TODO: might be usefull to warn the other threads before dropping the thread
    fn should_game_launch(&mut self) -> bool {
        // NOTE : we can always ensure that the first player we receive is the master device
        match self.players[0].receiver.try_recv() {
            Ok(_) => true,
            Err(TryRecvError::Empty) => false,
            Err(e) => {
                self.remove_player(0);
                warn!(target: self.target.as_str(), "master client disconnected, shut down the room ...");
                exit(0);
            }
        }
    }

    fn assign_rank(&mut self) {
        // we just assign them in the order they connected, maybe later some randomness would be cool
        let mut index = 0;
        while index < self.players.len() {
            self.players[index].rank = index as u8;
            match self.players[index]
                .sender
                .send(pipe::GameMessage::lock_message(index as u8))
            {
                Ok(_) => index += 1,
                Err(e) => {
                    self.remove_player(index);
                    warn!(target: self.target.as_str(), "client disconnected");
                }
            }
        }
    }

    fn unlock_game(&mut self) {
        let mut index = 0;
        while index < self.players.len() {
            match self.players[index]
                .sender
                .send(pipe::GameMessage::launch_message())
            {
                Ok(_) => index += 1,
                Err(e) => {
                    self.remove_player(index);
                    warn!(target: self.target.as_str(), "client disconnected");
                }
            }
        }

        info!(target: self.target.as_str(), "Game launched");

        // Here we will put the interface code with the client
        test_function(&mut self.players);
    }

    // TODO: might be usefull to warn the other threads before dropping the thread
    fn launch_game(&mut self) {
        match self.players[0].receiver.recv() {
            Ok(_) => self.unlock_game(),
            Err(_) => {
                self.remove_player(0);
                warn!(target: self.target.as_str(), "master client disconnected, shut down the room ...");
                exit(0);
            }
        }
    }

    pub fn manager(&mut self) {
        let mut is_game_on: bool = false;
        info!(target: self.target.as_str(), "Room created successfully");

        while !is_game_on {
            self.check_for_new_players();
            if !self.players.is_empty() && self.should_game_launch() {
                // We go first through a phase of locked game where we send a message to all the players of where they are positioned
                info!(target: self.target.as_str(), "Room locked");
                self.assign_rank();
                self.launch_game();

                is_game_on = false;
            }

            thread::sleep(time::Duration::from_millis(10));
        }
    }

    fn add_player(&mut self, p_sender: mpsc::Sender<pipe::GameMessage>) {
        let (sender, receiver) = mpsc::channel();
        match p_sender.send(pipe::GameMessage::init_message(sender, self.tocken)) {
            Ok(_) => {
                self.players.push(player::Player {
                    sender: p_sender,
                    receiver,
                    rank: 0,
                });
            }
            Err(_) => {
                warn!(target: self.target.as_str(), "client disconnected");
            }
        }
    }

    fn remove_player(&mut self, index: usize) {
        let _ = self.players.swap_remove(index);
        if self.players.len() == 1 {
            match self.players[0]
                .sender
                .send(pipe::GameMessage::error_message())
            {
                Ok(_) => {
                    warn!(target: self.target.as_str(), "too much client disonnected, stop the game")
                }
                Err(_) => warn!(target: self.target.as_str(), "client disconnected"),
            }
            exit(0);
        }
    }
}
