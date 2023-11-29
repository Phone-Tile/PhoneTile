#![allow(unused)]
mod bezier;
mod game;
mod vehicle;

use crate::network::packet;
use crate::network::player;

fn encode_data(game: &game::Game) -> Vec<u8> {
    let mut raw_data: Vec<u8> = Vec::new();
    let map = game.get_map();
    let cars = game.get_cars();
    raw_data.push(8 * 2 * (cars.len() as u8));
    for car in cars.iter() {
        let (x, y) = car;
        let xb = f64::to_be_bytes(*x);
        let yb = f64::to_be_bytes(*y);
        raw_data.extend(xb);
        raw_data.extend(yb);
    }
    for bezier in map.iter() {
        let p = bezier.get_points();
        let (x, y) = p.0.into_tuple();
        let xb = f64::to_be_bytes(x);
        let yb = f64::to_be_bytes(y);
        raw_data.extend(xb);
        raw_data.extend(yb);

        let (x, y) = p.1.into_tuple();
        let xb = f64::to_be_bytes(x);
        let yb = f64::to_be_bytes(y);
        raw_data.extend(xb);
        raw_data.extend(yb);

        let (x, y) = p.2.into_tuple();
        let xb = f64::to_be_bytes(x);
        let yb = f64::to_be_bytes(y);
        raw_data.extend(xb);
        raw_data.extend(yb);

        let (x, y) = p.3.into_tuple();
        let xb = f64::to_be_bytes(x);
        let yb = f64::to_be_bytes(y);
        raw_data.extend(xb);
        raw_data.extend(yb);
    }
    raw_data
}

fn send_data(players: &mut [player::Player], raw_data: &[u8]) {
    for player in players.iter_mut() {
        player.send(raw_data).err();
    }
}
fn recv_data(players: &mut [player::Player]) -> Vec<bool> {
    // when called, we can expect that the buffer received is a boolean, corresponding to having clicked or not
    let mut phone_accel = vec![false; players.len()];
    for (i_p, player) in players.iter_mut().enumerate() {
        let mut can_receive = true;
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
        let mut is_accelerating = false;
        while (can_receive) {
            match player.recv(&mut buffer) {
                Err(_) => can_receive = false,
                Ok(0) => can_receive = false,
                Ok(_) => {
                    is_accelerating = is_accelerating || buffer[0] == 1;
                }
            }
        }
        phone_accel[i_p] = is_accelerating
    }
    phone_accel
}

pub fn racer(players: &mut [player::Player]) -> Result<(), std::io::Error> {
    let mut dimensions = Vec::new();
    for player in players.iter() {
        dimensions.push((player.physical_width as f64, player.physical_height as f64))
    }
    let mut game = match game::Game::new(vec![], players.len(), &dimensions) {
        Ok(game) => game,
        Err(err) => {
            println!("{:?}", err);
            panic!("{:?}", err);           
        },
    };
    loop {
        let raw_data = encode_data(&game);
        send_data(players, &raw_data);
        let phone_accel = recv_data(players);
        for (i_p, _) in players.iter().enumerate() {
            game.update_position(i_p, phone_accel[i_p]);
        }
    }
    Ok(())

    // We first want to get the physical as well as the rendering size (cause I fucked up in the protocol and at this point it's easier to do it like that)

    // Then we want to send the map, unfortunatly we don't have anything working so I have to build a small map on the fly

    // Finally we just have to run the game, simple state machine I hope ... need to use very intensivally to_bits

    // We dont care of who win or loose or anything I DONT CARE I JUST WANT FUCKING CARS WORKING
}
