#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
// use std::fs::{File, OpenOptions};
// use std::io::{Write, Error};
use env_logger::{Builder, Target};

// #[macro_use]
// extern crate log;

fn init_logger() {
    // log without calling from command line
    env::set_var("RUST_LOG", "info");
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}

// fn write_log() -> Result<(), Error> {
//     let path = "test-log.log";
//     let mut output = File::create(path)?;
//     write!(output, "Hello from this program\n")?;
//
//     return Ok(());
// }
//
// fn append_file() {
//     let mut file = OpenOptions::new()
//         .append(true)
//         .open("test-log.log")
//         .expect("unable to open file");
//     file.write_all("Hello from a new line\n".as_bytes())
//         .expect("write failed");
//     println!("Success");
// }

pub mod file_contents {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    pub fn file_input() -> Vec<String> {
        let file = File::open("input.txt").expect("File not found!!");
        let buf = BufReader::new(file);
        return buf
            .lines()
            .map(|l| l.expect("Unable to parse lines!"))
            .collect();
    }
}

pub mod nested_array {
    /// bool - 1 = true, 0 = false
    /// Implementation:
    /// https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    pub fn create_array(file_data: &Vec<String>) -> Vec<Vec<bool>> {
        let height = file_data.len();
        let first_val = &file_data[0];
        let width = first_val.len();
        let mut array = vec![vec![true; width]; height];
        let mut data_array: Vec<Vec<bool>> = vec![];

        for (i, row) in array.iter_mut().enumerate() {

            let current_string = &file_data[i];
            let mut line_vec: Vec<bool> = vec![];

            for (j, col) in row.iter_mut().enumerate() {
                let current_char = current_string.chars().nth(j).unwrap();
                if current_char == '1' {
                    line_vec.push(true);
                } else {
                    line_vec.push(false);
                }
            }
            data_array.push(line_vec);
        }
        return data_array;
    }
}

fn main() {
    init_logger();
    // info!("Starting");
    // info!("hello");
    // warn!("warning you");
    // write_log().unwrap();
    // append_file();
    println!("Getting file contents");
    let file_vec = file_contents::file_input();
    println!("{:?}", file_vec);
    println!("{}", file_vec.len());
    println!("{}", &file_vec[0]);

    let first_val = &file_vec[0];
    let width = first_val.len();
    println!("{}", width);

    let array = nested_array::create_array(&file_vec);
    println!("{:?}", array);
}
