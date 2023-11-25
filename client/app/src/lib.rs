use std::convert::TryInto;
use std::ffi::c_char;
use std::time;

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
mod game;
mod network;
mod ui;
use ui::button;
use ui::button::{
    Button, Draw, Style
};
use ui::colors;

// Main function
#[no_mangle]
extern "C" fn main() {
    let mut network = network::Network::connect(1., 1., 1, 1);
    unsafe {
        //let game_selected: Option<Game> = None;
        TraceLog(
            TraceLogLevel_LOG_ERROR.try_into().unwrap(),
            raylib_str!("Hello from phone_tile"),
        );

        let screen_width = 0;
        let screen_height = 0;

        raylib::InitWindow(screen_width, screen_height, raylib_str!("rust app test"));

        raylib::ChangeDirectory(raylib_str!("assets"));

        let tex_bunny = raylib::LoadTexture(raylib_str!("wabbit_alpha.png"));

        let mut room = 0;

        SetTargetFPS(60);

        let mut is_host = true;

        while !WindowShouldClose() {
            draw!({
                ClearBackground(colors::BLACK);

                match network.get_status() {
                    network::Status::Connected => {
                        //TEXT : PHONE TILE
                        button::create_room().draw();
                        button::join_room().draw();

                        if button::create_room().colision() {
                            button::create_room().change_foreground_color(colors::BLUE);
                        };
                        if button::join_room().colision() {
                            button::join_room().change_foreground_color(colors::BLUE);
                        };
                        if button::create_room().click() {
                            // as char ???
                            room = network.create_room().unwrap();
                            // next line not necessary i think
                            // button::game_select().draw()
                        };
                        if button::join_room().click() {
                            // TYPE ID;
                            network.join_room(1).unwrap();
                            //TEXT : WAITING ...
                        }
                    }
                    network::Status::Disconnected => {
                        DrawText(
                            raylib_str!("Sorry, network unsable :("),
                            100,
                            200,
                            100,
                            colors::WHITE,
                        );
                    }
                    network::Status::InRoom => {
                        // if it is the host :
                        button::game_select().draw();
                        if button::game_select().colision() {
                            button::game_select().change_foreground_color(colors::BLUE);
                        };
                        if button::game_select().click() {
                            network.game_select();
                        }

                        DrawText(
                            raylib_str!(format!("{}", room)),
                            500,
                            1500,
                            300,
                            colors::WHITE,
                        );
                        // if it is not the host :
                        // TEXT : waiting ...
                    }
                    network::Status::SelectedGame => {
                        // if host
                        button::RACER.draw();
                        if button::RACER.colision() {
                            button::RACER.change_foreground_color(colors::BLUE);
                        };
                        if button::RACER.click() {
                            network.lock_room().unwrap();
                        }
                    }
                    network::Status::InLockRoom(n) => {
                        // as char ?
                        DrawText(raylib_str!(format!("{n}")), 100, 200, 1000, colors::PURPLE);
                        //if host (again) :
                        button::start_game().draw();
                        if button::start_game().colision() {
                            button::start_game().change_foreground_color(colors::BLUE);
                        };
                        if button::start_game().click() {
                            network.launch_game().unwrap();
                        }
                    }
                    network::Status::InGame => {
                        // network.send(data);
                        // main_game(receive(data));
                        break;
                    }
                }
            });

            DrawFPS(10, 10);
        }
        game::racer::main_game(&mut network);
        CloseWindow();
    }

    // UnloadTexture(tex_bunny);
}
