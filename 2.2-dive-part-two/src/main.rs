#![allow(dead_code)]
#![allow(unused_variables)]

pub mod file_contents {
    use std::fs::{File};
    use std::io::{prelude::*, BufReader, stdin, stdout};
    pub fn file_input() -> Vec<String> {
        println!("File name: ");
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Incorrect string format");
        if let Some('\n') = s.chars().next_back(){
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let file = read_file_contents(&s);
        return file;
    }

    fn read_file_contents(filename: &String) -> Vec<String> {
        let file = File::open(filename).expect("no file");
        let buf = BufReader::new(file);
        return buf.lines()
            .map(|l| l.expect("Unable to parse lines."))
            .collect();
    }
}

pub mod instructions {
    // use a tuple now instead of struct
    pub fn create_instructions_vec(directions: &Vec<String>) -> Vec<(String, i8)> {
        let mut data: Vec<(String, i8)> = Vec::new();
        for line in directions {
            let strings: Vec<&str> = line.split(" ").collect();
            let tup = (strings[0].to_string(), strings[1].to_string().parse::<i8>().unwrap());
            data.push(tup);
        }

        return data;
    }
}

pub mod position_management {
    use std::cell::Cell;

    #[derive(Debug)]
    pub struct Position {
        pub horizontal: Cell<i16>,
        pub depth: Cell<i16>,
        aim: Cell<i16>,
    }

    impl Position {
        pub fn new() -> Self {
            return Position {
                horizontal: Cell::new(0),
                depth: Cell::new(0),
                aim: Cell::new(0),
            };
        }

        /// increase horizontal position by val
        /// increases depth by aim * x
        /// make private
        pub fn parse_forward(&mut self, val: i16) {
            self.horizontal.set(self.horizontal.get() + val);
            self.depth.set(self.depth.get() + (val * self.aim.get()));
        }

        /// decreases aim by val
        /// make private
        pub fn parse_up(&mut self, val: i16) {
            self.aim.set(self.aim.get() - val);
        }

        /// increases aim by val
        /// make private
        pub fn parse_down(&mut self, val: i16) {
            self.aim.set(self.aim.get() + val);
        }
    }
}

fn main() {
    use crate::position_management::Position;
    
    let file = file_contents::file_input();
    let directions = instructions::create_instructions_vec(&file);
    println!("{:?}", directions);

    let mut pos = Position::new();
    println!("{:?}", pos);

    pos.parse_forward(10i16);
    println!("{:?}", pos);
    pos.parse_down(10i16);
    println!("{:?}", pos);
    pos.parse_forward(8i16);
    println!("{:?}", pos);
    pos.parse_up(4i16);
    println!("{:?}", pos);
    pos.parse_forward(8i16);
    println!("{:?}", pos);
}
