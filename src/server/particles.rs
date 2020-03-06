use std::collections::HashMap;
use crate::particle::Particle;

pub const BLOCK_SIZE : i32 = 8;
pub const BLOCKS_IN_WORLD_ROW : i32 = 8;
pub const PARTICLES_IN_ROW : i32 = BLOCK_SIZE * BLOCKS_IN_WORLD_ROW;

#[derive(Clone)]
pub struct ParticleBlock {
    pos: (i32, i32),
    dirty: bool,
    particles: HashMap<(i32, i32), Particle>, // particles are stored by global position
    texture: [u8; (BLOCK_SIZE * BLOCK_SIZE * 3) as usize],
    pub updated: bool
}

impl ParticleBlock {

    pub fn new(pos: (i32, i32)) -> ParticleBlock {
        ParticleBlock {
            pos: pos,
            dirty: false,
            particles: HashMap::new(),
            texture: [0; (BLOCK_SIZE * BLOCK_SIZE * 3) as usize],
            updated: false
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    // returns global position
    pub fn all_particles(&self) -> Vec<((i32, i32), Particle)> {
        self.particles.iter().map(|(k, v)| ((k.0 + self.pos.0, k.1 + self.pos.1), v.clone())).collect()
    }

    pub fn get_particle(&self, pos_in_block: (i32, i32)) -> &Particle {
        // TODO option or Air on miss
        self.particles.get(&pos_in_block).unwrap_or(&Particle::Air)
    }

    pub fn set_particle(&mut self, pos_in_block: (i32, i32), particle: Particle, mark_dirty: bool) {
        self.particles.insert(pos_in_block, particle);
        if mark_dirty {
            self.dirty = true;
        }
        self.updated = true;

        // TODO look up color
        let (r, g, b) = get_color(particle); 
        let idx  = ((pos_in_block.0 * 3) + (pos_in_block.1 * (BLOCK_SIZE) * 3)) as usize;
        self.texture[idx] = r;
        self.texture[idx + 1] = g;
        self.texture[idx + 2] = b;
    }

    pub fn get_texture(&self) -> &[u8] {
        &self.texture
    }
}

pub fn get_color(particle: Particle) -> (u8, u8, u8) {
    match particle {
        Particle::Sand => (195, 195, 0),
        _ => (0,0,0)
    }
}
