use std::ffi::{c_char, c_int};

use raylib::{raylib_str, Color, DrawRectangle, DrawText, Rectangle};

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
            if raylib::IsMouseButtonPressed(raylib::MouseButton_MOUSE_BUTTON_LEFT as i32) {
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
    pub fn change_foreground_color(&self, color: Color) {
        todo!()
        // change color
        // each color has an associated color to change into
        // want to know if I need to create an entirely new button
    }
    pub fn change_background_color(&self, color: Color) {
        todo!()
        // change color
    }
}

pub trait Draw {
    fn draw(&self) -> ();
}

/*** all useful buttons ***/

pub fn create_room() -> Button {
    Button::new(
        raylib::Rectangle {
            x: 200.0,
            y: 200.0,
            width: 1000.0,
            height: 300.0,
        },
        Style::new(colors::WHITE, colors::YELLOW),
        Some(format!("Create")),
    )
}

pub fn join_room() -> Button {
    Button::new(
        raylib::Rectangle {
            x: 200.0,
            y: 700.0,
            width: 1000.0,
            height: 300.0,
        },
        Style::new(colors::WHITE, colors::YELLOW),
        Some(format!("Join")),
    ) 
}

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
}

pub fn start_game() -> Button {
    Button::new(
        raylib::Rectangle {
            x: 100.0,
            y: 200.0,
            width: 1000.0,
            height: 300.0,
        },
        Style::new(colors::WHITE, colors::BLUE),
        Some(format!("Start2")),
    ) 
}

/*
pub fn start_game() -> Button {
    Button::new(
        raylib::Rectangle {
            x: 100.0,
            y: 200.0,
            width: 1000.0,
            height: 300.0,
        },
        Style::new(colors::WHITE, colors::BLUE),
        Some(format!("Start2")),
    ) 
}
 */

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
