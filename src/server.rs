
use simplelog::*;
use std::time::{Instant, Duration};
use std::thread;
use std::sync::{RwLock, Arc};
use std::sync::mpsc::channel;
use rayon;

pub mod game;
pub mod io;
pub mod particles;
pub mod simulation;

use game::{World, WriteState};
use particles::{Particle, ParticleBlock};

pub fn run() {
    let _ = SimpleLogger::init(LevelFilter::Debug, Config::default());
         
    let (msg_in_sender, msg_in_receiver) = channel();
    let msg_in = io::InboundMessages::new(msg_in_sender);
    let msg_out = io::OutboundMessages::new();
    let mut read_world = Arc::new(RwLock::new(World::new()));
    let mut write_world = Arc::new(RwLock::new(World::new()));
    let pool = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap();

    // DEBUG
    // for i in 0..16 {
    //     for j in 0..16 {
    //         read_world.write().unwrap().set_particle((i * 16, 0), Particle::Sand, true);
    //     }
    // }
    read_world.write().unwrap().set_particle((0,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((1,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((2,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((3,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((4,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((5,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((6,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((7,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((8,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((9,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((10,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((11,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((12,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((13,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((14,0), Particle::Sand, true);
    // read_world.write().unwrap().set_particle((15,0), Particle::Sand, true);



    let fps = 60;
    let frame_sleep = Duration::from_millis(1000);
    //let frame_sleep = Duration::from_millis(1 / fps);

    thread::spawn(move || {
        msg_in.start_listening();
    });

    let mut clients = Vec::new();
    loop {
        let frame_start = Instant::now();

        // QUEUE INPUTS
        loop {
            match msg_in_receiver.try_recv() {
                Ok(msg) => {
                    // TODO switch based on message content
                    info!("New client connected {}", msg.src_addr);
                    read_world.write().unwrap().set_particle((0,0), Particle::Sand, true);
                    clients.push(msg.src_addr);
                },
                Err(_) => {
                    break;
                }
            }
        }

        // UPDATE
        let mut updated_write_stats = Vec::<WriteState>::new();
        let mut dirty_block_pos = Vec::new();
        for (pos, block) in read_world.read().unwrap().all_blocks() {
            if block.is_dirty() {
                dirty_block_pos.push(pos.clone());
            }
            else{
                // clone unchanged blocks into write state
                write_world.write().unwrap().set_block(block.clone());
            }
        }


        let mut completed_writes = Vec::new();
        for pos in dirty_block_pos {
            let read_c = read_world.clone();
            let write_block = ParticleBlock::new(pos.clone());
            let jh = pool.install(move || {
                let mut write = WriteState::new(write_block);
                update(&read_c.read().unwrap(), &mut write);
                write
            });
            completed_writes.push(jh);                
        }   
        
        for w in completed_writes {
            //let updated : WriteState = jh.join().unwrap();
            updated_write_stats.push(w);
        }

        for write_state in updated_write_stats {
            // TODO handle any cross-block movement and messages generated e.t.c
            let (finished_block, cross_block) = write_state.finish();
            let mut ww = write_world.write().unwrap();
            ww.set_block(finished_block);
            for (pos, particle) in cross_block {
                ww.set_particle(pos, particle, true);
            }
        }

        // SEND
        msg_out.send_world(&clients, &write_world.read().unwrap());

        // SWAP
        let tmp = read_world;
        read_world = write_world;
        write_world = tmp;


        // SLEEP
        let frame_end = frame_start.elapsed();
        debug!("Frame end {:?}micros", frame_start.elapsed().as_micros());
        if frame_end < frame_sleep {
            std::thread::sleep(frame_sleep - frame_end);
        }

    }
}

fn update(read: &World, write: &mut WriteState) {
    trace!("Updating {:?} {:?}", write.get_block_pos(), thread::current().id());
    for (pos, particle) in read.get_block(write.get_block_pos()).unwrap().all_particles() {
        simulation::update_particle(pos, particle, read, write);
    }
}
