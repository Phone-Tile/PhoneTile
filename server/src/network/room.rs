use log::{info, warn};

use super::{client, player};
use super::{packet, pipe};
use std::io::{Error, ErrorKind};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time;

//////////////////////////////////////////////
///
///
/// Room structure
///
///
//////////////////////////////////////////////

pub struct Room {
    token: u16,
    target: String,

    game_id: client::Game,

    // Receiver for the main thread (join request)
    main_receiver: mpsc::Receiver<pipe::ServerMessage>,

    players: Vec<player::Player>,
}

impl Room {
    //////////////////////////////////////////////
    ///
    ///
    /// Manager
    ///
    ///
    //////////////////////////////////////////////

    pub fn new(receiver: mpsc::Receiver<pipe::ServerMessage>, token: u16) -> Room {
        let target: String = format!("Room {token}");
        Room {
            token,
            target,
            game_id: client::Game::Unknown,
            main_receiver: receiver,
            players: Vec::new(),
        }
    }

    pub fn manager(&mut self) -> Result<(), Error> {
        let mut is_game_on: bool = false;
        info!(target: self.target.as_str(), "Room created successfully");

        while !is_game_on {
            self.check_for_new_players();
            if !self.players.is_empty() && self.should_game_launch()? {
                // We go first through a phase of locked game where we send a message to all the players of where they are positioned
                info!(target: self.target.as_str(), "Room locked");
                self.assign_rank();
                self.launch_game()?;

                is_game_on = false;
            }

            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Helpers
    ///
    ///
    //////////////////////////////////////////////

    fn check_for_new_players(&mut self) {
        match self.main_receiver.try_recv() {
            Ok(message) => {
                info!(target: self.target.as_str(), "Client {} joined the room", message.session_token);
                self.add_player(message);
            }
            Err(TryRecvError::Empty) => (),
            Err(_) => panic!("Pipe with the server broke unexpectedly"),
        }
    }

    // TODO: might be usefull to warn the other threads before dropping the thread
    fn should_game_launch(&mut self) -> Result<bool, Error> {
        // NOTE : we can always ensure that the first player we receive is the master device
        match self.players[0].receiver.try_recv() {
            Ok(message) => {
                self.game_id = message.rank.unwrap().into(); // should never be None
                Ok(true)
            }
            Err(TryRecvError::Empty) => Ok(false),
            Err(e) => {
                self.remove_player(0);
                Err(Error::new(
                    ErrorKind::Interrupted,
                    "master client disconnected",
                ))
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
                .send(pipe::GameMessage::lock_message(index as u16))
            {
                Ok(_) => index += 1,
                Err(e) => {
                    self.remove_player(index);
                    warn!(target: self.target.as_str(), "client disconnected");
                }
            }
        }
        self.set_player_phone_location();
    }

    fn unlock_game(&mut self) -> Result<(), Error> {
        let mut index = 0;
        while index < self.players.len() {
            match self.players[index]
                .sender
                .send(pipe::GameMessage::launch_message(self.game_id.into()))
            {
                Ok(_) => index += 1,
                Err(e) => {
                    self.remove_player(index);
                    warn!(target: self.target.as_str(), "client disconnected");
                }
            }
        }

        info!(target: self.target.as_str(), "Game {} launched", self.game_id);

        // Here we will put the interface code with the client
        match self.game_id {
            client::Game::Racer => crate::game::racer::racer(&mut self.players),
            client::Game::Snake => Ok(()),
            client::Game::MazeFight => crate::game::maze_fight::maze_fight(&mut self.players),
            client::Game::Test => test_function(&mut self.players),
            client::Game::Unknown => Ok(()),
        }
    }

    // TODO: might be usefull to warn the other threads before dropping the thread
    fn launch_game(&mut self) -> Result<(), Error> {
        match self.players[0].receiver.recv() {
            Ok(_) => self.unlock_game(),
            Err(_) => {
                self.remove_player(0)?;
                Err(Error::new(
                    ErrorKind::Interrupted,
                    "master client disconnected",
                ))
            }
        }
    }

    fn add_player(&mut self, message: pipe::ServerMessage) {
        let (sender, receiver) = mpsc::channel();
        match message
            .sender
            .send(pipe::GameMessage::init_message(sender, self.token))
        {
            Ok(_) => {
                self.players.push(player::Player {
                    sender: message.sender,
                    receiver,
                    rank: 0,
                    top_left_x: 0.,
                    top_left_y: 0.,
                    physical_height: message.physical_height,
                    physical_width: message.physical_width,
                    window_height: message.window_height,
                    window_width: message.window_width,
                });
            }
            Err(_) => {
                warn!(target: self.target.as_str(), "client disconnected");
            }
        }
    }

    fn remove_player(&mut self, index: usize) -> Result<(), Error> {
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
            return Err(Error::new(
                ErrorKind::Interrupted,
                "master client disconnected",
            ));
        }
        Ok(())
    }

    fn set_player_phone_location(&mut self) {
        // find max
        let mut max_height: f32 = 0.;
        for p in &self.players {
            if p.physical_height > max_height {
                max_height = p.physical_height
            }
        }

        let mut current_x: f32 = 0.;
        // compute pos by upward rank
        for i in 0..self.players.len() {
            let j = self.find_player_of_rank(i as u8);
            self.players[j].top_left_x = current_x;
            current_x += self.players[j].physical_width;
            self.players[j].top_left_y = (max_height - self.players[j].physical_height) / 2.;
        }
    }

    fn find_player_of_rank(&self, rank: u8) -> usize {
        for i in 0..self.players.len() {
            if self.players[i].rank == rank {
                return i;
            }
        }
        panic!("should never happened")
    }
}

//////////////////////////////////////////////
///
///
/// Test server
///
///
//////////////////////////////////////////////

fn test_function(players: &mut Vec<player::Player>) -> Result<(), Error> {
    let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
    loop {
        // let mut p1 = &mut players[0];
        // let mut p2 = &mut players[1];
        if players.len() > 1 {
            if players[0].recv(&mut buffer).unwrap() > 0 {
                // println!("{buffer:?}");
                players[1].send(&buffer);
            }
            if players[1].recv(&mut buffer).unwrap() > 0 {
                players[0].send(&buffer);
            }
        } else {
            let _ = players[0].recv(&mut buffer);
        }
        thread::sleep(time::Duration::from_millis(10));
    }
    Ok(())
}
