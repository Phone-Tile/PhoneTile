use crate::{
    network::{self, packet},
    ui::colors,
};
use c_char;
use raylib::{self, Color, Rectangle, Vector2};
use std::{
    convert::TryInto,
    ffi::{c_float, c_int},
    vec,
};
use time;

mod sprite;
mod wall;

//////////////////////////////////////////////
///
///
/// Entity
///
///
//////////////////////////////////////////////

struct Entity {
    pub pos: raylib::Vector2,
    pub dir: raylib::Vector2,
    pub size: c_int,
    pub color: Color,
}

//////////////////////////////////////////////
///
///
/// Entry point
///
///
//////////////////////////////////////////////

pub unsafe fn main_game(network: &mut network::Network) {
    let tex_tile = raylib::LoadTexture(raylib::raylib_str!("marble_tile.png"));
    let tex_wall = raylib::LoadTexture(raylib::raylib_str!("wall.png"));
    let tex_sprite = raylib::LoadTexture(raylib::raylib_str!("sprite.png"));

    let mut buffer = [0_u8; network::packet::MAX_DATA_SIZE];
    while network.recv(&mut buffer) == 0 {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let maze = wall::Wall::unpack_maze(&buffer);

    while network.recv(&mut buffer) == 0 {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let mut _sprites = sprite::Sprite::unpack_sprites(&buffer);

    let tile_size = maze[0].end.x - maze[0].start.x + maze[0].end.y - maze[0].start.y;

    let mut bullets: Vec<Entity> = vec::Vec::new();
    let mut powerups: Vec<Vector2> = vec::Vec::new();
    let mut life = 10;

    let mut record_central_pos = raylib::Vector2 { x: 0., y: 0. };
    let mut update_pos = raylib::Vector2 { x: 0., y: 0. };

    let mut internal_timer = time::Instant::now();

    loop {
        let mut buffer = [0_u8; packet::MAX_DATA_SIZE];
        let mut anex = [0_u8; packet::MAX_DATA_SIZE];
        let n1 = network.recv(&mut anex);
        let mut n = 0;
        loop {
            buffer.copy_from_slice(&anex);
            n = network.recv(&mut anex);
            if n == 0 {
                break;
            }
        }
        if n1 > 0 {
            (bullets, powerups, life) = unpack_game_data(&buffer, &mut _sprites);
        } else {
            for s in _sprites.iter_mut() {
                s.update_sprite_pos(internal_timer, &maze);
            }

            // update bullet status
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
                        if w.intersect(&mut b.pos, b.size, b.size) {
                            i -= 1;
                            bullets.swap_remove(i);
                            break;
                        }
                    }
                }
            }
        }
        internal_timer = time::Instant::now();

        if raylib::IsMouseButtonPressed(raylib::MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap()) {
            record_central_pos = raylib::GetMousePosition();
        }

        if raylib::IsMouseButtonDown(raylib::MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap()) {
            update_pos = raylib::GetMousePosition();
        }

        if raylib::IsMouseButtonReleased(raylib::MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap())
        {
            record_central_pos = raylib::Vector2 { x: 0., y: 0. };
            update_pos = raylib::Vector2 { x: 0., y: 0. };
        }

        let speed = raylib::Vector2 {
            x: (update_pos.x - record_central_pos.x) / 10.,
            y: (update_pos.y - record_central_pos.y) / 10.,
        };
        send_speed(network, speed);

        raylib::draw!({
            raylib::ClearBackground(raylib::Color {
                r: 65,
                g: 65,
                b: 65,
                a: 255,
            });

            for i in 0..(raylib::GetScreenWidth() as f32 / tile_size) as usize + 1 {
                for j in 0..(raylib::GetScreenHeight() as f32 / tile_size) as usize + 1 {
                    raylib::DrawTextureEx(
                        tex_tile,
                        raylib::Vector2 {
                            x: i as f32 * tile_size,
                            y: j as f32 * tile_size,
                        },
                        0.,
                        tile_size / 1024.,
                        raylib::Color {
                            r: 255,
                            g: 255,
                            b: 255,
                            a: 255,
                        },
                    );
                }
            }

            for w in maze.iter() {
                w.draw(tex_wall);
            }
            for b in bullets.iter() {
                raylib::DrawRectangle(b.pos.x as i32, b.pos.y as i32, b.size, b.size, b.color);
            }
            for p in powerups.iter() {
                raylib::DrawCircle(
                    p.x as c_int,
                    p.y as c_int,
                    20.,
                    raylib::Color {
                        r: 100,
                        g: 100,
                        b: 100,
                        a: 255,
                    },
                );
            }

            for s in _sprites.iter() {
                if s.is_alive() {
                    raylib::DrawTexturePro(
                        tex_sprite,
                        raylib::Rectangle {
                            x: ((s.get_skin() % 4) * 103 + s.get_state() * 32) as f32,
                            y: ((s.get_skin() / 4) * 155 + (s.get_direction() * 36)) as f32,
                            width: 30.,
                            height: 36.,
                        },
                        raylib::Rectangle {
                            x: s.get_pos().x,
                            y: s.get_pos().y,
                            width: s.get_size().0 as f32,
                            height: s.get_size().1 as f32,
                        },
                        raylib::Vector2 { x: 0., y: 0. },
                        0.,
                        raylib::Color {
                            r: 255,
                            g: 255,
                            b: 255,
                            a: 255,
                        },
                    );
                }
            }

            raylib::DrawText(
                raylib::raylib_str!(format!("{}", life)),
                50,
                50,
                100,
                raylib::Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            );
        });
    }
}

