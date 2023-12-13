use crate::network::packet;
use crate::network::{self, player};
use rand;
use std::collections::VecDeque;
use std::io::Error;
use std::time;
use std::{thread, vec};

pub const BLOCK_SIZE: i32 = 50;

//////////////////////////////////////////////
///
///
/// Entry point
///
///
//////////////////////////////////////////////

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

#[derive(Clone, Copy, Debug)]
struct DiscreteVec {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Snake {
    tail: VecDeque<DiscreteVec>,
    dir: Direction,
    is_dead: bool,
}

pub fn snake(players: &mut [network::player::Player]) -> Result<(), Error> {
    let mut width: f32 = 0.;
    let mut height: f32 = 0.;
    for p in players.iter() {
        width += p.physical_width;
        height = height.max(p.physical_height);
    }

    for p in players.iter_mut() {
        let (block_width, block_height) = (
            p.to_local_proportion_vertical(BLOCK_SIZE as f32),
            p.to_local_proportion_horizontal(BLOCK_SIZE as f32),
        );
        let mut data = vec![];
        data.append(&mut block_width.to_be_bytes().to_vec());
        data.append(&mut block_height.to_be_bytes().to_vec());
        p.send(&data).unwrap();
    }

    let mut snakes = vec![];
    for p in players.iter() {
        snakes.push(Snake {
            tail: [DiscreteVec {
                x: (p.physical_width / 2. + p.top_left_x) as i32 / BLOCK_SIZE,
                y: (p.physical_height / 2. + p.top_left_y) as i32 / BLOCK_SIZE,
            }]
            .into(),
            dir: Direction::Right,
            is_dead: false,
        });
    }

    let mut food = vec![];
    for _ in players.iter() {
        for _ in 0..3 {
            food.push(DiscreteVec {
                x: (rand::random::<f32>() * width) as i32 / BLOCK_SIZE,
                y: (rand::random::<f32>() * height) as i32 / BLOCK_SIZE,
            })
        }
    }

    for p in players.iter_mut() {
        send_data(p, &snakes, &food);
    }

    let mut update_timer = time::Instant::now();
    let mut new_fruit_timer = time::Instant::now();

    loop {
        if update_timer.elapsed().as_millis() > 300 {
            update_snake_pos(&mut snakes, &mut food);
            update_timer = time::Instant::now();
        }

        if new_fruit_timer.elapsed().as_millis() > 10000 / players.len() as u128 {
            food.push(DiscreteVec {
                x: (rand::random::<f32>() * width) as i32 / BLOCK_SIZE,
                y: (rand::random::<f32>() * height) as i32 / BLOCK_SIZE,
            });
            new_fruit_timer = time::Instant::now();
        }

        let snakes_copy = snakes.clone();

        for (p, s) in players.iter_mut().zip(snakes.iter_mut()) {
            if let Some(dir) = recv_data(p) {
                if dir == Direction::Right && s.dir != Direction::Left {
                    s.dir = Direction::Right;
                } else if dir == Direction::Left && s.dir != Direction::Right {
                    s.dir = Direction::Left;
                } else if dir == Direction::Down && s.dir != Direction::Up {
                    s.dir = Direction::Down;
                } else if dir == Direction::Up && s.dir != Direction::Down {
                    s.dir = Direction::Up;
                }
                send_data(p, &snakes_copy, &food);
            }
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}

fn update_snake_pos(snakes: &mut [Snake], food: &mut Vec<DiscreteVec>) {
    let snakes_copy = snakes.to_owned();
    for snake in snakes.iter_mut() {
        if !snake.is_dead {
            let mut p = snake.tail[0];
            match snake.dir {
                Direction::Down => p.y += 1,
                Direction::Up => p.y -= 1,
                Direction::Left => p.x -= 1,
                Direction::Right => p.x += 1,
            }
            let mut i = 0;
            let mut should_pop = true;
            while i < food.len() {
                let f = food[i];
                if f.x == p.x && f.y == p.y {
                    food.swap_remove(i);
                    should_pop = false;
                    break;
                } else {
                    i += 1;
                }
            }
            'iter: for s in snakes_copy.iter() {
                for (i, p) in s.tail.iter().enumerate() {
                    if p.x == snake.tail[0].x && p.y == snake.tail[0].y && i != 0 {
                        snake.is_dead = true;
                        snake.tail.clear();
                        break 'iter;
                    }
                }
            }
            if !snake.is_dead {
                if should_pop {
                    snake.tail.pop_back().unwrap();
                }
                snake.tail.push_front(p);
            }
        }
    }
}

fn recv_data(player: &mut player::Player) -> Option<Direction> {
    let mut buffer = [0_u8; packet::MAX_DATA_SIZE];

    let size = player.recv(&mut buffer).unwrap();
    if size > 0 {
        let outcome = u8::from_be(buffer[0]);
        Some(outcome.into())
    } else {
        None
    }
}

fn send_data(player: &mut player::Player, snakes: &Vec<Snake>, food: &Vec<DiscreteVec>) {
    let mut data = vec![];
    data.push((snakes.len() as u8).to_be());
    for s in snakes.iter() {
        data.push((s.tail.len() as u8).to_be());
        for c in s.tail.iter() {
            let (x, y) =
                player.to_local_coordinates((c.x * BLOCK_SIZE) as f32, (c.y * BLOCK_SIZE) as f32);

            let pos_x = x.to_be_bytes();
            let pos_y = y.to_be_bytes();

            data.append(&mut pos_x.to_vec());
            data.append(&mut pos_y.to_vec());
        }
    }

    data.push((food.len() as u8).to_be());
    for f in food.iter() {
        let (x, y) =
            player.to_local_coordinates((f.x * BLOCK_SIZE) as f32, (f.y * BLOCK_SIZE) as f32);

        let pos_x = x.to_be_bytes();
        let pos_y = y.to_be_bytes();

        data.append(&mut pos_x.to_vec());
        data.append(&mut pos_y.to_vec());
    }

    player.send(&data).unwrap();
}
