use crate::network::player;

use super::bullet;
use super::maze;
use super::powerup;
use super::weapon;
use super::Vector2;

use std::time;
use std::vec;

const SPRITE_WIDTH: f32 = 80.;
const SPRITE_HEIGHT: f32 = SPRITE_WIDTH * 36. / 30.;

pub struct Sprite {
    pos: Vector2,
    pub speed: Vector2,
    id: usize,
    timer: time::Instant,
    skin: usize,
    is_dead: bool,
    modifiers: Vec<powerup::PowerUp>,
    life: usize,
}

impl Sprite {
    pub fn create_sprites(players: &[player::Player]) -> Vec<Self> {
        let mut sprites = vec::Vec::new();
        for (i, p) in players.iter().enumerate() {
            sprites.push(Self {
                pos: Vector2 {
                    x: p.physical_width / 2. + p.top_left_x,
                    y: p.physical_height / 2. + p.top_left_y,
                },
                speed: Vector2 { x: 0., y: 0. },
                id: p.rank as usize,
                timer: time::Instant::now(),
                skin: i + 3,
                is_dead: false,
                modifiers: vec::Vec::new(),
                life: 10,
            });
        }
        sprites
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_modifiers(&self) -> &Vec<powerup::PowerUp> {
        &self.modifiers
    }

    pub fn get_life(&self) -> usize {
        self.life
    }

    pub fn update_sprite_status(
        &mut self,
        maze: &[maze::Wall],
        bullets: &mut Vec<bullet::Bullet>,
        internal_timer: time::Instant,
    ) {
        if !self.is_dead {
            let mut size_modifiers = 0.;
            let mut firing_speed_modifiers = weapon::FIRERING_SPEED as f32;
            for m in self.modifiers.iter() {
                if m.get_type() == powerup::Type::SizeUp || m.get_type() == powerup::Type::SizeDown
                {
                    size_modifiers += m.modifier();
                }
                if m.get_type() == powerup::Type::FiringRateDown
                    || m.get_type() == powerup::Type::FiringRateUp
                {
                    firing_speed_modifiers += m.modifier();
                }
            }
            let mut norm = self.speed.x * self.speed.x + self.speed.y * self.speed.y;
            norm = norm.sqrt();
            if self.timer.elapsed().as_millis() > firing_speed_modifiers as u128 && norm > 0. {
                self.timer = std::time::Instant::now();
                bullets.push(bullet::Bullet::new(
                    Vector2 {
                        x: self.pos.x + (size_modifiers + SPRITE_WIDTH) / 2.,
                        y: self.pos.y + (size_modifiers + SPRITE_HEIGHT) / 2.,
                    },
                    Vector2 {
                        x: bullet::BULLET_SPEED * self.speed.x / norm,
                        y: bullet::BULLET_SPEED * self.speed.y / norm,
                    },
                    self.id,
                ));
            }

            self.pos.x += self.speed.x * internal_timer.elapsed().as_secs_f32() * 50.;
            self.pos.y += self.speed.y * internal_timer.elapsed().as_secs_f32() * 50.;

            for w in maze.iter() {
                w.realign_sprite(
                    &mut self.pos,
                    (SPRITE_WIDTH + size_modifiers) as usize,
                    (SPRITE_HEIGHT + size_modifiers) as usize,
                );
            }

            let mut i = 0;
            while i < self.modifiers.len() {
                if !self.modifiers[i].is_activated() {
                    self.modifiers.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn update_dead_status(&mut self, bullets: &mut Vec<bullet::Bullet>) {
        if !self.is_dead {
            let mut i = 0;
            while i < bullets.len() {
                let b = &bullets[i];
                if b.id != self.id
                    && b.pos.x > self.pos.x
                    && b.pos.x < self.pos.x + SPRITE_WIDTH
                    && b.pos.y > self.pos.y
                    && b.pos.y < self.pos.y + SPRITE_HEIGHT
                {
                    self.life -= 1;
                    if self.life == 0 {
                        self.is_dead = true;
                    }
                    bullets.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn update_powerup_status(&mut self, powerups: &mut Vec<powerup::PowerUp>) {
        if !self.is_dead {
            let mut i = 0;
            while i < powerups.len() {
                let powerup = &mut powerups[i];
                if powerup.pos().x > self.pos.x
                    && powerup.pos().x < self.pos.x + SPRITE_WIDTH
                    && powerup.pos().y > self.pos.y
                    && powerup.pos().y < self.pos.y + SPRITE_HEIGHT
                {
                    powerup.activate();
                    self.modifiers.push(*powerup);

                    powerups.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn pack_sprites(sprites: &[Self]) -> Vec<u8> {
        // We need : initial pos, skin, id
        let mut data = vec::Vec::new();
        data.push((sprites.len() as u8).to_be());

        for s in sprites.iter() {
            data.push((s.skin as u8).to_be());
            data.push((s.id as u8).to_be());
        }

        data
    }

    pub fn pack_game_sprites(sprites: &[Self], player: &player::Player) -> Vec<u8> {
        let mut len: u8 = 0;
        for s in sprites.iter() {
            if !s.is_dead {
                len += 1;
            }
        }

        let mut data = vec::Vec::new();
        data.push(len.to_be());

        for s in sprites.iter() {
            // we need to send the id, pos, speed and size
            if !s.is_dead {
                data.push((s.id as u8).to_be());

                let (x, y) = player.to_local_coordinates(s.pos.x, s.pos.y);

                let pos_x = x.to_be_bytes();
                let pos_y = y.to_be_bytes();

                data.append(&mut pos_x.to_vec());
                data.append(&mut pos_y.to_vec());

                let vx = player.to_local_proportion_horizontal(s.speed.x);
                let vy = player.to_local_proportion_horizontal(s.speed.y);

                let speed_x = vx.to_be_bytes();
                let speed_y = vy.to_be_bytes();

                data.append(&mut speed_x.to_vec());
                data.append(&mut speed_y.to_vec());

                let mut size_modifiers = 0.;
                for m in s.modifiers.iter() {
                    if m.get_type() == powerup::Type::SizeUp
                        || m.get_type() == powerup::Type::SizeDown
                    {
                        size_modifiers += m.modifier();
                    }
                }

                let _width = player.to_local_proportion_horizontal(size_modifiers + SPRITE_WIDTH);
                let _height = player.to_local_proportion_vertical(size_modifiers + SPRITE_HEIGHT);

                let width = _width.to_be_bytes();
                let height = _height.to_be_bytes();

                data.append(&mut width.to_vec());
                data.append(&mut height.to_vec());
            }
        }
        data
    }
}
