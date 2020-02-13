use std::collections::HashMap;

pub const BLOCK_SIZE : i32 = 16;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Particle {
    Air,
    Sand
}

#[derive(Clone)]
pub struct ParticleBlock {
    pos: (i32, i32),
    dirty: bool,
    particles: HashMap<(i32, i32), Particle>, // particles are stored by global position
    texture: [u8; (BLOCK_SIZE * BLOCK_SIZE) as usize] // maybe change to a dense array of enums?
}

impl ParticleBlock {

    pub fn new(pos: (i32, i32)) -> ParticleBlock {
        ParticleBlock {
            pos: pos,
            dirty: false,
            particles: HashMap::new(),
            texture: [0; (BLOCK_SIZE * BLOCK_SIZE) as usize]
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn all_particles(&self) -> Vec<((i32, i32), Particle)> {
        self.particles.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    pub fn get_particle(&self, pos_in_block: (i32, i32)) -> &Particle {
        // TODO option or Air on miss
        self.particles.get(&pos_in_block).unwrap_or(&Particle::Air)
    }

    pub fn set_particle(&mut self, pos_in_block: (i32, i32), particle: Particle, mark_dirty: bool) {
        self.particles.insert(pos_in_block, particle);
        self.dirty = mark_dirty;

        // TODO look up color
        let idx = (pos_in_block.0 + (pos_in_block.1 + BLOCK_SIZE)) as usize;
        self.texture[idx] = 255;
        self.texture[idx+1] = 255;
        self.texture[idx+2] = 255;
    }

    pub fn get_texture(&self) -> &[u8] {
        &self.texture
    }
}
