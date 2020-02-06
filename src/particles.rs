use std::collections::HashMap;


pub const BLOCK_SIZE : i32 = 16;

#[derive(Copy, Clone)]
pub enum Particle {
    Air,
    Sand
}

pub struct ParticleBlock {
    pos: (i32, i32),
    dirty: bool,
    particles: HashMap<(i32, i32), Particle> // particles are stored by global position
}

impl ParticleBlock {

    pub fn new(pos: (i32, i32)) -> ParticleBlock {
        ParticleBlock {
            pos: pos,
            dirty: true, // TODO might need to be configurable
            particles: HashMap::new() 
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn get_blocks(&self) -> Vec<((i32, i32), Particle)> {
        self.particles.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

}
