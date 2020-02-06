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

    pub fn get_block(&self, global_pos: (i32, i32)) -> &ParticleBlock {
        let floored = (global_pos.0 - (global_pos.0 % BLOCK_SIZE), global_pos.1 - (global_pos.1 % BLOCK_SIZE));
        self.blocks.get(&floored).unwrap()
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

    pub fn set_cell(&mut self, global_pos: (i32, i32)) {
        // TODO is the cell in the current block?
        // if yes. write it
        // if no, write it too the queue
    }
}