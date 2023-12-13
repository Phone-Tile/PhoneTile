use std::{time, ffi::c_int, thread, vec, default};

use crate::{
    network::{self, packet},
    ui::{colors, button::snake},
};
use raylib::{self, draw, raylib_str};
use std::collections::VecDeque;
use std::convert::TryInto;
use c_char;

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
    thread::sleep(time::Duration::from_millis(50));

    let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
    network.recv(&mut buffer);

    let mut tmp = [0_u8; 4];
    tmp.copy_from_slice(&buffer[..4]);
    let block_height = f32::from_be_bytes(tmp);
    tmp.copy_from_slice(&buffer[4..8]);
    let block_width = f32::from_be_bytes(tmp);

    let mut food = vec![];
    let mut snakes = vec![];

    let mut record_central_pos = raylib::Vector2 { x: 0., y: 0. };
    let mut update_pos = raylib::Vector2 { x: 0., y: 0. };

    thread::sleep(time::Duration::from_millis(10));

    loop {
        if raylib::IsMouseButtonPressed(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int) {
            record_central_pos = raylib::GetMousePosition();
        }

        if raylib::IsMouseButtonDown(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int) {
            update_pos = raylib::GetMousePosition();
        }

        if raylib::IsMouseButtonReleased(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int)
        {
            record_central_pos = raylib::Vector2 { x: 0., y: 0. };
            update_pos = raylib::Vector2 { x: 0., y: 0. };
        }

        recv_data(network, &mut snakes, &mut food);
        let mut speed = raylib::Vector2 {
            x: update_pos.x - record_central_pos.x,
            y: update_pos.y - record_central_pos.y,
        };
        send_data(network, &mut speed);

        draw!({
            raylib::ClearBackground(raylib::Color {
                r: 65,
                g: 65,
                b: 65,
                a: 255,
            });
            for s in snakes.iter() {
                for p in s.tail.iter() {
                    raylib::DrawRectangle(p.x, p.y, block_width as c_int, block_height as c_int, raylib::Color { r: 255, g: 0, b: 0, a: 255 });
                }
            }
            for f in food.iter() {
                raylib::DrawRectangle(f.x, f.y, block_width as c_int, block_height as c_int, raylib::Color { r: 0, g: 0, b: 0, a: 255 });
            }
        });
    }







    // let mut food = vec![DiscreteVec { x: 20, y: 30 }];

    // let mut snake = Snake {
    //     tail: VecDeque::new(),
    //     dir: Direction::Right,
    // };
    // snake.tail.push_back(DiscreteVec { x: 10, y: 10 });
    // snake.tail.push_back(DiscreteVec { x: 10, y: 11 });
    // snake.tail.push_back(DiscreteVec { x: 10, y: 12 });
    // snake.tail.push_back(DiscreteVec { x: 10, y: 13 });

    // let mut update_timer = time::Instant::now();
    // let mut new_fruit_timer = time::Instant::now();

    // let mut record_central_pos = raylib::Vector2 { x: 0., y: 0. };
    // let mut update_pos = raylib::Vector2 { x: 0., y: 0. };

    // loop {
    //     if update_timer.elapsed().as_millis() > 300 {
    //         let mut p = snake.tail[0].clone();
    //         match snake.dir {
    //             Direction::Down => p.y += 1,
    //             Direction::Up => p.y -= 1,
    //             Direction::Left => p.x -= 1,
    //             Direction::Right => p.x += 1,
    //         }
    //         let mut i = 0;
    //         let mut should_pop = true;
    //         while i < food.len() {
    //             let f = food[i];
    //             if f.x == p.x && f.y == p.y {
    //                 food.swap_remove(i);
    //                 should_pop = false;
    //                 break;
    //             } else {
    //                 i += 1;
    //             }
    //         }
    //         if should_pop {
    //             snake.tail.pop_back().unwrap();
    //         }
    //         snake.tail.push_front(p);
    //         update_timer = time::Instant::now();
    //     }

    //     if raylib::IsMouseButtonPressed(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int) {
    //         record_central_pos = raylib::GetMousePosition();
    //     }

    //     if raylib::IsMouseButtonDown(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int) {
    //         update_pos = raylib::GetMousePosition();
    //     }

    //     if raylib::IsMouseButtonReleased(raylib::MouseButton_MOUSE_BUTTON_LEFT as c_int)
    //     {
    //         record_central_pos = raylib::Vector2 { x: 0., y: 0. };
    //         update_pos = raylib::Vector2 { x: 0., y: 0. };
    //     }

    //     let mut speed = raylib::Vector2 {
    //         x: update_pos.x - record_central_pos.x,
    //         y: update_pos.y - record_central_pos.y,
    //     };
    //     let norm = (speed.x*speed.x + speed.y * speed.y).sqrt();
    //     if norm > 0. {
    //         speed.x /= norm;
    //         speed.y /= norm;

    //         let threashold: f32 = (2_f32).sqrt()/2.;

    //         if speed.x > threashold && snake.dir != Direction::Left {
    //             snake.dir = Direction::Right;
    //         } else if -speed.x > threashold && snake.dir != Direction::Right {
    //             snake.dir = Direction::Left;
    //         } else if speed.y > threashold && snake.dir != Direction::Up {
    //             snake.dir = Direction::Down;
    //         } else if -speed.y > threashold && snake.dir != Direction::Down {
    //             snake.dir = Direction::Up;
    //         }
    //     }

    //     draw!({
    //         raylib::ClearBackground(raylib::Color {
    //             r: 65,
    //             g: 65,
    //             b: 65,
    //             a: 255,
    //         });
    //         for p in snake.tail.iter() {
    //             raylib::DrawRectangle((p.x * BLOCK_SIZE) as c_int, (p.y * BLOCK_SIZE) as c_int, BLOCK_SIZE  as c_int, BLOCK_SIZE  as c_int, raylib::Color { r: 255, g: 0, b: 0, a: 255 });
    //         }
    //         for f in food.iter() {
    //             raylib::DrawRectangle((f.x * BLOCK_SIZE) as c_int, (f.y * BLOCK_SIZE) as c_int, BLOCK_SIZE  as c_int, BLOCK_SIZE  as c_int, raylib::Color { r: 0, g: 0, b: 0, a: 255 });
    //         }
    //     });
    // }
}

