use crate::network::Network;
use std::fmt::Display;


//////////////////////////////////////////////
///
///
/// Game flag
/// EDIT THE FOLLOWING IF YOU WANT TO ADD A GAME
///
///
//////////////////////////////////////////////

/// add your module
pub mod racer;
pub mod snake;
pub mod maze_fight;

/// add your game in enum
#[derive(Clone, Copy, PartialEq)]
pub enum Game {
    Racer,
    Snake,
    MazeFight,
    Test,
    Unknown,
}

/// add unique game id in the two following functions

impl From<Game> for u16 {
    fn from(value: Game) -> Self {
        match value {
            Game::Racer => 1,
            Game::Snake => 2,
            Game::MazeFight => 3,
            Game::Test => 0x80,
            Game::Unknown => 0xff,
        }
    }
}

impl From<u16> for Game {
    fn from(value: u16) -> Self {
        match value {
            1 => Game::Racer,
            2 => Game::Snake,
            3 => Game::MazeFight,
            0x80 => Game::Test,
            _ => Game::Unknown,
        }
    }
}

/// Add your game title
pub fn title(game_id: u16) -> String {
    String::from(
    match game_id {
        1 => "Racer",
        2 => "Snake",
        3 => "Maze Fight",
        _ => "Unknown ???",
    })
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::Racer => write!(f, "Racer"),
            Game::Snake => write!(f, "Snake"),
            Game::MazeFight => write!(f, "MazeFight"),
            Game::Test => write!(f, "Test"),
            _ => write!(f, "Unknown"),
        }
    }
}


impl Game {
    /// Add your game structure : title, main_game function, and max number of players
    pub fn create_game(&self) -> GameStruct {
        match self {
            Game::Racer => GameStruct::new(String::from("Racer"), Box::new(racer::main_game), None),
            Game::MazeFight => GameStruct::new(String::from("Maze Fight"), Box::new(|network| unsafe{maze_fight::main_game(network)}), Some(9)),
            _ => panic!("this game is still awaiting for your awesome code ..."),
        }
    }
}

/////////////////////////////////////
/// 
/// END EDITING
/// Game structure
/// 
/////////////////////////////////////


pub struct GameStruct{
    title: String,
    //description: String,
    main_game: Box<dyn Fn(&mut Network)>,
    max_player: Option<usize>
}

impl GameStruct {
    pub fn new(
        title: String,
        //description: String,
        main_game: Box<dyn Fn(&mut Network)>,
        max_player: Option<usize>)
        -> GameStruct {
        GameStruct {title, main_game, max_player}
    }
}