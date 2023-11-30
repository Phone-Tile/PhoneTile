use super::Vector2;
use crate::network::player;
use rand;
use std::vec;

//////////////////////////////////////////////
///
///
/// Wall
///
///
//////////////////////////////////////////////

#[derive(Debug)]
pub struct Wall {
    start: Vector2,
    end: Vector2,
}

impl Wall {
    pub fn realign_sprite(&self, sprite: &mut Vector2, size: usize) {
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

        let realign1 = self.correct_allong_axe(sprite, &col, size);
        let realign2 = self.correct_allong_axe(sprite, &normal, size);
        if realign1.abs() > realign2.abs() {
            sprite.x -= realign2 * normal.x;
            sprite.y -= realign2 * normal.y;
        } else {
            sprite.x -= realign1 * col.x;
            sprite.y -= realign1 * col.y;
        }
    }

    pub fn intersect(&self, sprite: &mut Vector2, size: usize) -> bool {
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

        let realign1 = self.correct_allong_axe(sprite, &col, size);
        let realign2 = self.correct_allong_axe(sprite, &normal, size);
        !(realign1.abs() == 0. || realign2.abs() == 0.)
    }

    fn correct_allong_axe(&self, sprite: &mut Vector2, axe: &Vector2, size: usize) -> f32 {
        // project the cube
        let proj1 = axe.x * sprite.x + axe.y * sprite.y;
        let proj2 = axe.x * (sprite.x + size as f32) + axe.y * sprite.y;
        let proj3 = axe.x * sprite.x + axe.y * (sprite.y + size as f32);
        let proj4 = axe.x * (sprite.x + size as f32) + axe.y * (sprite.y + size as f32);

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

//////////////////////////////////////////////
///
///
/// Maze generator
///
///
//////////////////////////////////////////////

pub fn gen_walls(players: &mut [player::Player]) -> Vec<Wall> {
    // for now build a map of the width the sum of width and heigh max(height)
    let mut width = 0.;
    let mut height = 0.;
    for p in players.iter() {
        width += p.physical_width;
        height = p.physical_height.max(height);
    }

    // we set the size to 10 milimeters for now

    let size = 150;
    let mut res = vec::Vec::new();

    for i in 0..(width as usize / size) {
        for j in 0..(height as usize / size) {
            let seed: f32 = rand::random();
            if seed < 0.25 {
                res.push(Wall {
                    start: Vector2 {
                        x: (i * size) as f32,
                        y: (j * size) as f32,
                    },
                    end: Vector2 {
                        x: ((i + 1) * size) as f32,
                        y: ((j) * size) as f32,
                    },
                });
            }
            if seed > 0.75 {
                res.push(Wall {
                    start: Vector2 {
                        x: (i * size) as f32,
                        y: (j * size) as f32,
                    },
                    end: Vector2 {
                        x: ((i) * size) as f32,
                        y: ((j + 1) * size) as f32,
                    },
                });
            }
        }
    }
    res
}

//////////////////////////////////////////////
///
///
/// Maze packer for send
///
///
//////////////////////////////////////////////

pub fn pack_maze(p: &mut crate::network::player::Player, maze: &Vec<Wall>) -> Vec<u8> {
    let mut res = vec::Vec::new();
    let tmp2 = (maze.len() as u16).to_be_bytes();
    res.append(&mut tmp2.to_vec());

    for w in maze.iter() {
        let (_start_x, _start_y) = p.to_local_coordinates(w.start.x, w.start.y);
        let (_end_x, _end_y) = p.to_local_coordinates(w.end.x, w.end.y);

        let start_x = _start_x.to_be_bytes();
        let start_y = _start_y.to_be_bytes();
        let end_x = _end_x.to_be_bytes();
        let end_y = _end_y.to_be_bytes();

        res.append(&mut start_x.to_vec());
        res.append(&mut start_y.to_vec());
        res.append(&mut end_x.to_vec());
        res.append(&mut end_y.to_vec());
    }

    res
}
