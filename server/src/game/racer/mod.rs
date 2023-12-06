#![allow(unused)]
mod bezier;
mod game;
mod vehicle;

use crate::network::packet;
use crate::network::player;
use std::{thread, time};

fn encode_data(game: &game::Game, player: &player::Player) -> Vec<u8> {
    let mut raw_data: Vec<u8> = Vec::new();
    let map = game.get_map();
    let cars = game.get_cars();
    raw_data.push(8 * 2 * (cars.len() as u8));
    for car in cars.iter() {
        let (x, y) = player.to_local_coordinates(car.0 as f32, car.1 as f32);
        let xb = f64::to_be_bytes(x as f64);
        let yb = f64::to_be_bytes(y as f64);
        raw_data.extend(xb);
        raw_data.extend(yb);
    }
    for bezier in map.iter() {
        let p = bezier.get_points();
        let (x, y) =
            player.to_local_coordinates(p.0.into_tuple().0 as f32, p.0.into_tuple().1 as f32);
        let xb = f64::to_be_bytes(x as f64);
        let yb = f64::to_be_bytes(y as f64);
        raw_data.extend(xb);
        raw_data.extend(yb);

        let (x, y) =
            player.to_local_coordinates(p.1.into_tuple().0 as f32, p.1.into_tuple().1 as f32);
        let xb = f64::to_be_bytes(x as f64);
        let yb = f64::to_be_bytes(y as f64);
        raw_data.extend(xb);
        raw_data.extend(yb);

        let (x, y) =
            player.to_local_coordinates(p.2.into_tuple().0 as f32, p.2.into_tuple().1 as f32);
        let xb = f64::to_be_bytes(x as f64);
        let yb = f64::to_be_bytes(y as f64);
        raw_data.extend(xb);
        raw_data.extend(yb);

        let (x, y) =
            player.to_local_coordinates(p.3.into_tuple().0 as f32, p.3.into_tuple().1 as f32);
        let xb = f64::to_be_bytes(x as f64);
        let yb = f64::to_be_bytes(y as f64);
        raw_data.extend(xb);
        raw_data.extend(yb);
    }
    raw_data
}

fn send_data(players: &mut [player::Player], game: &game::Game) {
    for player in players.iter_mut() {
        let raw_data = encode_data(&game, &player);
        player.send(raw_data.as_slice()).err();
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
        phone_accel[i_p] = is_accelerating;
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
            panic!("{:?}", err);
        }
    };
    loop {
        // let raw_data = encode_data(&game);
        send_data(players, &game);
        let phone_accel = recv_data(players);
        for (i_p, _) in players.iter().enumerate() {
            game.update_position(i_p, phone_accel[i_p]);
        }
        thread::sleep(time::Duration::from_millis(20));
    }
    Ok(())
}
