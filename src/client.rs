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

const SCREEN_SIZE : u32 = 512;

pub fn run() {
    // NETWORK
    let (msg_in_sender, msg_in_receiver) = channel();
    let outbound_msg = io::OutboundMessages::new("0.0.0.0:34256".to_owned());
    let inbound_msg = io::InboundMessages::new("0.0.0.0:34257".to_owned(), msg_in_sender);
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 34254);
    let new_client_msg = io::Msg::NewClient{port: 34257};
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
    let mut target_tex = texture_creator.create_texture_target(PixelFormatEnum::RGB24, 16, 16).map_err(|x| x.to_string()).unwrap();    

    'running: loop {

        for event in event_pump.poll_iter() { 
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
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
    }
}