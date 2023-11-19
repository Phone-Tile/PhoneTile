use std::ffi::{c_char, c_int};

use raylib::{raylib_str, Color, DrawRectangle, DrawText, Rectangle};

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

pub trait Draw {
    fn draw(&self) -> ();
}
