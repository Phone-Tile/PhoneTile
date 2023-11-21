use std::convert::TryInto;
use std::ffi::c_char;

extern crate raylib;
use raylib::*;

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
mod network;
mod game;
use ui::button::{Button, Draw, Style, JOIN_ROOM_BUTTON, CREATE_ROOM_BUTTON, START_GAME_BUTTON, LOCK_GAME_BUTTON};
use ui::button;
use ui::colors;
use network::network::{Status, get_status, create_room, join_room, lock_game, launch_game, send, receive};
use game::cars::game::{move_car, draw_tracks};





// Main function
#[no_mangle]
extern "C" fn main() {
    unsafe {
        //let game_selected: Option<Game> = None;
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

        while !WindowShouldClose() {
            draw!({
                ClearBackground(colors::BLACK);

                match get_status() {
                    Status::CONNECTED => {

                        //TEXT : PHONE TILE
                        button::CREATE_ROOM_BUTTON.draw();
                        button::JOIN_ROOM_BUTTON.draw();

                        if button::CREATE_ROOM_BUTTON.collision() {
                            button::CREATE_ROOM_BUTTON.change_foreground_color();
                        };
                        if button::JOIN_ROOM_BUTTON.collision() {
                            button::JOIN_ROOM_BUTTON.change_foreground_color();
                        };
                        if CREATE_ROOM_BUTTON.click() {
                            // as char ???
                            DrawText(
                                create_room() as char,
                                100,
                                200,
                                100,
                                Color::WHITE
                            );
                            // next line not necessary i think
                            // button::LOCK_GAME_BUTTON.draw()
                        };
                        if JOIN_ROOM_BUTTON.click() {
                            // TYPE ID;
                            join_room();
                            //TEXT : WAITING ...
                        }
                    }
                    Status::DISCONNECTED => {
                        DrawText(
                            "Sorry, network unsable :(",
                            100,
                            200,
                            100,
                            Color::WHITE
                        );
                    }
                    Status::IN_ROOM => {
                        // if it is the host :
                        button::LOCK_GAME_BUTTON.draw();
                        if button::LOCK_GAME_BUTTON.collision() {
                            button::LOCK_GAME_BUTTON.change_foreground_color();
                        };
                        if button::LOCK_GAME_BUTTON.click() {
                            game_select();
                        }
                        // if it is not the host :
                        // TEXT : waiting ...
                    }
                    Status::GAME_SELECT => {
                        // if host
                        button::RACER.draw();
                        if button::RACER.collision() {
                            button::RACER.change_foreground_color();
                        };
                        if button::RACER.click() {
                            lock_game();
                        }
                    }
                    Status::IN_LOCK_GAME(n) => {
                        // as char ?
                        DrawText(
                            n as char,
                            100,
                            200,
                            1000,
                            Color::PURPLE
                        );
                        //if host (again) :
                        button::START_GAME_BUTTON.draw();
                        if button::START_GAME_BUTTON.collision() {
                            button::START_GAME_BUTTON.change_foreground_color();
                        };
                        if button::START_GAME_BUTTON.click() {
                            launch_game();
                        }
                    }
                    Status::IN_GAME() => {
                        send(data);
                        main_game(receive(data));
                    }
                    }
                });

                DrawFPS(10, 10);
            };
        }

        UnloadTexture(tex_bunny);

        CloseWindow()
}

