use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, Error};
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

fn write_log() -> Result<(), Error> {
    let path = "test-log.log";
    let mut output = File::create(path)?;
    write!(output, "Hello from this program\n")?;

    return Ok(());
}

fn append_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("test-log.log")
        .expect("unable to open file");
    file.write_all("Hello from a new line\n".as_bytes())
        .expect("write failed");
    println!("Success");
}

fn main() {
    init_logger();
    info!("Starting");
    info!("hello");
    warn!("warning you");
    write_log().unwrap();
    append_file();
}
