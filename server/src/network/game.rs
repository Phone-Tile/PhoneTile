use log::info;

use super::{pipe, packet};
use std::hash::BuildHasher;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time;
use super::player;

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
        for p in players.iter_mut() {
            if p.recv(&mut buffer) {
                p.send(&buffer);
            }
        }
        thread::sleep(time::Duration::from_millis(100));
    }
}

impl Game {
    pub fn new(receiver: mpsc::Receiver<pipe::ServerMessage>, tocken: u16) -> Game {
        let target: String = format!("Room {}", tocken);
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
            },
            Err(TryRecvError::Empty) => (),
            Err(_) => panic!("Pipe with the server broke unexpectedly"),
        }
    }

    fn should_game_launch(&self) -> bool {
        // NOTE : we can always ensure that the first player we receive is the master device
        match self.players[0].receiver.try_recv() {
            Ok(_) => true,
            Err(TryRecvError::Empty) => false,
            Err(e) => panic!("Pipe with the master client broke unexpectedly : {e}"),
        }
    }

    fn assign_rank(&mut self) {
        // we just assign them in the order they connected, maybe later some randomness would be cool
        let mut index = 0;
        while index < self.players.len() {
            self.players[index].rank = index as u8;
            self.players[index].sender.send(pipe::GameMessage::lock_message(index as u8)).unwrap();
            index += 1;
        }
    }
    
    fn unlock_game(&mut self) {
        for p in self.players.iter() {
            p.sender.send(pipe::GameMessage::launch_message()).unwrap();
        }

        info!(target: self.target.as_str(), "Game launched");

        // Here we will put the interface code with the client
        test_function(&mut self.players);
    }

    fn launch_game(&mut self) {
        match self.players[0].receiver.recv() {
            Ok(_) => self.unlock_game(),
            Err(_) => panic!("Pipe with the master client broke unexpectedly"),
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
        p_sender.send(pipe::GameMessage::init_message(sender, self.tocken)).unwrap();
        self.players.push(
            player::Player {
                sender: p_sender,
                receiver,
                rank: 0,
            }
        );
    }
}