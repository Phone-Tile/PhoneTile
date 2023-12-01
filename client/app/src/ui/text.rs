use crate::ui::colors;
use raylib::{raylib_str, DrawText};
use std::ffi::{c_char, c_int};

pub fn waiting_text(screen_height: i32, screen_width: i32) {
    unsafe {
        DrawText(
            raylib_str!("Waiting ..."),
            ((screen_width as f32) * (1. / 9.)) as c_int,
            ((screen_height as f32) * (1. / 13.)) as c_int,
            ((screen_height as f32) * (1. / 13.)) as c_int,
            colors::RED,
        );
        DrawText(
            raylib_str!("Waiting ..."),
            ((screen_width as f32) * (1. / 9.)) as c_int,
            ((screen_height as f32) * (3. / 13.)) as c_int,
            ((screen_height as f32) * (1. / 13.)) as c_int,
            colors::ORANGE,
        );
        DrawText(
            raylib_str!("Waiting ..."),
            ((screen_width as f32) * (1. / 9.)) as c_int,
            ((screen_height as f32) * (5. / 13.)) as c_int,
            ((screen_height as f32) * (1. / 13.)) as c_int,
            colors::YELLOW,
        );
        DrawText(
            raylib_str!("Waiting ..."),
            ((screen_width as f32) * (1. / 9.)) as c_int,
            ((screen_height as f32) * (7. / 13.)) as c_int,
            ((screen_height as f32) * (1. / 13.)) as c_int,
            colors::GREEN,
        );
        DrawText(
            raylib_str!("Waiting ..."),
            ((screen_width as f32) * (1. / 9.)) as c_int,
            ((screen_height as f32) * (9. / 13.)) as c_int,
            ((screen_height as f32) * (1. / 13.)) as c_int,
            colors::BLUE,
        );
        DrawText(
            raylib_str!("Waiting ..."),
            ((screen_width as f32) * (1. / 9.)) as c_int,
            ((screen_height as f32) * (11. / 13.)) as c_int,
            ((screen_height as f32) * (1. / 13.)) as c_int,
            colors::PURPLE,
        );
    }
}
