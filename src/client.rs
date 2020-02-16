use std::net::{UdpSocket, SocketAddr};
use rmp;

pub fn run() {
    let socket = UdpSocket::bind("0.0.0.0:34256").unwrap();
    socket.send_to(&[5u8; 100], "0.0.0.0:34254");
    let mut buf = [0u8; 512];
    loop {
        match socket.recv_from(&mut buf) {
            Ok(_) => {
                let x = rmp::decode::read_i32(&mut &buf[..]).unwrap();
                let y = rmp::decode::read_i32(&mut &buf[..]).unwrap();
                println!("got data {} {}", x, y);
            },
            Err(_) => {

            }
        }
    }
}