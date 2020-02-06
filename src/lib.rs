extern crate futures;
#[macro_use]extern crate log;
extern crate simplelog;
extern crate rayon;

use simplelog::*;

// use async_std::task;
use log::{debug};
use crossbeam;
use std::time::{Instant, Duration};
use std::thread;
use std::sync::Arc;

mod particles;
mod game;

use game::{World, WriteState};
use particles::{Particle, ParticleBlock};

pub fn run() {
    // for every block in read_state
    // if not dirty. copy it over to write state
    // else. create a new block_write_state and update the block

    let _ = SimpleLogger::init(LevelFilter::Trace, Config::default());
            

    let mut read = Arc::new(World::new());
    let mut write = Arc::new(World::new());

    let fps = 60;
    let frame_sleep = 1 / fps;

    loop {
        let frame_start = Instant::now();

        // QUEUE INPUTS

        // UPDATE
        crossbeam::thread::scope(|s| { // TODO probably don't actually need crossbeam
            let mut jhandles = Vec::new();
            for dirty_pos in read.get_dirty_pos() {
                let read_c = read.clone();
                let pos = dirty_pos.clone();
                let jh = s.spawn(move |_| {
                    let mut write = WriteState::new(pos);
                    update(&read_c, &mut write);
                });
                jhandles.push(jh);
            }      
            
            for jh in jhandles {
                let _ = jh.join();
            }
        }).unwrap();

        // SEND

        // SLEEP
        let frame_end = frame_start.elapsed();
        trace!("Frame end {:?}micros", frame_start.elapsed().as_micros());
        if frame_end < Duration::from_millis(frame_sleep) {
            std::thread::sleep(Duration::from_millis(frame_sleep) - frame_end);
        }

    }
}

fn update(read: &World, write: &mut WriteState) {
    trace!("Updating {:?} {:?}", write.get_block_pos(), thread::current().id());
    for (pos, particle) in read.get_block(write.get_block_pos()).get_blocks() {
        // TODO update 
        update_particle(pos, particle, read, write);
    }
}


pub fn update_particle(global_pos: (i32, i32), particle : Particle, read: &World, write: &WriteState) {

}
