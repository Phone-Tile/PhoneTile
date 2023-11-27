#![allow(unused)]
mod bezier;
mod vehicle;

use crate::network::player;

pub fn racer(players: &[player::Player]) {
    // We first want to get the phisical as well as the rendering size (cause I fucked up in the protocol and at this point it's easier to do it like that)

    // Then we want to send the map, unfortunatly we don't have anything working so I have to build a small map on the fly

    // Finally we just have to run the game, simple state machine I hope ... need to use very intensivally to_bits

    // We dont care of who win or loose or anything I DONT CARE I JUST WANT FUCKING CARS WORKING
}
