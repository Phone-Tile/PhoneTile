use crate::ui::colors;
use raylib;
use std::ffi::{c_float, c_int};
use std::vec;
use Vector2;

//////////////////////////////////////////////
///
///
/// Consts
///
///
//////////////////////////////////////////////

const WALL_THICKNESS: c_float = 20.;

//////////////////////////////////////////////
///
///
/// Wall
///
///
//////////////////////////////////////////////

#[derive(Debug)]
pub struct Wall {
    pub start: raylib::Vector2,
    pub end: raylib::Vector2,
    pub color: raylib::Color,
}

impl Wall {
    //////////////////////////////////////////////
    ///
    ///
    /// Maze unpacking
    ///
    ///
    //////////////////////////////////////////////

    pub fn unpack_maze(_data: &[u8]) -> Vec<Wall> {
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

    //////////////////////////////////////////////
    ///
    ///
    /// Drawing
    ///
    ///
    //////////////////////////////////////////////

    pub unsafe fn draw(&self, tex_wall: raylib::Texture) {
        if self.start.x == self.end.x {
            raylib::DrawTexturePro(
                tex_wall,
                raylib::Rectangle {
                    x: 0.,
                    y: 0.,
                    width: 600.,
                    height: 600.,
                },
                raylib::Rectangle {
                    x: self.start.x - WALL_THICKNESS / 2.,
                    y: self.start.y,
                    width: WALL_THICKNESS,
                    height: self.end.y - self.start.y,
                },
                Vector2 { x: 0., y: 0. },
                0.,
                raylib::Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            );
        } else {
            raylib::DrawTexturePro(
                tex_wall,
                raylib::Rectangle {
                    x: 0.,
                    y: 0.,
                    width: 600.,
                    height: 600.,
                },
                raylib::Rectangle {
                    x: self.start.x,
                    y: self.start.y - WALL_THICKNESS / 2.,
                    width: self.end.x - self.start.x,
                    height: WALL_THICKNESS,
                },
                Vector2 { x: 0., y: 0. },
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

    //////////////////////////////////////////////
    ///
    ///
    /// Physics
    ///
    ///
    //////////////////////////////////////////////

    pub fn realign_sprite(&self, sprite: &mut Vector2, width: c_int, height: c_int) {
        // first check if this is pertinent to check for colisions
        let mut col = Vector2 {
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
        let mut normal = Vector2 {
            x: -(self.end.y - self.start.y),
            y: self.end.x - self.start.x,
        };
        norm = (normal.x * normal.x + normal.y * normal.y).sqrt();

        if norm == 0. {
            return;
        }

        normal.x /= norm;
        normal.y /= norm;

        let realign1 = self.correct_allong_axe(sprite, &col, width, height);
        let realign2 = self.correct_allong_axe(sprite, &normal, width, height);
        if realign1.abs() > realign2.abs() {
            sprite.x -= realign2 * normal.x;
            sprite.y -= realign2 * normal.y;
        } else {
            sprite.x -= realign1 * col.x;
            sprite.y -= realign1 * col.y;
        }
    }

    pub fn intersect(&self, sprite: &mut Vector2, width: c_int, height: c_int) -> bool {
        // first check if this is pertinent to check for colisions
        let mut col = Vector2 {
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
        let mut normal = Vector2 {
            x: -(self.end.y - self.start.y),
            y: self.end.x - self.start.x,
        };
        norm = (normal.x * normal.x + normal.y * normal.y).sqrt();

        if norm == 0. {
            return false;
        }

        normal.x /= norm;
        normal.y /= norm;

        let realign1 = self.correct_allong_axe(sprite, &col, width, height);
        let realign2 = self.correct_allong_axe(sprite, &normal, width, height);
        !(realign1.abs() == 0. || realign2.abs() == 0.)
    }

    fn correct_allong_axe(
        &self,
        sprite: &mut Vector2,
        axe: &Vector2,
        width: c_int,
        height: c_int,
    ) -> f32 {
        // project the cube
        let proj1 = axe.x * sprite.x + axe.y * sprite.y;
        let proj2 = axe.x * (sprite.x + width as f32) + axe.y * sprite.y;
        let proj3 = axe.x * sprite.x + axe.y * (sprite.y + height as f32);
        let proj4 = axe.x * (sprite.x + width as f32) + axe.y * (sprite.y + height as f32);

        let mut threashold_min = axe.x * self.end.x + axe.y * self.end.y;
        let mut threashold_max = axe.x * self.start.x + axe.y * self.start.y;

        if threashold_min > threashold_max {
            std::mem::swap(&mut threashold_min, &mut threashold_max);
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
