use std::convert::TryInto;
use std::ffi::c_char;

mod network;

extern crate raylib;
use raylib::{
    ClearBackground, CloseWindow, Color, DrawFPS, DrawText, DrawTexture, Rectangle, SetTargetFPS,
    TraceLog, TraceLogLevel_LOG_ERROR, UnloadTexture, WindowShouldClose,
};

use raylib::{draw, raylib_str};

//////////////////////////////////////////////////////////////////////////
// WARNING :                                                            //
// This is for android DON'T TOUCH THIS WORK !                          //
//////////////////////////////////////////////////////////////////////////
extern crate native_app_glue;

#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut native_app_glue::ANativeActivity,
    saved_state: *mut ::std::os::raw::c_void,
    saved_state_size: usize,
) {
    unsafe { native_app_glue::ANativeActivity_onCreate_C(activity, saved_state, saved_state_size) }
}
//////////////////////////////////////////////////////////////////////////
mod ui;
use ui::button::{Button, Draw, Style};

// Main function
#[no_mangle]
extern "C" fn main() {
    unsafe {
        TraceLog(
            TraceLogLevel_LOG_ERROR.try_into().unwrap(),
            raylib_str!("Hello from phone_tile"),
        );

        let screen_width = 800;
        let screen_height = 450;

        raylib::InitWindow(screen_width, screen_height, raylib_str!("rust app test"));

        raylib::ChangeDirectory(raylib_str!("assets"));

        let tex_bunny = raylib::LoadTexture(raylib_str!("wabbit_alpha.png"));

        SetTargetFPS(60);

        let style = Style::new(
            Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255,
            },
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        );

        let button = Button::new(
            Rectangle {
                x: 100.0,
                y: 200.0,
                width: 100.0,
                height: 100.0,
            },
            style,
            Some("Un bouton".to_string()),
        );

        while !WindowShouldClose() {
            draw!({
                ClearBackground(Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                });

                DrawText(
                    raylib_str!("Hello from the application"),
                    100,
                    100,
                    50,
                    Color {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 255,
                    },
                );
                button.draw();

                DrawTexture(
                    tex_bunny,
                    200,
                    100,
                    Color {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                );

                DrawFPS(10, 10);
            });
        }

        UnloadTexture(tex_bunny);

        CloseWindow()
    }
}
