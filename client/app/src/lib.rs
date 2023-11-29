use std::convert::TryInto;
use std::ffi::{c_char, c_float, c_int};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
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
use ui::keyboard::Keyboard;
use ui::{button, keyboard};
use ui::button::{
    Button, Draw, Style
};
use ui::colors;
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

        raylib::ChangeDirectory(raylib_str!("assets"));

        SetTargetFPS(60);



        TraceLog(
            TraceLogLevel_LOG_ERROR.try_into().unwrap(),
            raylib_str!("Holla from phone_tile : Try to connect"),
        );

        let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),8888);

        let mut network =network::Network::connect(&socket,GetMonitorPhysicalHeight(monitor) as f32, GetMonitorPhysicalWidth(monitor) as f32, screen_height as u32, screen_width as u32);
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
                            ((screen_width as f32)*(1./9.)) as c_int,
                            ((screen_height as f32)*(1./11.)) as c_int,
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
            network = network::Network::connect(&socket,GetMonitorPhysicalHeight(monitor) as f32, GetMonitorPhysicalWidth(monitor) as f32, screen_height as u32, screen_width as u32);
        }

        let mut network = network.unwrap();

        keyboard.reset_value();

        let mut room = 0;
        let mut is_host = false;
        let mut want_join = false;

        while !WindowShouldClose() {
            draw!({
                ClearBackground(colors::BLACK);

                match network.get_status() {
                    network::Status::Connected => {


                        if want_join {
                            let mut val = keyboard.get_value();
                            DrawText(
                                raylib_str!(format!("Room ID :")),
                                ((screen_width as f32)*(1.5/9.)) as c_int,
                                ((screen_height as f32)*(1./13.)) as c_int,
                                ((screen_height as f32)*(1./13.)) as c_int,
                                colors::YELLOW
                            );
                            DrawText(
                                raylib_str!(val),
                                ((screen_width as f32)*(2./5.)) as c_int,
                                ((screen_height as f32)*(2.1/13.)) as c_int,
                                ((screen_height as f32)*(1.9/13.)) as c_int,
                                colors::YELLOW
                            );
                            keyboard.draw();
                            keyboard.update();

                            if val.matches(".").count() > 0 {
                                val.pop();
                                let room = val.parse().unwrap();
                                keyboard.reset_value();
                                network.join_room(room).unwrap();
                            }
                        }else{
                        DrawText(
                            raylib_str!("Phone"),
                            ((screen_width as f32)*(1./9.)) as c_int,
                            ((screen_height as f32)*(1./11.)) as c_int,
                            ((screen_height as f32)*(1.4/11.)) as c_int,
                            colors::BLUE,
                        );
                        DrawText(
                            raylib_str!("Tile"),
                            ((screen_width as f32)*(2./9.)) as c_int,
                            ((screen_height as f32)*(2.6/11.)) as c_int,
                            ((screen_height as f32)*(1.4/11.)) as c_int,
                            colors::PURPLE,
                        );

                        button::create_room(screen_height, screen_width).draw();
                        button::join_room(screen_height, screen_width).draw();

                        if button::create_room(screen_height, screen_width).click() {
                            is_host = true;
                            room = network.create_room().unwrap();
                        };
                        if button::join_room(screen_height, screen_width).click() {
                            // TODO : TYPE ID;
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
                                ((screen_width as f32)*(1.5/9.)) as c_int,
                                ((screen_height as f32)*(1./13.)) as c_int,
                                ((screen_height as f32)*(1./13.)) as c_int,
                                colors::YELLOW
                            );
                            DrawText(
                                raylib_str!(format!("{}", room)),
                                ((screen_width as f32)*(2./5.)) as c_int,
                                ((screen_height as f32)*(2.1/13.)) as c_int,
                                ((screen_height as f32)*(1.9/13.)) as c_int,
                                colors::YELLOW
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
                            waiting_text(screen_height, screen_width) 
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
                            raylib_str!(format!("Take your")),
                            ((screen_width as f32)*(1./9.)) as c_int,
                            ((screen_height as f32)*(1./13.)) as c_int,
                            ((screen_height as f32)*(1./13.)) as c_int,
                            colors::RED
                        );
                        DrawText(
                            raylib_str!(format!("positions !")),
                            ((screen_width as f32)*(1./9.)) as c_int,
                            ((screen_height as f32)*(2./13.)) as c_int,
                            ((screen_height as f32)*(1./13.)) as c_int,
                            colors::RED
                        );
                        DrawText(
                            raylib_str!(format!("{n}")),
                            ((screen_width as f32)*(2./5.)) as c_int,
                            ((screen_height as f32)*(4./13.)) as c_int,
                            ((screen_height as f32)*(2./13.)) as c_int,
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
