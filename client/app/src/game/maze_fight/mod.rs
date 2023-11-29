use crate::network;
use raylib;
use std::{convert::TryInto, ffi::{c_int, c_float}, vec};
use std::cmp;

const SPRITE_SIZE: c_int = 80;
const WALL_THICKNESS: c_float = 10.;

struct Wall {
    pub start: raylib::Vector2,
    pub end: raylib::Vector2,
    pub color: raylib::Color,
}

impl Wall {
    pub fn draw(&self) {
        unsafe {
            raylib::DrawLineEx(self.start, self.end, WALL_THICKNESS, self.color)
        }
    }

    pub fn realign_sprite(&self, sprite: &mut raylib::Vector2) {
        // first check if this is pertinent to check for colisions
        let mut col = raylib::Vector2 {x: self.end.x - self.start.x , y: self.end.y - self.start.y };
        let mut norm = (col.x*col.x + col.y*col.y).sqrt();

        if norm == 0. {
            return;
        }

        col.x /= norm;
        col.y /= norm;

        // the idea is to project the square on a vector perpendicular to the wall and look if the point projection of the line is in the projection of the cube
        let mut normal = raylib::Vector2 {x: - (self.end.y - self.start.y) , y: self.end.x - self.start.x };
        norm = (normal.x*normal.x + normal.y*normal.y).sqrt();

        if norm == 0. {
            return;
        }

        normal.x /= norm;
        normal.y /= norm;


        // if (self.check_might_intersect_along_axe(sprite, col) && self.check_might_intersect_along_axe(sprite,normal)) {
        //     self.correct_allong_axe(sprite, col);
        //     self.correct_allong_axe(sprite, normal);
        // }

        let realign1 = self.correct_allong_axe(sprite, col);
        let realign2 = self.correct_allong_axe(sprite, normal);
        if realign1.abs() > realign2.abs() {
            sprite.x -= realign2 * normal.x;
            sprite.y -= realign2 * normal.y;
        } else {
            sprite.x -= realign1 * col.x;
            sprite.y -= realign1 * col.y;
        }

    }

    fn check_might_intersect_along_axe(&self, sprite: &mut raylib::Vector2, axe: raylib::Vector2) -> bool {
        // project the cube
        let proj1 = axe.x*sprite.x + axe.y*sprite.y;
        let proj2 = axe.x*(sprite.x+SPRITE_SIZE as c_float) + axe.y*sprite.y;
        let proj3 = axe.x*sprite.x + axe.y*(sprite.y+SPRITE_SIZE as c_float);
        let proj4 = axe.x*(sprite.x+SPRITE_SIZE as c_float) + axe.y*(sprite.y+SPRITE_SIZE as c_float);

        let threashold = axe.x*self.end.x + axe.y*self.end.y;

        let max = Self::fuck_max( Self::fuck_max(proj1, proj2), Self::fuck_max(proj3, proj4) );
        let min = Self::fuck_min( Self::fuck_min(proj1, proj2), Self::fuck_min(proj3, proj4) );

        if max > threashold && min < threashold {
            true
        } else {
            false
        }
    }

    fn correct_allong_axe(&self, sprite: &mut raylib::Vector2, axe: raylib::Vector2) -> f32 {
        // project the cube
        let proj1 = axe.x*sprite.x + axe.y*sprite.y;
        let proj2 = axe.x*(sprite.x+SPRITE_SIZE as c_float) + axe.y*sprite.y;
        let proj3 = axe.x*sprite.x + axe.y*(sprite.y+SPRITE_SIZE as c_float);
        let proj4 = axe.x*(sprite.x+SPRITE_SIZE as c_float) + axe.y*(sprite.y+SPRITE_SIZE as c_float);

        let mut threashold_min = axe.x*self.end.x + axe.y*self.end.y;
        let mut threashold_max = axe.x*self.start.x + axe.y*self.start.y;

        if threashold_min > threashold_max {
            let tmp = threashold_min;
            threashold_min = threashold_max;
            threashold_max = tmp;
        }

        let max = Self::fuck_max( Self::fuck_max(proj1, proj2), Self::fuck_max(proj3, proj4) );
        let min = Self::fuck_min( Self::fuck_min(proj1, proj2), Self::fuck_min(proj3, proj4) );

        if max > threashold_min && min < threashold_max {
            let realign = if (threashold_max-min > max-threashold_min) {max-threashold_min} else {min-threashold_max};
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

pub fn maze_fight() {
    unsafe {
        let screen_height = raylib::GetScreenHeight();
        let screen_width = raylib::GetScreenWidth();

        let squarre_size = screen_width/10;

        let mut record_central_pos = raylib::Vector2 { x: 0., y: 0. };
        let mut sprite_pos = raylib::Vector2 { x: 0., y:0. };
        let mut update_pos = raylib::Vector2 { x: 0., y:0. };

        raylib::SetTargetFPS(60);

        let w = Wall {
            start: raylib::Vector2 { x: 100., y: 100. },
            end: raylib::Vector2 { x: 100., y: screen_height as f32 - 100. },
            color: raylib::Color { r: 0, g: 0, b: 0, a: 255 },
        };

        loop {
            if raylib::IsMouseButtonPressed(raylib::MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap()) {
                record_central_pos = raylib::GetMousePosition();
            }

            if raylib::IsMouseButtonDown(raylib::MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap()) {
                update_pos = raylib::GetMousePosition();
            }

            if raylib::IsMouseButtonReleased(raylib::MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap()) {
                record_central_pos = raylib::Vector2 { x: 0., y:0. };
                update_pos = raylib::Vector2 { x: 0., y:0. };
            }

            let mut speed = raylib::Vector2 { x: update_pos.x - record_central_pos.x, y: update_pos.y - record_central_pos.y };
            let mut norm = speed.x*speed.x + speed.y*speed.y;
            norm = norm.sqrt();
            if norm > 20. {
                norm /= 20.;
                speed.x /= norm;
                speed.y /= norm;
            }

            sprite_pos.x += speed.x;
            sprite_pos.y += speed.y;

            w.realign_sprite(&mut sprite_pos);

            raylib::draw!({
                raylib::ClearBackground(raylib::Color {
                    r: 65,
                    g: 65,
                    b: 65,
                    a: 255,
                });
    
                let start = raylib::Vector2 { x: 100., y: 0. };
                let end = raylib::Vector2 { x: 100., y: screen_height as f32 };
                w.draw();
                // raylib::DrawLineEx(start, end, 300., raylib::Color { r: 0, g: 0, b: 0, a: 255 });

                raylib::DrawRectangle(sprite_pos.x as i32, sprite_pos.y as i32, SPRITE_SIZE, SPRITE_SIZE, raylib::Color { r: 0, g: 0, b: 0, a: 255 });
            });
        }
    }
}