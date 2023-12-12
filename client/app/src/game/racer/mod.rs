use crate::network;
use std::convert::TryInto;
use std::ffi::{c_float, c_int};
use crate::network::packet;
use std::ffi::CString;

use c_char;

use raylib::{
    ClearBackground, Color, DrawCircle, DrawFPS, DrawRectanglePro, DrawSplineSegmentBezierCubic,
    DrawSplineSegmentLinear, DrawText, GetScreenHeight, GetScreenWidth, GetSplinePointBezierCubic,
    GetSplinePointLinear, IsMouseButtonDown, MouseButton_MOUSE_BUTTON_LEFT, Rectangle,
    SetTargetFPS, Vector2, WindowShouldClose,
};

use raylib::{draw, raylib_str};

struct Car<'a> {
    pos: Vector2,
    track: &'a Track,
    track_segment: u8,
    track_t: c_float,
    speed: c_float,
    speed_max: c_float,
    accelertion: c_float,
    decceleration: c_float,
    rotation: c_float,
    rectangle: Rectangle,
    color: Color,
}

impl<'a> Car<'a> {
    fn new(track: &'a Track) -> Car {
        Car {
            pos: track.get_pos(0, 0.5),
            track,
            track_segment: 0,
            track_t: 0.5,
            speed: 0.0,
            speed_max: 0.05,
            accelertion: 0.001,
            decceleration: 0.01,
            rotation: 1.0,
            rectangle: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 200.0,
            },
            color: Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        }
    }

    fn accelerate(&mut self) {
        self.speed = self.speed_max.min(self.speed + self.accelertion);
    }

    fn decelerate(&mut self) {
        self.speed = 0.0_f32.max(self.speed - self.decceleration);
    }

    fn new_pos(&mut self) {
        if self.speed >= 0.0 {
            self.track_t += self.speed;
            if self.track_t >= 1.0 {
                self.track_t = self.track_t - 1.0;
                self.track_segment = (self.track_segment + 1) % 4;
            }
            let new_pos = self.track.get_pos(self.track_segment, self.track_t);
            let old_pos = self.pos;
            let (w_x, w_y) = (new_pos.x - old_pos.x, new_pos.y - old_pos.y);
            let (v_x, v_y) = (0.0, -1.0);
            self.rotation = (w_y * v_x - w_x * v_y)
                .atan2(w_x * v_x + w_y * v_y)
                .to_degrees();
            self.pos = new_pos;
        }
    }

    unsafe fn draw(&self) {
        DrawRectanglePro(
            Rectangle {
                x: self.pos.x,
                y: self.pos.y,
                width: self.rectangle.width,
                height: self.rectangle.height,
            },
            Vector2 {
                x: self.rectangle.width / 2.0,
                y: self.rectangle.height / 2.0,
            },
            self.rotation as f32,
            self.color,
        );
    }
}

struct Track {
    p1: (c_int, c_int),
    p2: (c_int, c_int),
    p3: (c_int, c_int),
    p4: (c_int, c_int),
    width: c_int,
    height: c_int,
    thick: c_float,
    color: Color,
}

