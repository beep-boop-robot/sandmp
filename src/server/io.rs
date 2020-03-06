use std::time::Instant;
use std::net::SocketAddr;

use super::game::World;
use crate::io::{OutboundMessages, Msg};


pub fn send_world_updates(clients: &Vec::<SocketAddr>, outbound: &OutboundMessages, world: &World) {
    let s = Instant::now();
    let mut msgs = Vec::new();
    for (_, block) in world.all_blocks() {
        if block.updated {
            msgs.push(Msg::TextureUpdate{x: block.get_pos().0, y: block.get_pos().1, data: block.get_texture().to_vec()});
        }        
    }  
    outbound.send(clients, &msgs);
        
    trace!("Serialize and send {:?}micros", s.elapsed().as_micros());
}
