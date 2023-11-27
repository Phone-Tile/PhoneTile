use std::convert::TryInto;
use std::ffi::{c_char, c_float, c_int};
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
    unsafe {
        let monitor = GetCurrentMonitor();
        let screen_width = GetScreenWidth();
        let screen_height = GetScreenHeight();

        let mut network = network::Network::connect(GetMonitorPhysicalHeight(monitor) as f32, GetMonitorPhysicalWidth(monitor) as f32, screen_height as u32, screen_width as u32).unwrap();
        TraceLog(
            TraceLogLevel_LOG_ERROR.try_into().unwrap(),
            raylib_str!("Hello from phone_tile"),
        );

        raylib::InitWindow(screen_height, screen_width, raylib_str!("rust app test"));

        raylib::ChangeDirectory(raylib_str!("assets"));

        let mut room = 0;

        SetTargetFPS(60);

        let mut is_host = false;

        while !WindowShouldClose() {
            draw!({
                ClearBackground(colors::BLACK);

                match network.get_status() {
                    network::Status::Connected => {
                        DrawText(
                            raylib_str!("Phone Tile"),
                            0,
                            ((screen_height as f32)*(1./11.)) as c_int,
                            ((screen_height as f32)*(3./11.)) as c_int,
                            colors::WHITE,
                        );

                        button::create_room(screen_height, screen_width).draw();
                        button::join_room(screen_height, screen_width).draw();

                        /* 
                        if button::create_room().colision() {
                            button::create_room().change_foreground_color(colors::BLUE);
                        };
                        if button::join_room().colision() {
                            button::join_room().change_foreground_color(colors::BLUE);
                        };*/
                        if button::create_room(screen_height, screen_width).click() {
                            is_host = true;
                            room = network.create_room().unwrap();
                        };
                        if button::join_room(screen_height, screen_width).click() {
                            // TODO : TYPE ID;
                            network.join_room(1).unwrap();
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
                        if is_host {
                            DrawText(
                                raylib_str!(format!("{}", room)),
                                0,
                                ((screen_height as f32)*(1./13.)) as c_int,
                                ((screen_height as f32)*(2./13.)) as c_int,
                                colors::WHITE,
                            );
                            button::racer(screen_height, screen_width).draw();
                            button::snake(screen_height, screen_width).draw();
                            button::golf(screen_height, screen_width).draw();
                            /*if button::GAME().colision() {
                                button::GAME().change_foreground_color(colors::BLUE);
                            };*/
                            if button::racer(screen_height, screen_width).click() {
                                network.lock_room(network::Game::Racer).unwrap();
                            }
                            if button::snake(screen_height, screen_width).click() {
                                network.lock_room(network::Game::Test).unwrap();
                            }
                            if button::golf(screen_height, screen_width).click() {
                                network.lock_room(network::Game::Unknown).unwrap();
                            }
                        } else {
                            DrawText(
                                raylib_str!("Waiting ..."),
                                100,
                                200,
                                100,
                                colors::WHITE,
                            )    
                        }
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
                        DrawText(
                            raylib_str!(format!("{n}")),
                            0,
                            ((screen_height as f32)*(1./9.)) as c_int,
                            ((screen_height as f32)*(3./9.)) as c_int,
                            colors::PURPLE
                        );
                        if is_host {
                            button::start_game(screen_height, screen_width).draw();
                            /*if button::start_game().colision() {
                                button::start_game().change_foreground_color(colors::BLUE);
                            };*/
                            if button::start_game(screen_height, screen_width).click() {
                                network.launch_game().unwrap();
                            }
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
