use std::net::{UdpSocket, SocketAddr};
use rmp;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Instant, Duration};
use sdl2::pixels::PixelFormatEnum;

const SCREEN_SIZE : u32 = 512;

pub fn run() {
    // NETWORK
    let socket = UdpSocket::bind("0.0.0.0:34256").unwrap();
    socket.send_to(&[5u8; 100], "0.0.0.0:34254");
    let mut buf = [0u8; 512];

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

    loop {
        match socket.recv_from(&mut buf) {
            Ok(_) => {
                let mut offset = 0;
                let x = rmp::decode::read_i32(&mut &buf[offset..offset+5]).unwrap();
                offset += 5;
                let y = rmp::decode::read_i32(&mut &buf[offset..offset+5]).unwrap();
                offset += 5;
                let len = rmp::decode::read_bin_len(&mut &buf[offset..offset+5]).unwrap() as usize;
                offset+=4;
                let texture = buf[offset..offset+len].to_vec();                
                println!("got data {} {} {:?}", x, y, texture);

                let r = Rect::new(x, y, 8, 8);
                target_tex.update(r, &texture, 8 * 3);
            },
            Err(_) => {

            }
        }        

        canvas.clear();
        canvas.copy(&target_tex, None, None);
        canvas.present();
    }
}