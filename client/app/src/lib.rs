use std::convert::TryInto;
use std::ffi::{c_char, c_int};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
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
use ui::button::Draw;
use ui::colors;
use ui::keyboard::Keyboard;
use ui::text::waiting_text;

// Main function
#[no_mangle]
extern "C" fn main() {
    unsafe {
        let monitor = GetCurrentMonitor();
        let screen_width = GetScreenWidth();
        let screen_height = GetScreenHeight();

        TraceLog(
            TraceLogLevel_LOG_ERROR.try_into().unwrap(),
            raylib_str!("Hello from phone_tile"),
        );
        raylib::InitWindow(screen_height, screen_width, raylib_str!("rust app test"));

        let screen_width = GetScreenWidth();
        let screen_height = GetScreenHeight();

        let mut keyboard = Keyboard::new(screen_width as f32, screen_height as f32);

        //game::maze_fight::maze_fight();
        raylib::ChangeDirectory(raylib_str!("assets"));

        SetTargetFPS(60);

        TraceLog(
            TraceLogLevel_LOG_ERROR.try_into().unwrap(),
            raylib_str!("Holla from phone_tile : Try to connect"),
        );

        let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 2, 2)), 8888);

        let mut network = network::Network::connect(
            &socket,
            1547.,
            757.,
            screen_height as u32,
            screen_width as u32,
        );
        // let mut network =network::Network::connect(&socket,GetMonitorPhysicalHeight(monitor) as f32, GetMonitorPhysicalWidth(monitor) as f32, screen_height as u32, screen_width as u32);
        TraceLog(
            TraceLogLevel_LOG_ERROR.try_into().unwrap(),
            raylib_str!("Holla from phone_tile : Try end to connect"),
        );

        while let Err(_) = network {
            keyboard.reset_value();
            let mut val = keyboard.get_value();
            'window: while !WindowShouldClose() {
                draw!({
                    ClearBackground(colors::BLACK);
                    DrawText(
                        raylib_str!(format!("Addr : {val}")),
                        ((screen_width as f32) * (1. / 9.)) as c_int,
                        ((screen_height as f32) * (1. / 11.)) as c_int,
                        50,
                        colors::BLUE,
                    );
                    keyboard.draw();
                    keyboard.update();
                });
                val = keyboard.get_value();
                if val.matches(".").count() > 3 {
                    break 'window;
                }
            }
            val.pop();
            socket.set_ip(IpAddr::from_str(format!("{val}").as_str()).unwrap());
            network = network::Network::connect(
                &socket,
                1547.,
                757.,
                screen_height as u32,
                screen_width as u32,
            );
        }

        let mut network = network.unwrap();

        keyboard.reset_value();

        let mut room = 0;
        let mut want_join = false;

        let mut is_host = false;

        while !WindowShouldClose() {
            draw!({
                ClearBackground(colors::BLACK);

                match network.get_status() {
                    network::Status::Connected => {
                        if want_join {
                            let mut val = keyboard.get_value();
                            DrawText(
                                raylib_str!(format!("Room ID :")),
                                ((screen_width as f32) * (1.5 / 9.)) as c_int,
                                ((screen_height as f32) * (1. / 13.)) as c_int,
                                ((screen_height as f32) * (1. / 13.)) as c_int,
                                colors::YELLOW,
                            );
                            DrawText(
                                raylib_str!(val),
                                ((screen_width as f32) * (2. / 5.)) as c_int,
                                ((screen_height as f32) * (2.1 / 13.)) as c_int,
                                ((screen_height as f32) * (1.9 / 13.)) as c_int,
                                colors::YELLOW,
                            );
                            keyboard.draw();
                            keyboard.update();

                            if val.matches(".").count() > 0 {
                                val.pop();
                                let room = val.parse().unwrap();
                                keyboard.reset_value();
                                network.join_room(room).unwrap();
                            }
                        } else {
                            DrawText(
                                raylib_str!("Phone"),
                                ((screen_width as f32) * (1. / 9.)) as c_int,
                                ((screen_height as f32) * (1. / 11.)) as c_int,
                                ((screen_height as f32) * (1.4 / 11.)) as c_int,
                                colors::BLUE,
                            );
                            DrawText(
                                raylib_str!("Tile"),
                                ((screen_width as f32) * (2. / 9.)) as c_int,
                                ((screen_height as f32) * (2.6 / 11.)) as c_int,
                                ((screen_height as f32) * (1.4 / 11.)) as c_int,
                                colors::PURPLE,
                            );

                            let create_room = button::create_room(screen_height, screen_width);
                            let join_room = button::join_room(screen_height, screen_width);

                            create_room.draw();
                            join_room.draw();

                            if create_room.click() {
                                is_host = true;
                                room = network.create_room().unwrap();
                            };
                            if join_room.click() {
                                want_join = true;
                            }
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
                                raylib_str!(format!("Room ID :")),
                                ((screen_width as f32) * (1.5 / 9.)) as c_int,
                                ((screen_height as f32) * (1. / 13.)) as c_int,
                                ((screen_height as f32) * (1. / 13.)) as c_int,
                                colors::YELLOW,
                            );
                            DrawText(
                                raylib_str!(format!("{}", room)),
                                ((screen_width as f32) * (2. / 5.)) as c_int,
                                ((screen_height as f32) * (2.1 / 13.)) as c_int,
                                ((screen_height as f32) * (1.9 / 13.)) as c_int,
                                colors::YELLOW,
                            );

                            let racer = button::racer(screen_height, screen_width);
                            let snake = button::snake(screen_height, screen_width);
                            let golf = button::golf(screen_height, screen_width);

                            racer.draw();
                            snake.draw();
                            golf.draw();

                            /*if button::GAME().colision() {
                                button::GAME().change_foreground_color(colors::BLUE);
                            };*/
                            if racer.click() {
                                network.lock_room(network::Game::Racer).unwrap();
                            }
                            if snake.click() {
                                network.lock_room(network::Game::Snake).unwrap();
                            }
                            if golf.click() {
                                network.lock_room(network::Game::Unknown).unwrap();
                            }
                        } else {
                            waiting_text(screen_height, screen_width)
                        }
                    }
                    network::Status::InLockRoom(n) => {
                        DrawText(
                            raylib_str!(format!("Take your")),
                            ((screen_width as f32) * (1. / 9.)) as c_int,
                            ((screen_height as f32) * (1. / 13.)) as c_int,
                            ((screen_height as f32) * (1. / 13.)) as c_int,
                            colors::RED,
                        );
                        DrawText(
                            raylib_str!(format!("positions !")),
                            ((screen_width as f32) * (1. / 9.)) as c_int,
                            ((screen_height as f32) * (2. / 13.)) as c_int,
                            ((screen_height as f32) * (1. / 13.)) as c_int,
                            colors::RED,
                        );
                        DrawText(
                            raylib_str!(format!("{n}")),
                            ((screen_width as f32) * (2. / 5.)) as c_int,
                            ((screen_height as f32) * (4. / 13.)) as c_int,
                            ((screen_height as f32) * (2. / 13.)) as c_int,
                            colors::PURPLE,
                        );
                        if is_host {
                            let start_game = button::start_game(screen_height, screen_width);
                            start_game.draw();
                            /*if button::start_game().colision() {
                                button::start_game().change_foreground_color(colors::BLUE);
                            };*/
                            if start_game.click() {
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
        game::snake::main_game(&mut network);
        game::maze_fight::main_game(&mut network);
        CloseWindow();
    }

    // UnloadTexture(tex_bunny);
}
