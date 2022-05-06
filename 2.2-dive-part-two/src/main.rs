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
        pub horizontal: Cell<i64>,
        pub depth: Cell<i64>,
        aim: Cell<i64>,
    }

    impl Position {
        pub fn new() -> Self {
            return Position {
                horizontal: Cell::new(0),
                depth: Cell::new(0),
                aim: Cell::new(0),
            };
        }

        pub fn parse_instructions(&mut self, instructions: &Vec<(String, i8)>) {
            let forward = String::from("forward");
            let up = String::from("up");
            let down = String::from("down");
            
            for ins in instructions {
                if ins.0.eq(&forward) {
                    println!("Forward -> {}", ins.1);
                    self.parse_forward(i64::from(ins.1));
                } else if ins.0.eq(&up) {
                    println!("Up -> {}", ins.1);
                    self.parse_up(i64::from(ins.1));
                } else if ins.0.eq(&down) {
                    println!("Down -> {}", ins.1);
                    self.parse_down(i64::from(ins.1));
                }
            }
        }

        /// increase horizontal position by val
        /// increases depth by aim * x
        /// make private
        pub fn parse_forward(&mut self, val: i64) {
            self.horizontal.set(self.horizontal.get() + val);
            self.depth.set(self.depth.get() + (val * self.aim.get()));
        }

        /// decreases aim by val
        /// make private
        pub fn parse_up(&mut self, val: i64) {
            self.aim.set(self.aim.get() - val);
        }

        /// increases aim by val
        /// make private
        pub fn parse_down(&mut self, val: i64) {
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

    pos.parse_instructions(&directions);

    println!("Final Values");
    println!("{:?}", pos);

    let mult = pos.horizontal.get() * pos.depth.get();
    println!("Multiple value: {}", mult);
}