unsafe fn recv_data(network: &mut network::Network, snakes: &mut Vec<Snake>, food: &mut Vec<DiscreteVec>) -> bool {
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
                tmp.copy_from_slice(&mut buffer[j*8..j*8+4]);
                let x = f32::from_be_bytes(tmp);
                tmp.copy_from_slice(&mut buffer[j*8+4..j*8+8]);
                let y = f32::from_be_bytes(tmp);

                s.tail.push(DiscreteVec { x: x as c_int, y: y as c_int });
            }
            buffer = &mut buffer[ns*8..];
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

            food.push(DiscreteVec { x: x as c_int, y: y as c_int });
            buffer = &mut buffer[8..];
        }
        raylib::TraceLog(
            raylib::TraceLogLevel_LOG_ERROR.try_into().unwrap(),
            raylib_str!(format!("BITE{:?}", snakes)),
        );
        true
    } else {
        false
    }
}

fn send_data(network: &mut network::Network, speed: &mut raylib::Vector2) {
    let norm = (speed.x*speed.x + speed.y * speed.y).sqrt();
    if norm > 0. {
        speed.x /= norm;
        speed.y /= norm;

        let threashold: f32 = (2_f32).sqrt()/2.;

        let mut dir = Direction::Right;

        if speed.x > threashold {
            dir = Direction::Right;
        } else if -speed.x > threashold {
            dir = Direction::Left;
        } else if speed.y > threashold {
            dir = Direction::Down;
        } else if -speed.y > threashold {
            dir = Direction::Up;
        }
        network.send(&[dir.into()]);
    } else {
        network.send(&[Direction::Right.into()]).unwrap();
    }
}



