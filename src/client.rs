use std::net::{IpAddr, Ipv4Addr, UdpSocket, SocketAddr};
use std::sync::mpsc::channel;
use std::thread;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Instant, Duration};
use sdl2::pixels::PixelFormatEnum;
use rmp;

use crate::io;
use crate::particle::Particle;

const SCREEN_SIZE : u32 = 512;
const TEXTURE_SIZE : u32 = 64;
const MOUSE_RATIO : f32 = (SCREEN_SIZE / TEXTURE_SIZE) as f32;

pub fn run() {
    // NETWORK
    let (msg_in_sender, msg_in_receiver) = channel();
    let mut socket: UdpSocket;
    let mut client_port : u16;
    match std::net::UdpSocket::bind("0.0.0.0:34256".to_owned()) {
        Ok(s) => {
            socket = s;
            client_port = 34256;
        },
        Err{..} => {
            socket = std::net::UdpSocket::bind("0.0.0.0:34257".to_owned()).unwrap();
            client_port = 34257;
        }
    }
    let outbound_msg = io::OutboundMessages::new(socket.try_clone().unwrap());
    let inbound_msg = io::InboundMessages::new(socket.try_clone().unwrap(), msg_in_sender);
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 34254);
    let new_client_msg = io::Msg::NewClient{port: client_port};
    outbound_msg.send(&vec!(server_addr), &vec!(new_client_msg));

    thread::spawn(move || {
        inbound_msg.start_listening();
    });

    // WINDOW
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sand", SCREEN_SIZE, SCREEN_SIZE)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut target_tex = texture_creator.create_texture_target(PixelFormatEnum::RGB24, TEXTURE_SIZE, TEXTURE_SIZE).map_err(|x| x.to_string()).unwrap();    


    let mut input_sleep = Duration::from_millis(0);
    let mut cursor_pos = (0, 0);
    let mut mouse_down = false;
    'running: loop {
        let frame_start = Instant::now();
        for event in event_pump.poll_iter() { 
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::MouseMotion{x, y, ..} => {
                    cursor_pos = ((x as f32 / MOUSE_RATIO) as i32, (y as f32 / MOUSE_RATIO) as i32);
                },
                Event::MouseButtonDown{..} => {
                    mouse_down = true
                },
                Event::MouseButtonUp{..} => {
                    mouse_down = false
                },
                _ => {}
            }
        } 

        if (input_sleep >= Duration::from_millis(32) && mouse_down) {
            outbound_msg.send(&vec!(server_addr), &vec!(io::Msg::SetParticle{x: cursor_pos.0, y: cursor_pos.1, particle: Particle::Sand}));
            input_sleep = Duration::from_millis(0);
        }
        
        
        loop {
            match msg_in_receiver.try_recv() {
                Ok((msg, src_addr)) => {
                    match msg {
                        io::Msg::TextureUpdate{x, y, data} => {
                            let r = Rect::new(x, y, 8, 8);
                            target_tex.update(r, &data, 8 * 3);
                        },
                        _ => {}
                    }                    
                },
                Err(_) => {
                    break;
                }
            }
        }

        canvas.clear();
        canvas.copy(&target_tex, None, None);
        canvas.present();
        input_sleep += frame_start.elapsed();
    }
}