use std::time::{Instant, Duration};

use crate::game::World;
use crate::particles::BLOCK_SIZE;

pub fn send_world(world: &World) {
    let s = Instant::now();
    let mut buf = Vec::new();
    for (_, block) in world.all_blocks() {
        if block.is_dirty() {
            rmp::encode::write_i32(&mut buf, block.get_pos().0);
            rmp::encode::write_i32(&mut buf, block.get_pos().1);
            rmp::encode::write_bin_len(&mut buf, (BLOCK_SIZE * BLOCK_SIZE) as u32);
            rmp::encode::write_bin(&mut buf, block.get_texture());
        }        
    }    
    debug!("Serialize {:?}micros", s.elapsed().as_micros());
}