impl Track {
    fn new(width: c_int, height: c_int) -> Track {
        let x1 = width / 9;
        let x2 = width - width / 9;
        let y1 = height / 3;
        let y2 = height - height / 3;

        Track {
            p1: (x1, y1),
            p2: (x1, y2),
            p3: (x2, y2),
            p4: (x2, y1),
            width,
            height,
            thick: 10.0,
            color: Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255,
            },
        }
    }

    unsafe fn draw(&self) {
        DrawCircle(self.p1.0, self.p1.1, 10.0, self.color);
        DrawCircle(self.p2.0, self.p2.1, 10.0, self.color);
        DrawCircle(self.p3.0, self.p3.1, 10.0, self.color);
        DrawCircle(self.p4.0, self.p4.1, 10.0, self.color);

        let p1 = Vector2 {
            x: self.p1.0 as c_float,
            y: self.p1.1 as c_float,
        };
        let p2 = Vector2 {
            x: self.p2.0 as c_float,
            y: self.p2.1 as c_float,
        };
        let p3 = Vector2 {
            x: self.p3.0 as c_float,
            y: self.p3.1 as c_float,
        };
        let p4 = Vector2 {
            x: self.p4.0 as c_float,
            y: self.p4.1 as c_float,
        };

        DrawSplineSegmentBezierCubic(
            p1,
            Vector2 {
                x: p1.x,
                y: p1.y - (self.height / 5) as f32,
            },
            Vector2 {
                x: p4.x,
                y: p4.y - (self.height / 5) as f32,
            },
            p4,
            self.thick,
            self.color,
        );
        DrawSplineSegmentBezierCubic(
            p2,
            Vector2 {
                x: p2.x,
                y: p2.y + (self.height / 5) as f32,
            },
            Vector2 {
                x: p3.x,
                y: p3.y + (self.height / 5) as f32,
            },
            p3,
            self.thick,
            self.color,
        );
        DrawSplineSegmentLinear(p1, p2, self.thick, self.color);
        DrawSplineSegmentLinear(p4, p3, self.thick, self.color);
    }

    fn get_pos(&self, segment: u8, t: c_float) -> Vector2 {
        let p1 = Vector2 {
            x: self.p1.0 as c_float,
            y: self.p1.1 as c_float,
        };
        let p2 = Vector2 {
            x: self.p2.0 as c_float,
            y: self.p2.1 as c_float,
        };
        let p3 = Vector2 {
            x: self.p3.0 as c_float,
            y: self.p3.1 as c_float,
        };
        let p4 = Vector2 {
            x: self.p4.0 as c_float,
            y: self.p4.1 as c_float,
        };
        unsafe {
            match segment {
                0 => GetSplinePointLinear(p1, p2, t),
                1 => GetSplinePointBezierCubic(
                    p2,
                    Vector2 {
                        x: p2.x,
                        y: p2.y + (self.height / 5) as f32,
                    },
                    Vector2 {
                        x: p3.x,
                        y: p3.y + (self.height / 5) as f32,
                    },
                    p3,
                    t,
                ),
                2 => GetSplinePointLinear(p4, p3, 1.0 - t),
                3 => GetSplinePointBezierCubic(
                    p1,
                    Vector2 {
                        x: p1.x,
                        y: p1.y - (self.height / 5) as f32,
                    },
                    Vector2 {
                        x: p4.x,
                        y: p4.y - (self.height / 5) as f32,
                    },
                    p4,
                    1.0 - t,
                ),
                _ => panic!("never append"),
            }
        }
    }
}

unsafe fn send_data(network: &mut network::Network) {
    let buffer = [IsMouseButtonDown(
        MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap(),
    ) as u8];
    network.send(&buffer);
}

