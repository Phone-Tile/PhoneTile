use crate::{
    network::{self, packet},
    ui::colors,
};
use c_char;
use raylib::{self, Color, Vector2};
use std::{
    convert::TryInto,
    ffi::{c_float, c_int},
    vec,
};
use time;

//////////////////////////////////////////////
///
///
/// Consts
///
///
//////////////////////////////////////////////

const WALL_THICKNESS: c_float = 10.;

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
/// Wall
///
///
//////////////////////////////////////////////

#[derive(Debug)]
struct Wall {
    pub start: raylib::Vector2,
    pub end: raylib::Vector2,
    pub color: raylib::Color,
}

impl Wall {
    pub fn draw(&self) {
        unsafe { raylib::DrawLineEx(self.start, self.end, WALL_THICKNESS, self.color) }
    }

    pub fn realign_sprite(&self, sprite: &mut raylib::Vector2, size: c_int) {
        // first check if this is pertinent to check for colisions
        let mut col = raylib::Vector2 {
            x: self.end.x - self.start.x,
            y: self.end.y - self.start.y,
        };
        let mut norm = (col.x * col.x + col.y * col.y).sqrt();

        if norm == 0. {
            return;
        }

        col.x /= norm;
        col.y /= norm;

        // the idea is to project the square on a vector perpendicular to the wall and look if the point projection of the line is in the projection of the cube
        let mut normal = raylib::Vector2 {
            x: -(self.end.y - self.start.y),
            y: self.end.x - self.start.x,
        };
        norm = (normal.x * normal.x + normal.y * normal.y).sqrt();

        if norm == 0. {
            return;
        }

        normal.x /= norm;
        normal.y /= norm;

        let realign1 = self.correct_allong_axe(sprite, col, size);
        let realign2 = self.correct_allong_axe(sprite, normal, size);
        if realign1.abs() > realign2.abs() {
            sprite.x -= realign2 * normal.x;
            sprite.y -= realign2 * normal.y;
        } else {
            sprite.x -= realign1 * col.x;
            sprite.y -= realign1 * col.y;
        }
    }

    pub fn intersect(&self, sprite: &mut raylib::Vector2, size: c_int) -> bool {
        // first check if this is pertinent to check for colisions
        let mut col = raylib::Vector2 {
            x: self.end.x - self.start.x,
            y: self.end.y - self.start.y,
        };
        let mut norm = (col.x * col.x + col.y * col.y).sqrt();

        if norm == 0. {
            return false;
        }

        col.x /= norm;
        col.y /= norm;

        // the idea is to project the square on a vector perpendicular to the wall and look if the point projection of the line is in the projection of the cube
        let mut normal = raylib::Vector2 {
            x: -(self.end.y - self.start.y),
            y: self.end.x - self.start.x,
        };
        norm = (normal.x * normal.x + normal.y * normal.y).sqrt();

        if norm == 0. {
            return false;
        }

        normal.x /= norm;
        normal.y /= norm;

        let realign1 = self.correct_allong_axe(sprite, col, size);
        let realign2 = self.correct_allong_axe(sprite, normal, size);
        if realign1.abs() == 0. || realign2.abs() == 0. {
            false
        } else {
            true
        }
    }

    fn correct_allong_axe(
        &self,
        sprite: &mut raylib::Vector2,
        axe: raylib::Vector2,
        size: c_int,
    ) -> f32 {
        // project the cube
        let proj1 = axe.x * sprite.x + axe.y * sprite.y;
        let proj2 = axe.x * (sprite.x + size as c_float) + axe.y * sprite.y;
        let proj3 = axe.x * sprite.x + axe.y * (sprite.y + size as c_float);
        let proj4 = axe.x * (sprite.x + size as c_float) + axe.y * (sprite.y + size as c_float);

        let mut threashold_min = axe.x * self.end.x + axe.y * self.end.y;
        let mut threashold_max = axe.x * self.start.x + axe.y * self.start.y;

        if threashold_min > threashold_max {
            let tmp = threashold_min;
            threashold_min = threashold_max;
            threashold_max = tmp;
        }

        let max = Self::fuck_max(Self::fuck_max(proj1, proj2), Self::fuck_max(proj3, proj4));
        let min = Self::fuck_min(Self::fuck_min(proj1, proj2), Self::fuck_min(proj3, proj4));

        if max > threashold_min && min < threashold_max {
            let realign = if threashold_max - min > max - threashold_min {
                max - threashold_min
            } else {
                min - threashold_max
            };
            return realign;
        }
        0.
    }

