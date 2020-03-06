use std::time::{Instant, Duration};
use std::sync::mpsc::{Receiver, Sender};
use std::net::{UdpSocket, SocketAddr, TcpStream};

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

use crate::particle::Particle;

#[derive(Serialize, Deserialize)]
pub enum Msg{
    NewClient{
        port: u16
    },
    TextureUpdate{
        x: i32,
        y: i32,
        data: Vec::<u8>
    },
    SetParticle{
        x: i32,
        y: i32,
        particle: Particle
    }
}

// TODO move back into server code and keep common IO msg here?
pub struct InboundMessages {
    msg_in_sender: Sender<(Msg, SocketAddr)>,
    socket: UdpSocket
}

impl InboundMessages {

    pub fn new(socket: UdpSocket, msg_in_sender: Sender<(Msg, SocketAddr)>) -> InboundMessages {
        InboundMessages {
            msg_in_sender,
            socket
        }
    }

    pub fn start_listening(&self){
        loop {
            let mut buf = [0; 512];
            match self.socket.recv_from(&mut buf) {
                Ok((buf_size, src_addr)) => {
                    let msg: Msg = rmp_serde::from_read_ref(&buf[..buf_size]).unwrap();
                    self.msg_in_sender.send((msg, src_addr));
                },
                Err(_) => {

                }
            }
        }        
    }
}

pub struct OutboundMessages {
    socket: UdpSocket
} 

impl OutboundMessages {

    pub fn new(socket: UdpSocket) -> OutboundMessages {
        OutboundMessages {
            socket
        }
    }


    pub fn send(&self, clients: &Vec::<SocketAddr>, msgs: &Vec::<Msg>) {
        let s = Instant::now();  
        for msg in msgs {
            let mut buf = Vec::new();
            msg.serialize(&mut Serializer::new(&mut buf)).unwrap();
            for client in clients {
                self.socket.send_to(&buf, client);
            } 
        }
    }
}
