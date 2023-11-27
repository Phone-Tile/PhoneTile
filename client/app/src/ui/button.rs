use std::ffi::{c_char, c_int, c_float};

use raylib::{raylib_str, Color, DrawRectangle, DrawText, Rectangle, IsMouseButtonDown, IsMouseButtonPressed, IsMouseButtonReleased, MouseButton_MOUSE_BUTTON_LEFT, GetMousePosition};
use std::convert::TryInto;
use crate::ui::colors;

pub struct Style {
    foreground: Color,
    background: Color,
}

pub struct Button {
    loc: Rectangle,
    style: Style,
    text: Option<String>,
}

impl Style {
    pub fn new(foreground: Color, background: Color) -> Style {
        Style {
            foreground,
            background,
        }
    }
}

impl Button {
    pub fn new(loc: Rectangle, style: Style, text: Option<String>) -> Button {
        Button { loc, style, text }
    }
}

impl Draw for Button {
    fn draw(&self) {
        unsafe {
            DrawRectangle(
                self.loc.x as c_int + 5,
                self.loc.y as c_int + 5,
                self.loc.width as c_int,
                self.loc.height as c_int,
                self.style.background,
            );
            DrawRectangle(
                self.loc.x as c_int,
                self.loc.y as c_int,
                self.loc.width as c_int,
                self.loc.height as c_int,
                self.style.foreground,
            );
            if let Some(text) = &self.text {
                DrawText(
                    raylib_str!(text),
                    self.loc.x as c_int,
                    self.loc.y as c_int,
                    self.loc.height as c_int,
                    self.style.background,
                );
            }
        };
    }
}

impl Button {
    pub fn colision(&self) -> bool {
        // check if finger slides over button
        false
    }
    pub fn click(&self) -> bool {
        // check if click on the button
        unsafe {
            if raylib::IsMouseButtonReleased(raylib::MouseButton_MOUSE_BUTTON_LEFT as i32) {
                let mouse_pos = raylib::GetMousePosition();
                if self.loc.x < mouse_pos.x
                    && mouse_pos.x < self.loc.x + self.loc.width
                    && self.loc.y < mouse_pos.y
                    && mouse_pos.y < self.loc.y + self.loc.height
                {
                    return true;
                }
            }
            false
        }
    }
    /* 
    pub fn change_foreground_color(&self, color: Color) {
        unsafe{
            let x = self.loc.x;
            let y = self.loc.y;
            let mouse_pos = GetMousePosition();

            if IsMouseButtonDown(MouseButton_MOUSE_BUTTON_LEFT.try_into().unwrap())
            && x < mouse_pos.x && mouse_pos.x < x+self.loc.width
            && y < mouse_pos.y && mouse_pos.y < y+self.loc.height {
                Button::new(
                    self.loc,
                    Style::new(colors::WHITE, {match self.style.background {
                        colors::YELLOW => colors::ORANGE,
                        colors::GREEN => colors::BLUE,
                        colors::BLUE => colors::PURPLE,
                        colors::PURPLE => colors::PINK,
                        _ => panic!("there shouldn't be any buttons of such color !")
                    }}),
                    Some(format!("Create")),
                ).draw()
            }
        }
        
    }*/
    pub fn change_background_color(&self, color: Color) {
        todo!()
        // change color
    }
}

pub trait Draw {
    fn draw(&self) -> ();
}

/*** all useful buttons ***/

pub fn create_room(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: 200.0,
            y: (screen_height as f32)*(5./11.) as c_float,
            width: 1000.0,
            height: (screen_height as f32)*(2./11.) as c_float,
        },
        Style::new(colors::WHITE, colors::YELLOW),
        Some(format!("Create")),
    )
}

pub fn join_room(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: 200.0,
            y: (screen_height as f32)*(8./11.) as c_float,
            width: 1000.0,
            height: (screen_height as f32)*(2./11.) as c_float,
        },
        Style::new(colors::WHITE, colors::YELLOW),
        Some(format!("Join")),
    ) 
}

/* 
pub fn game_select() -> Button {
    Button::new(
        raylib::Rectangle {
            x: 200.0,
            y: 400.0,
            width: 1000.0,
            height: 300.0,
        },
        Style::new(colors::WHITE, colors::GREEN),
        Some(format!("Start1")),
    ) 
}*/

pub fn start_game(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: (screen_width as f32)*(1./4.) as c_float,
            y: (screen_height as f32)*(6./9.) as c_float,
            width: (screen_width as f32)*(1./2.) as c_float,
            height: (screen_height as f32)*(2./9.) as c_float,
        },
        Style::new(colors::WHITE, colors::BLUE),
        Some(format!("Start")),
    ) 
}

pub fn racer(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: 100.0,
            y: (screen_height as f32)*(4./13.) as c_float,
            width: 1000.0,
            height: (screen_height as f32)*(2./13.) as c_float,
        },
        Style::new(colors::WHITE, colors::GREEN),
        Some(format!("Racer")),
    ) 
}

pub fn snake(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: 100.0,
            y: (screen_height as f32)*(7./13.) as c_float,
            width: 1000.0,
            height: (screen_height as f32)*(2./13.) as c_float,
        },
        Style::new(colors::WHITE, colors::GREEN),
        Some(format!("Snake")),
    ) 
}

pub fn golf(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: 100.0,
            y: (screen_height as f32)*(10./13.) as c_float,
            width: 1000.0,
            height: (screen_height as f32)*(2./13.) as c_float,
        },
        Style::new(colors::WHITE, colors::GREEN),
        Some(format!("Golf")),
    ) 
}

/*
pub fn game_select() -> Button {
    Button::new(
        raylib::Rectangle {
            x: 200.0,
            y: 400.0,
            width: 1000.0,
            height: 300.0,
        },
        Style::new(colors::WHITE, colors::GREEN),
        Some(format!("Start1")),
    ) 
} */

pub const RACER: Button = Button {
    loc: Rectangle {
        x: 100.0,
        y: 400.0,
        width: 1000.0,
        height: 300.0,
    },
    style: Style {
        foreground: colors::WHITE,
        background: colors::PURPLE,
    },
    text: None,
};
