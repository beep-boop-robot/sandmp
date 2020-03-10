use std::net::{IpAddr, Ipv4Addr, UdpSocket, SocketAddr, TcpStream};
use std::sync::mpsc::channel;
use std::thread;
use std::io::{Read, Write};

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Instant, Duration};
use sdl2::pixels::PixelFormatEnum;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

use crate::particle::Particle;
use crate::msg::Msg;

const SCREEN_SIZE : u32 = 512;
const TEXTURE_SIZE : u32 = 64;
const MOUSE_RATIO : f32 = (SCREEN_SIZE / TEXTURE_SIZE) as f32;

pub fn run() {
    // NETWORK
    let mut server_stream = TcpStream::connect("0.0.0.0:34254").unwrap();
    server_stream.set_nonblocking(true);

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
            let mut buf = Vec::new();
            let msg = Msg::SetParticle{x: cursor_pos.0, y: cursor_pos.1, particle: Particle::Sand};
            msg.serialize(&mut Serializer::new(&mut buf)).unwrap();
            
            server_stream.write(&buf).unwrap();
            server_stream.flush().unwrap();
            input_sleep = Duration::from_millis(0);
        }
        
        let mut buf = Vec::new();
        match server_stream.read_to_end(&mut buf){
            Ok(n) => {
                let msg: Msg = rmp_serde::from_read_ref(&buf[..n]).unwrap();
                match msg {
                    Msg::TextureUpdate{x, y, data} => {
                        let r = Rect::new(x, y, 8, 8);
                        target_tex.update(r, &data, 8 * 3);
                    },
                    _ => {}
                }
            },
            Err{..} => {}
        }

        canvas.clear();
        canvas.copy(&target_tex, None, None);
        canvas.present();
        input_sleep += frame_start.elapsed();
    }
}