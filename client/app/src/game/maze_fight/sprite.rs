use raylib;
use raylib::Vector2;
use std::ffi::{c_float, c_int};
use std::time;
use std::vec;

use crate::game::maze_fight::sprite;

pub struct Sprite {
    pos: raylib::Vector2,
    speed: raylib::Vector2,
    height: c_float,
    width: c_float,

    id: usize,

    skin: usize,
    internal_state: usize,
    internal_direction: usize,
    internal_state_timer: time::Instant,

    is_alive: bool,
}

impl Sprite {
    pub fn unpack_game_sprites(sprites: &mut [Self], _data: &[u8]) -> Vec<u8> {
        const SIZE: usize = 25;

        let mut update_recorder = vec::Vec::with_capacity(sprites.len());
        for s in sprites.iter() {
            update_recorder.push(false);
        }

        let len = u8::from_be(_data[0]) as usize;
        let data = &_data[1..];

        for i in 0..len {
            let id = u8::from_be(data[i * SIZE]) as usize;

            let mut buffer = [0_u8; 4];

            buffer.copy_from_slice(&data[i * SIZE + 1..i * SIZE + 5]);
            let pos_x = f32::from_be_bytes(buffer);
            buffer.copy_from_slice(&data[i * SIZE + 5..i * SIZE + 9]);
            let pos_y = f32::from_be_bytes(buffer);

            sprites[id].pos.x = pos_x;
            sprites[id].pos.y = pos_y;

            buffer.copy_from_slice(&data[i * SIZE + 9..i * SIZE + 13]);
            let speed_x = f32::from_be_bytes(buffer);
            buffer.copy_from_slice(&data[i * SIZE + 13..i * SIZE + 17]);
            let speed_y = f32::from_be_bytes(buffer);

            sprites[id].speed.x = speed_x;
            sprites[id].speed.y = speed_y;

            buffer.copy_from_slice(&data[i * SIZE + 17..i * SIZE + 21]);
            let width = f32::from_be_bytes(buffer);
            buffer.copy_from_slice(&data[i * SIZE + 21..i * SIZE + 25]);
            let height = f32::from_be_bytes(buffer);

            sprites[id].width = width;
            sprites[id].height = height;
            update_recorder[id] = true;
        }

        for (i, s) in sprites.iter_mut().enumerate() {
            s.is_alive = update_recorder[i];
        }

        data[len * SIZE..].to_vec()
    }

    pub fn unpack_sprites(_data: &[u8]) -> Vec<Self> {
        let len = u8::from_be(_data[0]) as usize;
        let data = &_data[1..];

        let mut sprites = vec::Vec::with_capacity(len);

        for i in 0..len {
            sprites.push(Sprite {
                pos: Vector2 { x: 0., y: 0. },
                speed: Vector2 { x: 0., y: 0. },
                height: 0.,
                width: 0.,
                id: i,
                skin: 0,
                internal_state: 1,
                internal_direction: 2,
                internal_state_timer: time::Instant::now(),
                is_alive: true,
            });
        }

        for i in 0..len {
            let skin = u8::from_be(data[i * 2]);
            let id = u8::from_be(data[i * 2 + 1]) as usize;

            sprites[id].skin = skin as usize;
        }

        sprites
    }

    pub fn update_sprite_pos(&mut self, timer: time::Instant, maze: &Vec<super::wall::Wall>) {
        self.pos.x += self.speed.x * timer.elapsed().as_secs_f32() * 50.;
        self.pos.y += self.speed.y * timer.elapsed().as_secs_f32() * 50.;

        for w in maze.iter() {
            w.realign_sprite(&mut self.pos, self.width as c_int, self.height as c_int);
        }

        if self.speed.x.abs() + self.speed.y.abs() > 0. {
            if self.internal_state_timer.elapsed().as_millis() > 400 {
                self.internal_state_timer = time::Instant::now();
                self.internal_state = (self.internal_state + 1) % 3;
            }
            if self.speed.x.abs() > self.speed.y.abs() {
                if self.speed.x > 0. {
                    self.internal_direction = 1;
                } else {
                    self.internal_direction = 3;
                }
            } else {
                if self.speed.y > 0. {
                    self.internal_direction = 2;
                } else {
                    self.internal_direction = 0;
                }
            }
        } else {
            self.internal_state = 1;
        }
    }

    pub fn get_pos(&self) -> raylib::Vector2 {
        self.pos
    }

    pub fn get_size(&self) -> (c_float, c_float) {
        (self.width, self.height)
    }

    pub fn get_state(&self) -> usize {
        self.internal_state
    }

    pub fn get_direction(&self) -> usize {
        self.internal_direction
    }

    pub fn get_skin(&self) -> usize {
        self.skin
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}
