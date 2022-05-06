use std::cell::Cell;
use crate::instructions::Instruction;

#[derive(Debug)]
pub struct Position {
    horizontal: Cell<i16>,
    depth: Cell<i16>,
}

impl Position {
    pub fn new() -> Self {
        return Position {
            horizontal: Cell::new(0),
            depth: Cell::new(0),
        };
    }

    pub fn update_horizontal(&mut self, val: i16) {
        self.horizontal.set(val);
    }

    pub fn update_depth(&mut self, val: i16) {
        self.depth.set(val);
    }

    pub fn parse_forward(&mut self, val: i8) {
        self.horizontal.set(self.horizontal.get() + i16::from(val));
    }

    pub fn parse_down(&mut self, val: i8) {
        self.depth.set(self.depth.get() + i16::from(val));
    }

    pub fn parse_up(&mut self, val: i8) {
        self.depth.set(self.depth.get() - i16::from(val));
    }

    pub fn parse_instructions_vec(&mut self, instructions: &[Instruction]) {
        let forward = String::from("forward");
        let up = String::from("up");
        let down = String::from("down");

        println!("Starting Values");
        println!("{:?}", self);
        
        for ins in instructions.iter() {
            if ins.direction.eq(&forward) {
                println!("found a forward -> {}: {}", ins.direction, ins.magnitude);
                self.parse_forward(ins.magnitude);
                println!("{:?}", self);
            } else if ins.direction.eq(&up) {
                println!("Found an up!! -> {}: {}", ins.direction, ins.magnitude);
                self.parse_up(ins.magnitude);
                println!("{:?}", self);
            } else if ins.direction.eq(&down) {
                println!("Found a Down!! -> {}: {}", ins.direction, ins.magnitude);
                self.parse_down(ins.magnitude);
                println!("{:?}", self);
            }
        }

        println!("Final Values:");
        println!("{:?}", self);
        let horiz: i64 = i64::from(self.horizontal.get());
        let depth_64: i64 = i64::from(self.depth.get());
        println!("horiz: {}, depth: {}", horiz, depth_64);
        println!("{}", horiz * depth_64);
    }
}
