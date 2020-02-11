use rand::prelude::*;

use crate::game::{World, WriteState};
use crate::particles::{Particle, ParticleBlock};

pub fn update_particle(global_pos: (i32, i32), particle : Particle, read: &World, write: &mut WriteState) {
    let (x, y) = global_pos;
    match particle {
        Particle::Sand => {
            let new_y = y + 1;
            let choices = vec![-1, 0, 1];
            let sideways = choices.choose(&mut thread_rng()).unwrap();
            let new_x = x + sideways;
            if read.is_empty((x, new_y)) {
                write.set_cell(particle, (x, new_y), true);
                // TODO if cell was on a boundary mark the neighbour block dirty
            }
            else if read.is_empty((new_x, new_y)) {
                 write.set_cell(particle, (new_x, new_y), true); 
                 // TODO if cell was on a boundary mark the neighbour block dirty
            }
            else {
                write.set_cell(particle, (x, y), false);
                // On the ground
            }
        },
        Particle::Air => {

        }
    }
}