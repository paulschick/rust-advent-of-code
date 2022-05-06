use std::env;
use env_logger::{Builder, Target};

#[macro_use]
extern crate log;

fn init_logger() {
    // log without calling from command line
    env::set_var("RUST_LOG", "info");
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}

fn main() {
    init_logger();
    info!("Starting");
    info!("hello");
    warn!("warning you");
}
