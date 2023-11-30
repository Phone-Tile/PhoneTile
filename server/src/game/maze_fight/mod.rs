use crate::network::packet;
use crate::network::{self, player};
use std::io::Error;
use std::time;
use std::vec;
use std::thread;

//////////////////////////////////////////////
///
///
/// Modules
///
///
//////////////////////////////////////////////
mod bullet;
mod maze;
mod weapon;

//////////////////////////////////////////////
///
///
/// Constants
///
///
//////////////////////////////////////////////

const SPRITE_SIZE: usize = 80;
const BULLET_SIZE: usize = 20;

//////////////////////////////////////////////
///
///
/// Vector
///
///
//////////////////////////////////////////////

#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

//////////////////////////////////////////////
///
///
/// Local Player structure
///
///
//////////////////////////////////////////////

struct LocalPlayer {
    pos: Vector2,
    speed: Vector2,
    id: usize,
    timer: time::Instant,
    color: Color,
    is_dead: bool,
}

//////////////////////////////////////////////
///
///
/// Color structure
///
///
//////////////////////////////////////////////

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
};

const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

const GREEN: Color = Color {
    r: 80,
    g: 240,
    b: 0,
};

const BLUE: Color = Color {
    r: 0,
    g: 55,
    b: 255,
};

const PURPLE: Color = Color {
    r: 164,
    g: 55,
    b: 255,
};

const PINK: Color = Color {
    r: 242,
    g: 159,
    b: 255,
};

const RED: Color = Color {
    r: 250,
    g: 0,
    b: 23,
};

const YELLOW: Color = Color {
    r: 255,
    g: 184,
    b: 0,
};

const ORANGE: Color = Color {
    r: 255,
    g: 111,
    b: 20,
};

const COLOR_LIST: [Color; 8] = [BLACK, WHITE, BLUE, GREEN, YELLOW, PURPLE, ORANGE, PINK];


//////////////////////////////////////////////
///
///
/// Entry point
///
///
//////////////////////////////////////////////

