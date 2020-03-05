
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
use crate::particle::Particle;

pub fn run() {
    let _ = SimpleLogger::init(LevelFilter::Debug, Config::default());
         
    let (msg_in_sender, msg_in_receiver) = channel();
    let msg_in = crate::io::InboundMessages::new("0.0.0.0:34254".to_owned(), msg_in_sender);
    let msg_out = crate::io::OutboundMessages::new("0.0.0.0:34255".to_owned());
    let mut read_world = Arc::new(RwLock::new(World::new()));
    let mut write_world = Arc::new(RwLock::new(World::new()));
    let pool = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap();   


    let fps = 30;
    //let frame_sleep = Duration::from_millis(1000);
    let frame_sleep = Duration::from_millis(1 / fps);

    thread::spawn(move || {
        msg_in.start_listening();
    });

    let mut clients = Vec::new();
    loop {
        let frame_start = Instant::now();

        // QUEUE INPUTS
        loop {
            match msg_in_receiver.try_recv() {
                Ok((msg, src_addr)) => {
                    match msg {
                        crate::io::Msg::NewClient{port} => {
                            info!("New client connected {}", src_addr);
                            let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
                            clients.push(client_addr);
                        },
                        crate::io::Msg::SetParticle{x, y, particle} => {
                            read_world.write().unwrap().set_particle((x, y), particle, true);
                        },
                        _ => {}
                    }                    
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
        io::send_world_updates(&clients, &msg_out, &(write_world.read().unwrap()));

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
