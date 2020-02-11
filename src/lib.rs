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
mod simulation;

use game::{World, WriteState};
use particles::{Particle, ParticleBlock};

pub fn run() {
    // for every block in read_state
    // if not dirty. copy it over to write state
    // else. create a new block_write_state and update the block

    let _ = SimpleLogger::init(LevelFilter::Trace, Config::default());
            

    let mut read = Arc::new(World::new());
    let mut write = Arc::new(World::new());

    // DEBUG
    read.set_particle((1,0), Particle::Sand, true);

    let fps = 60;
    let frame_sleep = 1 / fps;

    loop {
        let frame_start = Instant::now();

        // QUEUE INPUTS

        // UPDATE
        let mut updated_write_stats = Vec::new();
        crossbeam::thread::scope(|s| { // TODO probably don't actually need crossbeam
            let mut jhandles = Vec::new();
            // TODO iterate all blocks. queue dirty for update. clone unchanged ones
            for dirty_pos in read.get_dirty_pos() { 
                let read_c = read.clone();
                let pos = dirty_pos.clone();
                let jh = s.spawn(move |_| {
                    let mut write = WriteState::new(pos);
                    let updated_write = update(&read_c, write);
                    updated_write
                });
                jhandles.push(jh);
            }      
            
            for jh in jhandles {
                let updated = jh.join().unwrap();
                updated_write_stats.push(updated);
            }
        }).unwrap();

        for _ in updated_write_stats {
            // TODO handle any cross-block movement and messages generated e.t.c
        }

        // SWAP
        let tmp = read;
        read = write;
        write = tmp;

        // SEND
        // look at the writestates and send each to the each client

        // SLEEP
        let frame_end = frame_start.elapsed();
        trace!("Frame end {:?}micros", frame_start.elapsed().as_micros());
        if frame_end < Duration::from_millis(frame_sleep) {
            std::thread::sleep(Duration::from_millis(frame_sleep) - frame_end);
        }

    }
}

fn update(read: &World, write: WriteState) -> WriteState {
    trace!("Updating {:?} {:?}", write.get_block_pos(), thread::current().id());
    for (pos, particle) in read.get_block(write.get_block_pos()).unwrap().all_particles() {
        simulation::update_particle(pos, particle, read, &mut write);
    }
    write
}
