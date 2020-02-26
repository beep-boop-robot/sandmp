use std::time::{Instant, Duration};
use std::sync::mpsc::{Receiver, Sender};
use std::net::{UdpSocket, SocketAddr};

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

//use super::game::World;
//use super::particles::BLOCK_SIZE;

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg{
    NewClient,
    TextureUpdate{
        x: i32,
        y: i32,
        data: Vec::<u8>
    }
}

// TODO move back into server code and keep common IO msg here?
pub struct InboundMessages {
    msg_in_sender: Sender<(Msg, SocketAddr)>,
    socket: UdpSocket
}

impl InboundMessages {

    pub fn new(addr: String, msg_in_sender: Sender<(Msg, SocketAddr)>) -> InboundMessages {
        let socket = UdpSocket::bind(addr).unwrap();
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
                    let msg: Msg = rmp_serde::from_read_ref(&buf[..]).unwrap();
                    let data = buf[..buf_size].to_vec();
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

    pub fn new(addr: String) -> OutboundMessages {
        let socket = UdpSocket::bind(addr).unwrap();
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
         
         
        debug!("Serialize and send {:?}micros", s.elapsed().as_micros());
    }
}
