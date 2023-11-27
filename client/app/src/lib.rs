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
    Button, Draw, Style, CREATE_ROOM_BUTTON, JOIN_ROOM_BUTTON, LOCK_GAME_BUTTON, START_GAME_BUTTON,
};
use ui::colors;

// Main function
#[no_mangle]
extern "C" fn main() {
    let mut network = network::Network::connect(1., 1., 1, 1).unwrap();
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

        let create_room_button = button::Button::new(
            raylib::Rectangle {
                x: 200.0,
                y: 200.0,
                width: 1000.0,
                height: 300.0,
            },
            button::Style::new(colors::WHITE, colors::YELLOW),
            Some(format!("Create")),
        );

        SetTargetFPS(60);

        while !WindowShouldClose() {
            draw!({
                ClearBackground(colors::BLACK);

                match network.get_status() {
                    network::Status::Connected => {
                        //TEXT : PHONE TILE
                        create_room_button.draw();
                        button::JOIN_ROOM_BUTTON.draw();

                        if create_room_button.colision() {
                            create_room_button.change_foreground_color(colors::BLUE);
                        };
                        if button::JOIN_ROOM_BUTTON.colision() {
                            button::JOIN_ROOM_BUTTON.change_foreground_color(colors::BLUE);
                        };
                        if create_room_button.click() {
                            // as char ???
                            room = network.create_room().unwrap();
                            // next line not necessary i think
                            // button::LOCK_GAME_BUTTON.draw()
                        };
                        if JOIN_ROOM_BUTTON.click() {
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
                        button::LOCK_GAME_BUTTON.draw();
                        if button::LOCK_GAME_BUTTON.colision() {
                            button::LOCK_GAME_BUTTON.change_foreground_color(colors::BLUE);
                        };
                        if button::LOCK_GAME_BUTTON.click() {
                            network.lock_room(network::Game::Test).unwrap();
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
                    // network::Status::SelectedGame => {
                    //     // if host
                    //     button::RACER.draw();
                    //     if button::RACER.colision() {
                    //         button::RACER.change_foreground_color(colors::BLUE);
                    //     };
                    //     if button::RACER.click() {
                    //         network.lock_room(network::Game::Racer).unwrap();
                    //     }
                    // }
                    network::Status::InLockRoom(n) => {
                        // as char ?
                        DrawText(raylib_str!(format!("{n}")), 100, 200, 1000, colors::PURPLE);
                        //if host (again) :
                        button::START_GAME_BUTTON.draw();
                        if button::START_GAME_BUTTON.colision() {
                            button::START_GAME_BUTTON.change_foreground_color(colors::BLUE);
                        };
                        if button::START_GAME_BUTTON.click() {
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
