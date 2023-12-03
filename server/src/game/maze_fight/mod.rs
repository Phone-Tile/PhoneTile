use crate::network::packet;
use crate::network::{self, player};
use rand;
use std::io::Error;
use std::thread;
use std::time;
use std::vec;

//////////////////////////////////////////////
///
///
/// Modules
///
///
//////////////////////////////////////////////
mod bullet;
mod maze;
mod powerup;
mod sprite;
mod weapon;

//////////////////////////////////////////////
///
///
/// Constants
///
///
//////////////////////////////////////////////

const BULLET_SIZE: usize = 20;

//////////////////////////////////////////////
///
///
/// Vector
///
///
//////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

//////////////////////////////////////////////
///
///
/// Entry point
///
///
//////////////////////////////////////////////

pub fn maze_fight(players: &mut [network::player::Player]) -> Result<(), Error> {
    let maze = maze::gen_walls(players);

    let mut width: f32 = 0.;
    let mut height: f32 = 0.;
    for p in players.iter() {
        width += p.physical_width;
        height = height.max(p.physical_height);
    }

    let mut bullets = vec::Vec::new();

    let mut internal_timer = time::Instant::now();

    let mut powerups = vec::Vec::new();
    let mut last_modifier_gen = time::Instant::now();

    for p in players.iter() {
        for _ in 0..3 {
            let x: f32 = ((rand::random::<f32>() * p.physical_width) as usize
                + maze::WALL_LENGTH / 2
                - ((rand::random::<f32>() * p.physical_width) as usize % maze::WALL_LENGTH))
                as f32;
            let y: f32 = ((rand::random::<f32>() * p.physical_height) as usize
                + maze::WALL_LENGTH / 2
                - ((rand::random::<f32>() * p.physical_height) as usize % maze::WALL_LENGTH))
                as f32;
            let powerup = (rand::random::<f32>() * powerup::POWERUP_COUNT as f32) as usize;

            powerups.push(powerup::PowerUp::new(
                powerup.into(),
                Vector2 {
                    x: x + p.top_left_x,
                    y: y + p.top_left_y,
                },
            ));
        }
    }

    let mut sprites = sprite::Sprite::create_sprites(players);
    for p in players.iter_mut() {
        let packed_maze = maze::pack_maze(p, &maze);
        p.send(&packed_maze)?;
        let data = sprite::Sprite::pack_sprites(&sprites);
        p.send(&data)?;
    }

    loop {
        for s in sprites.iter_mut() {
            s.update_sprite_status(&maze, &mut bullets, internal_timer);
        }

        update_bullet_status(&mut bullets, &maze, internal_timer);

        for s in sprites.iter_mut() {
            s.update_dead_status(&mut bullets);
            s.update_powerup_status(&mut powerups);
        }

        if last_modifier_gen.elapsed().as_secs() > 5 {
            last_modifier_gen = time::Instant::now();
            generate_new_modifiers(&mut powerups, width, height);
        }

        internal_timer = time::Instant::now();

        for p in players.iter_mut() {
            send_game_data(p, &bullets, &powerups, &sprites)?;
            recv_game_data(p, &mut sprites);
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}

fn update_bullet_status(
    bullets: &mut Vec<bullet::Bullet>,
    maze: &[maze::Wall],
    internal_timer: time::Instant,
) {
    let mut i = 0;
    while i < bullets.len() {
        let b = &mut bullets[i];
        b.pos.x += b.dir.x * internal_timer.elapsed().as_secs_f32() * 50.;
        b.pos.y += b.dir.y * internal_timer.elapsed().as_secs_f32() * 50.;
        if b.pos.x < 0. || b.pos.y < 0. || b.pos.x > 5000. || b.pos.y > 5000. {
            bullets.swap_remove(i);
        } else {
            i += 1;
            for w in maze.iter() {
                if w.intersect(&mut b.pos, BULLET_SIZE, BULLET_SIZE) {
                    i -= 1;
                    bullets.swap_remove(i);
                    break;
                }
            }
        }
    }
}

fn generate_new_modifiers(powerups: &mut Vec<powerup::PowerUp>, width: f32, height: f32) {
    let x: f32 = (rand::random::<f32>() * width).floor() + maze::WALL_LENGTH as f32 / 2.;
    let y: f32 = (rand::random::<f32>() * height).floor() + maze::WALL_LENGTH as f32 / 2.;
    let powerup = (rand::random::<f32>() * powerup::POWERUP_COUNT as f32) as usize;

    powerups.push(powerup::PowerUp::new(powerup.into(), Vector2 { x, y }));
}

fn send_game_data(
    p: &mut player::Player,
    bullets: &[bullet::Bullet],
    powerups: &[powerup::PowerUp],
    sprites: &[sprite::Sprite],
) -> Result<(), Error> {
    let mut data = vec::Vec::new();
    for s in sprites.iter() {
        if s.get_id() == p.rank as usize {
            data.push((s.get_life() as u8).to_be());
        }
    }

    data.append(&mut sprite::Sprite::pack_game_sprites(sprites, p));

    data.push((bullets.len() as u8).to_be());

    for bullet in bullets.iter() {
        let (x, y) = p.to_local_coordinates(bullet.pos.x, bullet.pos.y);

        let pos_x = x.to_be_bytes();
        let pos_y = y.to_be_bytes();

        data.append(&mut pos_x.to_vec());
        data.append(&mut pos_y.to_vec());

        let vx = p.to_local_proportion_horizontal(bullet.dir.x);
        let vy = p.to_local_proportion_vertical(bullet.dir.y);

        let speed_x = vx.to_be_bytes();
        let speed_y = vy.to_be_bytes();

        data.append(&mut speed_x.to_vec());
        data.append(&mut speed_y.to_vec());

        let _size = p.to_local_proportion_vertical(BULLET_SIZE as f32);

        let size = _size.to_be_bytes();

        data.append(&mut size.to_vec());
    }

    data.push((powerups.len() as u8).to_be());

    for powerup in powerups.iter() {
        let (x, y) = p.to_local_coordinates(powerup.pos().x, powerup.pos().y);

        let pos_x = x.to_be_bytes();
        let pos_y = y.to_be_bytes();

        data.append(&mut pos_x.to_vec());
        data.append(&mut pos_y.to_vec());
    }

    p.send(&data)
}

fn recv_game_data(p: &mut player::Player, sprites: &mut [sprite::Sprite]) {
    let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
    let mut anex = [0_u8; packet::MAX_DATA_SIZE];
    let n1 = p.recv(&mut anex).unwrap();
    buffer.copy_from_slice(&anex);
    let mut n = p.recv(&mut anex).unwrap();
    while n > 0 {
        buffer.copy_from_slice(&anex);
        n = p.recv(&mut anex).unwrap();
    }
    if n1 > 0 {
        for s in sprites.iter_mut() {
            if s.get_id() == p.rank as usize {
                let mut bb = [0_u8; 4];

                bb.copy_from_slice(&buffer[..4]);
                s.speed.x = f32::from_be_bytes(bb);
                bb.copy_from_slice(&buffer[4..8]);
                s.speed.y = f32::from_be_bytes(bb);

                let mut norm = s.speed.x * s.speed.x + s.speed.y * s.speed.y;
                norm = norm.sqrt();

                let mut speed_modifiers = 0.;
                for m in s.get_modifiers().iter() {
                    if m.get_type() == powerup::Type::SpeedDown
                        || m.get_type() == powerup::Type::SpeedUp
                    {
                        speed_modifiers += m.modifier();
                    }
                }

                let limit = (6. + speed_modifiers).max(1.);

                if norm > limit {
                    s.speed.x *= limit / norm;
                    s.speed.y *= limit / norm;
                }
            }
        }
    }
}
