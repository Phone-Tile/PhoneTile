use std::{default, ffi::c_int, thread, time, vec};

use crate::{
    network::{self, packet},
    ui::{button::snake, colors},
};
use c_char;
use raylib::{self, draw, raylib_str};
use std::collections::VecDeque;
use std::convert::TryInto;

//////////////////////////////////////////////
///
///
/// Entry point
///
///
//////////////////////////////////////////////

const BLOCK_SIZE: usize = 50;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            _ => Direction::Down,
        }
    }
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Left => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Down => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct DiscreteVec {
    x: c_int,
    y: c_int,
}

#[derive(Clone, Debug)]
struct Snake {
    tail: Vec<DiscreteVec>,
}

pub unsafe fn main_game(network: &mut network::Network) {
    // pub unsafe fn main_game() {

    let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
    while network.recv(&mut buffer) == 0 {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let mut block_size = 0.;
    let mut tmp = [0_u8; 4];
    tmp.copy_from_slice(&buffer[..4]);
    let block_height = f32::from_be_bytes(tmp);
    tmp.copy_from_slice(&buffer[4..8]);
    block_size = f32::from_be_bytes(tmp).max(block_height);

    let mut food = vec![];
    let mut snakes = vec![];

    let mut record_central_pos = raylib::Vector2 { x: 0., y: 0. };
    let mut update_pos = raylib::Vector2 { x: 0., y: 0. };

    let mut dir = Direction::Right;

    thread::sleep(time::Duration::from_millis(10));

    loop {
        if raylib::IsMouseButtonPressed(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int) {
            record_central_pos = raylib::GetMousePosition();
        }

        if raylib::IsMouseButtonDown(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int) {
            update_pos = raylib::GetMousePosition();
        }

        if raylib::IsMouseButtonReleased(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int) {
            record_central_pos = raylib::Vector2 { x: 0., y: 0. };
            update_pos = raylib::Vector2 { x: 0., y: 0. };
        }

        if recv_data(network, &mut snakes, &mut food) {
            let mut speed = raylib::Vector2 {
                x: update_pos.x - record_central_pos.x,
                y: update_pos.y - record_central_pos.y,
            };
            let norm = (speed.x * speed.x + speed.y * speed.y).sqrt();
            if norm > 0. {
                speed.x /= norm;
                speed.y /= norm;

                let threashold: f32 = (2_f32).sqrt() / 2.;

                if speed.x > threashold {
                    dir = Direction::Right;
                } else if -speed.x > threashold {
                    dir = Direction::Left;
                } else if speed.y > threashold {
                    dir = Direction::Down;
                } else if -speed.y > threashold {
                    dir = Direction::Up;
                }
            }

            send_data(network, dir);
        }

        draw!({
            raylib::ClearBackground(raylib::Color {
                r: 65,
                g: 65,
                b: 65,
                a: 255,
            });
            for s in snakes.iter() {
                for p in s.tail.iter() {
                    raylib::DrawRectangle(
                        p.x,
                        p.y,
                        block_size as c_int,
                        block_size as c_int,
                        raylib::Color {
                            r: 255,
                            g: 0,
                            b: 0,
                            a: 255,
                        },
                    );
                }
            }
            for f in food.iter() {
                raylib::DrawRectangle(
                    f.x,
                    f.y,
                    block_size as c_int,
                    block_size as c_int,
                    raylib::Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255,
                    },
                );
            }
        });
    }
}

unsafe fn recv_data(
    network: &mut network::Network,
    snakes: &mut Vec<Snake>,
    food: &mut Vec<DiscreteVec>,
) -> bool {
    let mut _buffer = [0_u8; packet::MAX_DATA_SIZE];

    let size = network.recv(&mut _buffer);
    let mut buffer = _buffer.as_mut_slice();
    if size > 0 {
        let mut n = u8::from_be(buffer[0]) as usize;
        snakes.clear();
        buffer = &mut buffer[1..];
        for i in 0..n {
            let ns = u8::from_be(buffer[0]) as usize;
            buffer = &mut buffer[1..];
            let mut s = Snake {
                tail: vec::Vec::with_capacity(ns.into()),
            };
            for j in 0..ns {
                let mut tmp = [0_u8; 4];
                tmp.copy_from_slice(&mut buffer[j * 8..j * 8 + 4]);
                let x = f32::from_be_bytes(tmp);
                tmp.copy_from_slice(&mut buffer[j * 8 + 4..j * 8 + 8]);
                let y = f32::from_be_bytes(tmp);

                s.tail.push(DiscreteVec {
                    x: x as c_int,
                    y: y as c_int,
                });
            }
            snakes.push(s);
            buffer = &mut buffer[ns * 8..];
        }

        n = u8::from_be(buffer[0]) as usize;
        food.clear();
        buffer = &mut buffer[1..];
        for i in 0..n {
            let mut tmp = [0_u8; 4];
            tmp.copy_from_slice(&mut buffer[..4]);
            let x = f32::from_be_bytes(tmp);
            tmp.copy_from_slice(&mut buffer[4..8]);
            let y = f32::from_be_bytes(tmp);

            food.push(DiscreteVec {
                x: x as c_int,
                y: y as c_int,
            });
            buffer = &mut buffer[8..];
        }
        true
    } else {
        false
    }
}

fn send_data(network: &mut network::Network, dir: Direction) {
    network.send(&[u8::from(dir).to_be()]).unwrap();
}