//////////////////////////////////////////////
///
///
/// Unpack game data
///
///
//////////////////////////////////////////////

fn unpack_game_data(
    _data: &[u8],
    sprites: &mut [sprite::Sprite],
) -> (Vec<Entity>, Vec<Vector2>, u8) {
    let life = u8::from_be(_data[0]);
    let mut data = &sprite::Sprite::unpack_game_sprites(sprites, &_data[1..])[..];
    let mut n = u8::from_be(data[0]) as usize;
    data = &data[1..];
    let mut bullets = vec::Vec::new();

    for i in 0..n {
        let mut buffer = [0_u8; 4];

        buffer.copy_from_slice(&data[i * 20..i * 20 + 4]);
        let pos_x = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 20 + 4..i * 20 + 8]);
        let pos_y = f32::from_be_bytes(buffer);

        buffer.copy_from_slice(&data[i * 20 + 8..i * 20 + 12]);
        let speed_x = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 20 + 12..i * 20 + 16]);
        let speed_y = f32::from_be_bytes(buffer);

        buffer.copy_from_slice(&data[i * 20 + 16..i * 20 + 20]);
        let size = f32::from_be_bytes(buffer);

        bullets.push(Entity {
            pos: Vector2 { x: pos_x, y: pos_y },
            dir: Vector2 {
                x: speed_x,
                y: speed_y,
            },
            size: size as c_int,
            color: raylib::Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        });
    }

    data = &data[n * 20..];
    n = u8::from_be(data[0]) as usize;
    data = &data[1..];
    let mut powerups = vec::Vec::new();

    for i in 0..n {
        let mut buffer = [0_u8; 4];

        buffer.copy_from_slice(&data[i * 8..i * 8 + 4]);
        let pos_x = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 8 + 4..i * 8 + 8]);
        let pos_y = f32::from_be_bytes(buffer);

        powerups.push(Vector2 { x: pos_x, y: pos_y });
    }

    (bullets, powerups, life)
}

//////////////////////////////////////////////
///
///
/// Sender
///
///
//////////////////////////////////////////////

fn send_speed(network: &mut network::Network, speed: Vector2) {
    let mut data = [0_u8; 8];

    let mut buffer = speed.x.to_be_bytes();
    data[..4].copy_from_slice(&buffer);
    buffer = speed.y.to_be_bytes();
    data[4..].copy_from_slice(&buffer);

    network.send(&data).unwrap();
}
