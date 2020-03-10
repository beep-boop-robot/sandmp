use std::time::Instant;
use std::net::SocketAddr;
use std::sync::mpsc::{Receiver, Sender};
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

use super::game::World;
use crate::msg::Msg;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};


pub struct TcpConnectionHandler {
    listener: TcpListener,
    clients: Vec::<TcpStream>
}

impl TcpConnectionHandler {
    pub fn new(addr: String) -> TcpConnectionHandler {
        let listener = TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true);
        TcpConnectionHandler {
            listener,
            clients: Vec::new()
        }
    }

    pub fn tick(&mut self, msgs_to_send: Vec::<Msg>) -> Vec::<Msg> {
        // Accept new clients
        'accept: loop {
            match self.listener.accept() {
                Ok((client, _)) => {
                    client.set_nonblocking(true);
                    self.clients.push(client);
                },
                Err{..} => {
                    break 'accept
                }
            }
        }

        let mut msgs= Vec::new();
        for client in self.clients.iter_mut() {
            // in
            let mut in_buf = Vec::new();
            let res = client.read_to_end(&mut in_buf);
            match res {
                Ok(n) => {
                    if n >  0 {
                        let msg: Msg = rmp_serde::from_read_ref(&in_buf).unwrap();
                        msgs.push(msg);
                    }
                },
                Err{..} => {}
            }

            //out
            for msg in msgs_to_send.iter() {
                let mut out_buf = Vec::new();
                msg.serialize(&mut Serializer::new(&mut out_buf)).unwrap();
                client.write(&out_buf);
                client.flush();
            }
        }

        msgs
    }
}