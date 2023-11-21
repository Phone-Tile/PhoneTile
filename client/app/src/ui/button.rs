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
    fn colision(&self) -> bool {
        // check if finger slides over button
        True;
    }
    fn click(&self) -> bool {
        // check if click on the button
        true;
    }
    fn change_foreground_color(&self, color: Color) {
        panic!()
        // change color
        // each color has an associated color to change into
        // want to know if I need to create an entirely new button
    }
    fn change_background_color(&self, color: Color) {
        panic!()
        // change color
    }
}

pub trait Draw {
    fn draw(&self) -> ();
}




/*** all useful buttons ***/

pub const CREATE_ROOM_BUTTON : Button = Button {
    loc: Rectangle {
        x: 200.0,
        y: 200.0,
        width: 100.0,
        height: 200.0,
    },
    style: Style {
        foreground: colors::WHITE,
        background: colors::RED
    },
    text: None,
};

pub const JOIN_ROOM_BUTTON : Button = Button {
    loc: Rectangle {
        x: 200.0,
        y: 400.0,
        width: 100.0,
        height: 200.0,
    },
    style: Style {
        foreground: colors::WHITE,
        background: colors::YELLOW
    },
    text: None,
};

pub const LOCK_GAME_BUTTON : Button = Button {
    loc: Rectangle {
        x: 200.0,
        y: 400.0,
        width: 100.0,
        height: 200.0,
    },
    style: Style {
        foreground: colors::WHITE,
        background: colors::GREEN,
    },
    text: None,
};

pub const START_GAME_BUTTON : Button = Button {
    loc: Rectangle {
        x: 100.0,
        y: 200.0,
        width: 100.0,
        height: 200.0,
    },
    style: Style {
        foreground: colors::WHITE,
        background: colors::BLUE
    },
    text: None,
};


/*  
pub const RACER : Button = Button {
    loc: Rectangle {
        x: 100.0,
        y: 200.0,
        width: 100.0,
        height: 200.0,
    },
    style: Style {
        foreground: colors::WHITE,
        background: colors::PURPLE,
    },
    text: None,
};
*/