fn recv_data(
    network: &mut network::Network,
    buffer_cars: &mut Vec<(f64, f64)>,
    buffer_bezier: &mut Vec<(f64, f64)>,
    buffer_directions: &mut Vec<(f64, f64)>,
) {
    let mut update_data = [0_u8; packet::MAX_DATA_SIZE];
    let mut new_data = update_data.clone();
    let mut flushing = network.recv(&mut update_data);
    let mut N = 0;
    while flushing > 0 {
        new_data = update_data.clone();
        N = flushing;
        flushing = network.recv(&mut update_data);
    }
    if N > 0 {
        while buffer_cars.len() < (new_data[0]/16).into() {
            buffer_cars.push((0., 0.));
            buffer_directions.push((0., 0.))
        }
        while buffer_bezier.len() < ((N - new_data[0] as usize) / 16) {
            buffer_bezier.push((0., 0.))
        }
        let cars = &new_data[1..(new_data[0]+1).into()];
        let bezier = &new_data[(new_data[0]+1).into()..N];
        for car_idx in 0..(cars.len() / 16) {
            let mut temp_cars = [0_u8; 8];
            temp_cars.copy_from_slice(&cars[(32 * car_idx)..(32 * car_idx + 8)]);
            let x = f64::from_be_bytes(temp_cars);
            temp_cars.copy_from_slice(&cars[(32 * car_idx + 8)..(32 * car_idx + 16)]);
            let y = f64::from_be_bytes(temp_cars);
            buffer_cars[car_idx] = (x, y);

            let mut temp_dir = [0_u8; 8];
            temp_dir.copy_from_slice(&cars[(32 * car_idx + 16)..(32 * car_idx + 24)]);
            let dir1 = f64::from_be_bytes(temp_dir);
            temp_dir.copy_from_slice(&cars[(32 * car_idx + 24)..(32 * car_idx + 32)]);
            let dir2 = f64::from_be_bytes(temp_dir);
            buffer_directions[car_idx] = (dir1, dir2);
        }
        for bezier_idx in 0..(bezier.len() / 64) {
            let mut temp_cars = [0_u8; 8];
            temp_cars.copy_from_slice(&bezier[(64 * bezier_idx)..(64 * bezier_idx+ 8)]);
            let p1x = f64::from_be_bytes(temp_cars);
            temp_cars.copy_from_slice(&bezier[(64 * bezier_idx + 8)..(64 * bezier_idx+ 16)]);
            let p1y = f64::from_be_bytes(temp_cars);
            temp_cars.copy_from_slice(&bezier[(64 * bezier_idx + 16)..(64 * bezier_idx + 24)]);
            let p2x = f64::from_be_bytes(temp_cars);
            temp_cars.copy_from_slice(&bezier[(64 * bezier_idx + 24)..(64 * bezier_idx + 32)]);
            let p2y = f64::from_be_bytes(temp_cars);
            temp_cars.copy_from_slice(&bezier[(64 * bezier_idx + 32)..(64 * bezier_idx + 40)]);
            let p3x = f64::from_be_bytes(temp_cars);
            temp_cars.copy_from_slice(&bezier[(64 * bezier_idx + 40)..(64 * bezier_idx + 48)]);
            let p3y = f64::from_be_bytes(temp_cars);
            temp_cars.copy_from_slice(&bezier[(64 * bezier_idx + 48)..(64 * bezier_idx + 56)]);
            let p4x = f64::from_be_bytes(temp_cars);
            temp_cars.copy_from_slice(&bezier[(64 * bezier_idx + 56)..(64 * bezier_idx + 64)]);
            let p4y = f64::from_be_bytes(temp_cars);
            buffer_bezier[bezier_idx*4] = (p1x, p1y);
            buffer_bezier[bezier_idx*4 + 1] = (p2x, p2y);
            buffer_bezier[bezier_idx*4 + 2] = (p3x, p3y);
            buffer_bezier[bezier_idx*4 + 3] = (p4x, p4y);
        }
    }
}

unsafe fn draw_cars(car: (f64, f64), dir: (f64, f64)) {
    let theta = if (dir.1 != 0.){
        let rapport = dir.0/dir.1;
        rapport.atan()
    }
    else{
        if (dir.0 > 0.){
            std::f64::consts::PI/2.
        }
        else {
            -std::f64::consts::PI/2.
        }
    };
    let rec = Rectangle{x:car.0 as f32, y:car.1 as f32, width:40., height:60.};
    DrawRectanglePro(
        rec,
        Vector2{x: rec.x - rec.width/2., y: rec.y - rec.height/2.},
        theta as f32,
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
    );
}

unsafe fn draw_bez(buffer: &Vec<(f64,f64)>) {
    let (width, height) = (GetScreenWidth(), GetScreenHeight());
    for i in 0..(buffer.len() / 4) {
        DrawSplineSegmentBezierCubic(
            Vector2{ x: buffer[4 * i].0 as f32, y: buffer[4 * i].1 as f32},
            Vector2{ x: buffer[4 * i + 1].0 as f32, y: buffer[4 * i + 1].1 as f32},
            Vector2{ x: buffer[4 * i + 2].0 as f32, y: buffer[4 * i + 2].1 as f32},
            Vector2{ x: buffer[4 * i + 3].0 as f32, y: buffer[4 * i + 3].1 as f32},
            10.,
            crate::ui::colors::WHITE,
        )
    }
}

pub unsafe fn main_game(network: &mut network::Network) {
    let (width, height) = (GetScreenWidth(), GetScreenHeight());
    let mut buffer_cars = Vec::new();
    let mut buffer_bezier = Vec::new();
    let mut buffer_directions = Vec::new();
    while !WindowShouldClose() {
        recv_data(network, &mut buffer_cars, &mut buffer_bezier, &mut buffer_directions);
        draw!({
            ClearBackground(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            });
            draw_bez(&buffer_bezier);
            buffer_cars.iter().zip(buffer_directions.clone()).for_each(|(car,dir)| draw_cars(*car, dir));
        });
        send_data(network);
    }
}
