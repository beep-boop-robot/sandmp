use sandmp::server;
use sandmp::client;
use std::env;

fn main() {
    if env::args().count() > 1 {
        client::run();
    }
    else {
        server::run();
    }
}
