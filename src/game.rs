use std::collections::HashMap;
use crate::particles::{Particle, ParticleBlock, BLOCK_SIZE};


pub struct World {
    blocks: HashMap<(i32, i32), ParticleBlock> // blocks are stored by global pos
}

impl World {

    pub fn new() -> World {
        let mut blocks = HashMap::new();
        blocks.insert((0, 0), ParticleBlock::new((0, 0)));
        blocks.insert((0, BLOCK_SIZE), ParticleBlock::new((0, BLOCK_SIZE)));
        World {
            blocks
        }
    }

    pub fn get_dirty_pos(&self) -> Vec<(i32, i32)> {
        let mut write_states = Vec::new();
        for block in self.blocks.values() {
            if block.is_dirty() {
                write_states.push(block.get_pos());
            }
        }
        write_states
    }

    pub fn get_block(&self, global_pos: (i32, i32)) -> Option<&ParticleBlock> {
        let floored = (global_pos.0 - (global_pos.0 % BLOCK_SIZE), global_pos.1 - (global_pos.1 % BLOCK_SIZE));
        self.blocks.get(&floored)
    }

    pub fn get_particle(&self, global_pos: (i32, i32)) -> &Particle {
        let block = self.get_block(global_pos);
        match block {
            Some(block) => {
                let pos_in_block = (global_pos.0 % BLOCK_SIZE, global_pos.1 % BLOCK_SIZE);
                block.get_particle(pos_in_block)
            }
            None => {
                &Particle::Air // TODO maybe boundary particle
            }
        }
    }

    pub fn set_particle(&mut self, global_pos: (i32, i32), particle: Particle, mark_dirty: bool) {
        match self.get_block(global_pos) {
            Some(block) => {
                let pos_in_block = (global_pos.0 % BLOCK_SIZE, global_pos.1 % BLOCK_SIZE);
                block.set_particle(pos_in_block, particle, mark_dirty);
            },
            None => {
                // TODO do something useful
            }
        }
    }

    pub fn is_empty(&self, global_pos: (i32, i32)) -> bool {
        self.get_cell(global_pos) == &Particle::Air
    }
}

pub struct WriteState {
    active_block: ParticleBlock
}


impl WriteState {

    pub fn new(pos : (i32, i32)) -> WriteState {
        WriteState {
            active_block: ParticleBlock::new(pos)
        }
    }

    // TODO would this produce a new block via a method that gets called later (after update)?
    pub fn get_block_pos(&self) -> (i32, i32) {
        self.active_block.get_pos()
    }

    pub fn set_cell(&mut self, particle: Particle, global_pos: (i32, i32), mark_dirty: bool) {
        // TODO is the cell in the current block?
        // TODO if block should be marked dirty and cell is a boundary mark the neighburing block(s) dirty
        // if yes. write it
        // if no, write it too the queue
        let block_pos = self.active_block.get_pos();
        let pos_in_block = ((global_pos.0 % block_pos.0), (global_pos.1 % block_pos.1));
        let is_in_block = pos_in_block.0 < BLOCK_SIZE && pos_in_block.1 < BLOCK_SIZE;
        if is_in_block {
            self.active_block.set_particle(pos_in_block, particle, mark_dirty);
        }
        else {
            // TODO write to queue to resolve later. or lock the global world state and write the cell
        }
    }
}