    fn fuck_max(x: f32, y: f32) -> f32 {
        if x > y {
            x
        } else {
            y
        }
    }

    fn fuck_min(x: f32, y: f32) -> f32 {
        if x > y {
            y
        } else {
            x
        }
    }
}

//////////////////////////////////////////////
///
///
/// Entry point
///
///
//////////////////////////////////////////////

pub unsafe fn main_game(network: &mut network::Network) {
    let mut buffer = [0_u8; network::packet::MAX_DATA_SIZE];
    while network.recv(&mut buffer) == 0 {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let maze = unpack_maze(&buffer);

    let mut sprites: Vec<Entity> = vec::Vec::new();
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
            (sprites, bullets, powerups, life) = unpack_game_data(&buffer);
        } else {
            for p in sprites.iter_mut() {
                p.pos.x += p.dir.x * internal_timer.elapsed().as_secs_f32() * 50.;
                p.pos.y += p.dir.y * internal_timer.elapsed().as_secs_f32() * 50.;

                for w in maze.iter() {
                    w.realign_sprite(&mut p.pos, p.size);
                }
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
                        if w.intersect(&mut b.pos, b.size) {
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
            for w in maze.iter() {
                w.draw();
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
                        r: 200,
                        g: 200,
                        b: 200,
                        a: 255,
                    },
                );
            }
            for s in sprites.iter() {
                raylib::DrawRectangle(s.pos.x as i32, s.pos.y as i32, s.size, s.size, s.color);
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
/// Maze unpacking
///
///
//////////////////////////////////////////////

fn unpack_maze(_data: &[u8]) -> Vec<Wall> {
    let mut buffer = [0_u8; 2];
    buffer.copy_from_slice(&_data[..2]);
    let n = u16::from_be_bytes(buffer) as usize;
    let data = &_data[2..];
    let mut res = vec::Vec::new();
    for i in 0..n {
        let mut buffer = [0_u8; 4];

        buffer.copy_from_slice(&data[i * 16..i * 16 + 4]);
        let start_x = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 16 + 4..i * 16 + 8]);
        let start_y = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 16 + 8..i * 16 + 12]);
        let end_x = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 16 + 12..i * 16 + 16]);
        let end_y = f32::from_be_bytes(buffer);
        res.push(Wall {
            start: Vector2 {
                x: start_x,
                y: start_y,
            },
            end: Vector2 { x: end_x, y: end_y },
            color: colors::BLACK,
        });
    }
    res
}

fn unpack_game_data(_data: &[u8]) -> (Vec<Entity>, Vec<Entity>, Vec<Vector2>, u8) {
    let life = u8::from_be(_data[0]);
    let mut n = u8::from_be(_data[1]) as usize;
    let mut data = &_data[2..];
    let mut sprites = vec::Vec::new();

    for i in 0..n {
        let mut buffer = [0_u8; 4];

        buffer.copy_from_slice(&data[i * 23..i * 23 + 4]);
        let pos_x = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 23 + 4..i * 23 + 8]);
        let pos_y = f32::from_be_bytes(buffer);

        buffer.copy_from_slice(&data[i * 23 + 8..i * 23 + 12]);
        let speed_x = f32::from_be_bytes(buffer);
        buffer.copy_from_slice(&data[i * 23 + 12..i * 23 + 16]);
        let speed_y = f32::from_be_bytes(buffer);

        buffer.copy_from_slice(&data[i * 23 + 16..i * 23 + 20]);
        let size = f32::from_be_bytes(buffer);

        sprites.push(Entity {
            pos: Vector2 { x: pos_x, y: pos_y },
            dir: Vector2 {
                x: speed_x,
                y: speed_y,
            },
            size: size as c_int,
            color: Color {
                r: u8::from_be(data[i * 23 + 20]),
                g: u8::from_be(data[i * 23 + 21]),
                b: u8::from_be(data[i * 23 + 22]),
                a: 255,
            },
        });
    }

    data = &data[n * 23..];
    n = u8::from_be(data[0]) as usize;
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

    (sprites, bullets, powerups, life)
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
