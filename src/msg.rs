use std::time::{Instant, Duration};
use std::sync::mpsc::{Receiver, Sender};
use std::net::{UdpSocket, SocketAddr, TcpStream};

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

use crate::particle::Particle;

#[derive(Serialize, Deserialize)]
pub enum Msg{
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
