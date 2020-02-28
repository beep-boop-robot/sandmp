use std::collections::HashMap;
use std::cmp::min;
use super::particles::{Particle, ParticleBlock, BLOCK_SIZE, BLOCKS_IN_WORLD_ROW};

pub struct World {
    blocks: HashMap<(i32, i32), ParticleBlock> // blocks are stored by global pos
}

impl World {

    pub fn new() -> World {
        let mut blocks = HashMap::new();
        for i in 0..BLOCKS_IN_WORLD_ROW {
            for j in 0.. BLOCKS_IN_WORLD_ROW {
                let (x, y) = (i * BLOCK_SIZE, j * BLOCK_SIZE);
                blocks.insert((x, y), ParticleBlock::new((x, y)));
            }
        }
        World {
            blocks
        }
    }

    pub fn set_block(&mut self, block: ParticleBlock) {
        self.blocks.insert(block.get_pos().clone(), block);
    }

    pub fn get_block(&self, global_pos: (i32, i32)) -> Option<&ParticleBlock> {
        let floored = (global_pos.0 - (global_pos.0 % BLOCK_SIZE), global_pos.1 - (global_pos.1 % BLOCK_SIZE));
        self.blocks.get(&floored)
    }

    pub fn get_block_mut(&mut self, global_pos: (i32, i32)) -> Option<&mut ParticleBlock> {
        let floored = (global_pos.0 - (global_pos.0 % BLOCK_SIZE), global_pos.1 - (global_pos.1 % BLOCK_SIZE));
        self.blocks.get_mut(&floored)
    }

    pub fn all_blocks(&self) -> &HashMap<(i32, i32), ParticleBlock> {
        &self.blocks
    }

    pub fn get_particle(&self, global_pos: (i32, i32)) -> &Particle {
        let block = self.get_block(global_pos);
        match block {
            Some(block) => {
                let pos_in_block = (global_pos.0 % BLOCK_SIZE, global_pos.1 % BLOCK_SIZE);
                block.get_particle(pos_in_block)
            }
            None => {
                &Particle::Boundary
            }
        }
    }

    pub fn set_particle(&mut self, global_pos: (i32, i32), particle: Particle, mark_dirty: bool) {
        match self.get_block_mut(global_pos) {
            Some(block) => {
                let pos_in_block = (global_pos.0 % BLOCK_SIZE, global_pos.1 % BLOCK_SIZE);
                block.set_particle(pos_in_block, particle, mark_dirty);
            },
            None => {
                // TODO do something useful
                let x = 0;
            }
        }
    }

    pub fn is_empty(&self, global_pos: (i32, i32)) -> bool {
        self.get_particle(global_pos) == &Particle::Air
    }
}

pub struct WriteState {
    active_block: ParticleBlock,
    cross_block_moves: Vec::<((i32, i32), Particle)>, 
}


impl WriteState {

    pub fn new(mut particle_block: ParticleBlock) -> WriteState {
        particle_block.updated = true;
        WriteState {
            active_block: particle_block,
            cross_block_moves: Vec::new()
        }
    }

    // TODO would this produce a new block via a method that gets called later (after update)?
    pub fn get_block_pos(&self) -> (i32, i32) {
        self.active_block.get_pos()
    }

    pub fn finish(self) -> (ParticleBlock, Vec::<((i32, i32), Particle)>) {
        (self.active_block, self.cross_block_moves)
    }

    pub fn set_cell(&mut self, particle: Particle, global_pos: (i32, i32), mark_dirty: bool) {
        // TODO is the cell in the current block?
        // TODO if block should be marked dirty and cell is a boundary mark the neighburing block(s) dirty
        // if yes. write it
        // if no, write it too the queue
        let block_pos = self.active_block.get_pos();
        let is_in_block = global_pos.0 >= block_pos.0 &&
             global_pos.0 < block_pos.0 + BLOCK_SIZE &&
             global_pos.1 >= block_pos.1 &&
             global_pos.1 < block_pos.1 + BLOCK_SIZE;
        if is_in_block {
            let pos_in_block = (global_pos.0 - block_pos.0, global_pos.1 - block_pos.1);
            self.active_block.set_particle(pos_in_block, particle, mark_dirty);
        }
        else {
            // TODO write to queue to resolve later. or lock the global world state and write the cell
            trace!("Attempted to set outside of current block. Resetting to block limit");
            self.cross_block_moves.push((global_pos, particle));
        }
    }
}