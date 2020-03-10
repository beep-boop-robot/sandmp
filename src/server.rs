
use simplelog::*;
use std::time::{Instant, Duration};
use std::net::{IpAddr, Ipv4Addr, UdpSocket, SocketAddr};
use std::thread;
use std::sync::{RwLock, Arc};
use std::sync::mpsc::channel;
use rayon;

pub mod game;
pub mod io;
pub mod particles;
pub mod simulation;

use game::{World, WriteState};
use particles::{ParticleBlock};
use crate::msg::Msg;

pub fn run() {
    let _ = SimpleLogger::init(LevelFilter::Debug, Config::default());
         
    let mut client_listener = crate::server::io::TcpConnectionHandler::new("0.0.0.0:34254".to_owned());
    let mut read_world = Arc::new(RwLock::new(World::new()));
    let mut write_world = Arc::new(RwLock::new(World::new()));
    let pool = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap();   


    let fps = 30;
    let mut frame_times = Vec::new();
    let mut time_since_log = Duration::from_millis(0);
    let frame_sleep = Duration::from_millis(1 / fps);

    let mut msgs_in = Vec::new();
    loop {
        let frame_start = Instant::now();

        // QUEUE INPUTS
        for msg in msgs_in {
            match msg {
                Msg::SetParticle{x, y, particle} => {
                    read_world.write().unwrap().set_particle((x, y), particle, true);
                },
                _ => {}
            }           
        }

        // UPDATE
        read_world.write().unwrap().clear_update_flags();
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


        let mut crossblocks = Vec::new();
        for write_state in updated_write_stats {
            // TODO handle any cross-block movement and messages generated e.t.c
            let (finished_block, cross_block) = write_state.finish();
            let mut ww = write_world.write().unwrap();
            ww.set_block(finished_block);
            crossblocks.extend(cross_block);
        }
        for (pos, particle) in crossblocks {
            let mut ww = write_world.write().unwrap();
            ww.set_particle(pos, particle, true);
        }

        // SEND
        let mut msgs = Vec::new();
        for (_, block) in write_world.read().unwrap().all_blocks() {
            if block.updated {
                msgs.push(Msg::TextureUpdate{x: block.get_pos().0, y: block.get_pos().1, data: block.get_texture().to_vec()});
            }
        }
        msgs_in = client_listener.tick(msgs);

        // SWAP
        let tmp = read_world;
        read_world = write_world;
        write_world = tmp;

        // SLEEP
        time_since_log += frame_start.elapsed();
        let frame_end = frame_start.elapsed();
        frame_times.push(frame_end);
        if time_since_log >= Duration::from_millis(1000) {
            let sum : u128 = frame_times.iter().map(|x| x.as_micros()).sum();
            let avg : u128 = sum / (frame_times.len() as u128);
            debug!("Frame update avg {:?}micros", avg);
            time_since_log = Duration::from_millis(0);
            frame_times.clear();
        }
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
