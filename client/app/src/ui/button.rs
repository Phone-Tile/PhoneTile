use std::ffi::{c_char, c_int, c_float};

use raylib::{raylib_str, Color, DrawRectangle, Vector2, DrawText, Rectangle, IsMouseButtonDown, IsMouseButtonPressed, IsMouseButtonReleased, MouseButton_MOUSE_BUTTON_LEFT, GetMousePosition};
use std::convert::TryInto;
use crate::ui::colors;

#[derive(Clone, Copy)]
pub struct Style {
    foreground: Color,
    background: Color,
}

pub struct Button {
    loc: Rectangle,
    style: Style,
    text: Option<String>,
    text_pos_x: f32,
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
    pub fn new(loc: Rectangle, style: Style, text: Option<String>, text_pos_x: f32) -> Button {
        Button { loc, style, text, text_pos_x }
    }

    pub fn new_ratio(px : f32, py : f32, pwidth : f32, pheight:f32, style: Style,text: Option<String>, screen_width : f32, screen_height : f32) -> Button {
        Self::new(
            raylib::Rectangle {
                x: ((screen_width as f32)* px) as c_float,
                y: ((screen_height as f32)* py) as c_float,
                width: ((screen_width as f32)* pwidth) as c_float,
                height: ((screen_height as f32)* pheight) as c_float,
            },
            style,
            text,
            25 as c_float,
        )
    }

    pub fn get_text(&self) -> Option<String> {
        match &self.text {
            None => None,
            Some(text) => {
                Some(text.to_string())
            }
        }
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
                    (self.loc.x+(self.text_pos_x)) as c_int,
                    (self.loc.y+(self.loc.height/4.)) as c_int,
                    (self.loc.height/2.) as c_int,
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
            x: ((screen_width as f32)*(1./5.)) as c_float,
            y: ((screen_height as f32)*(5.5/11.)) as c_float,
            width: ((screen_width as f32)*(3./5.)) as c_float,
            height: ((screen_height as f32)*(1.5/11.)) as c_float,
        },
        Style::new(colors::WHITE, colors::RED),
        Some(format!("Create")),
        1./24. * (screen_width as f32) as c_float,
    )
}

pub fn join_room(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: ((screen_width as f32) * (1./5.)) as c_float,
            y: ((screen_height as f32)*(8./11.)) as c_float,
            width: ((screen_width as f32) * (3./5.)) as c_float,
            height: ((screen_height as f32)*(1.5/11.)) as c_float,
        },
        Style::new(colors::WHITE, colors::ORANGE),
        Some(format!("Join")),
        1./7. * (screen_width as f32) as c_float,
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
            x: ((screen_width as f32)*(1./5.)) as c_float,
            y: (screen_height as f32)*(7./13.) as c_float,
            width: ((screen_width as f32)*(3./5.)) as c_float,
            height: ((screen_height as f32)*(1.5/13.)) as c_float,
        },
        Style::new(colors::WHITE, colors::PINK),
        Some(format!("Start")),
        1./7. * (screen_width as f32) as c_float,
    ) 
}

pub fn racer(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: ((screen_width as f32)*(1./5.)) as c_float,
            y: ((screen_height as f32)*(4.5/13.)) as c_float,
            width: ((screen_width as f32)*(3./5.)) as c_float,
            height: ((screen_height as f32)*(1.5/13.)) as c_float,
        },
        Style::new(colors::WHITE, colors::GREEN),
        Some(format!("Racer")),
        1./8. * (screen_width as f32) as c_float,
    ) 
}

pub fn snake(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: ((screen_width as f32)*(1./5.)) as c_float,
            y: (screen_height as f32)*(7./13.) as c_float,
            width: ((screen_width as f32)*(3./5.)) as c_float,
            height: ((screen_height as f32)*(1.5/13.)) as c_float,
        },
        Style::new(colors::WHITE, colors::GREEN),
        Some(format!("Snake")),
        1./9. * (screen_width as f32) as c_float,
    ) 
}

pub fn golf(screen_height: i32, screen_width: i32) -> Button {
    Button::new(
        raylib::Rectangle {
            x: ((screen_width as f32)*(1./5.)) as c_float,
            y: (screen_height as f32)*(9.5/13.) as c_float,
            width: ((screen_width as f32)*(3./5.)) as c_float,
            height: ((screen_height as f32)*(1.5/13.)) as c_float,
        },
        Style::new(colors::WHITE, colors::GREEN),
        Some(format!("Golf")),
        1./6. * (screen_width as f32) as c_float,
    ) 
}