pub fn maze_fight(players: &mut [network::player::Player]) -> Result<(), Error> {
    let maze = maze::gen_walls(players);

    let mut players_system = vec::Vec::new();
    let mut i = 0;
    for p in players.iter_mut() {
        let packed_maze = maze::pack_maze(p, &maze);
        p.send(&packed_maze)?;
        players_system.push(LocalPlayer {
            pos: Vector2 {
                x: p.physical_width / 2. + p.top_left_x,
                y: p.physical_height / 2. + p.top_left_y,
            },
            speed: Vector2 { x: 0., y: 0. },
            id: p.rank as usize,
            timer: time::Instant::now(),
            color: COLOR_LIST[i],
            is_dead: false,
        });
        i += 1;
    }

    let mut bullets = vec::Vec::new();

    let mut internal_timer = time::Instant::now();

    loop {
        // send position of players, bullets and other ...

        // receive new speed ...

        // update game status

        // update player status
        for p in players_system.iter_mut() {
            if !p.is_dead {
                let mut norm = p.speed.x * p.speed.x + p.speed.y * p.speed.y;
                norm = norm.sqrt();
                if p.timer.elapsed().as_millis() > weapon::FIRERING_SPEED && norm > 0. {
                    p.timer = std::time::Instant::now();
                    bullets.push(bullet::Bullet::new(
                        Vector2 {
                            x: p.pos.x + (SPRITE_SIZE / 2) as f32,
                            y: p.pos.y + (SPRITE_SIZE / 2) as f32,
                        },
                        Vector2 {
                            x: bullet::BULLET_SPEED * p.speed.x / norm,
                            y: bullet::BULLET_SPEED * p.speed.y / norm,
                        },
                        p.id,
                    ));
                }
    
                p.pos.x += p.speed.x * internal_timer.elapsed().as_secs_f32()*50.;
                p.pos.y += p.speed.y * internal_timer.elapsed().as_secs_f32()*50.;
    
                for w in maze.iter() {
                    w.realign_sprite(&mut p.pos, SPRITE_SIZE);
                }
            }
        }

        // update bullet status
        i = 0;
        while i < bullets.len() {
            let b = &mut bullets[i];
            b.pos.x += b.dir.x * internal_timer.elapsed().as_secs_f32()*50.;
            b.pos.y += b.dir.y * internal_timer.elapsed().as_secs_f32()*50.;
            if b.pos.x < 0. || b.pos.y < 0. || b.pos.x > 5000. || b.pos.y > 5000. {
                bullets.swap_remove(i);
            } else {
                i += 1;
                for w in maze.iter() {
                    if w.intersect(&mut b.pos, BULLET_SIZE) {
                        i -= 1;
                        bullets.swap_remove(i);
                        break;
                    }
                }
            }
        }

        // update dead status
        for p in players_system.iter_mut() {
            for b in bullets.iter() {
                if b.id != p.id && b.pos.x > p.pos.x && b.pos.x < p.pos.x+SPRITE_SIZE as f32 && b.pos.y > p.pos.y && b.pos.y < p.pos.y+SPRITE_SIZE as f32 {
                    p.is_dead = true;
                }
            }
        }

        internal_timer = time::Instant::now();

        for p in players.iter_mut() {
            send_game_data(p, &players_system, &bullets)?;
            recv_game_data(p, &mut players_system);
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}

fn send_game_data(
    p: &mut player::Player,
    players: &Vec<LocalPlayer>,
    bullets: &Vec<bullet::Bullet>,
) -> Result<(), Error> {
    let mut data = vec::Vec::new();
    let mut players_playing = 0;
    for p in players.iter() {
        if !p.is_dead {
            players_playing += 1;
        }
    }
    data.push(players_playing as u8);

    for player in players.iter() {
        if !player.is_dead {
            let (x, y) = p.to_local_coordinates(player.pos.x, player.pos.y);

            let pos_x = x.to_be_bytes();
            let pos_y = y.to_be_bytes();
    
            data.append(&mut pos_x.to_vec());
            data.append(&mut pos_y.to_vec());
    
            let vx = p.to_local_proportion(player.speed.x);
            let vy = p.to_local_proportion(player.speed.y);
    
            let speed_x = vx.to_be_bytes();
            let speed_y = vy.to_be_bytes();
    
            data.append(&mut speed_x.to_vec());
            data.append(&mut speed_y.to_vec());
    
            let _size = p.to_local_proportion(SPRITE_SIZE as f32);
    
            let size = _size.to_be_bytes();
    
            data.append(&mut size.to_vec());
    
            data.push(player.color.r.to_be());
            data.push(player.color.g.to_be());
            data.push(player.color.b.to_be());
        }
    }

    data.push(bullets.len() as u8);

    for bullet in bullets.iter() {
        let (x, y) = p.to_local_coordinates(bullet.pos.x, bullet.pos.y);

        let pos_x = x.to_be_bytes();
        let pos_y = y.to_be_bytes();

        data.append(&mut pos_x.to_vec());
        data.append(&mut pos_y.to_vec());

        let vx = p.to_local_proportion(bullet.dir.x);
        let vy = p.to_local_proportion(bullet.dir.y);

        let speed_x = vx.to_be_bytes();
        let speed_y = vy.to_be_bytes();

        data.append(&mut speed_x.to_vec());
        data.append(&mut speed_y.to_vec());

        let _size = p.to_local_proportion(BULLET_SIZE as f32);

        let size = _size.to_be_bytes();

        data.append(&mut size.to_vec());
    }

    p.send(&data)
}

fn recv_game_data(p: &mut player::Player,players: &mut Vec<LocalPlayer>,) {
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
        for pp in players.iter_mut() {
            if pp.id == p.rank as usize {
                let mut bb = [0_u8; 4];

                bb.copy_from_slice(&buffer[..4]);
                pp.speed.x = f32::from_be_bytes(bb);
                bb.copy_from_slice(&buffer[4..8]);
                pp.speed.y = f32::from_be_bytes(bb);

                let mut norm = pp.speed.x * pp.speed.x + pp.speed.y * pp.speed.y;
                norm = norm.sqrt();

                if norm > 10. {
                    pp.speed.x *= 10. / norm;
                    pp.speed.y *= 10. / norm;
                }
            }
        }
    }
}