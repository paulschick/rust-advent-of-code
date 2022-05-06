#![allow(dead_code)]
#![allow(unused_variables)]

mod instructions;
mod position;

use crate::instructions::{parse_instructions};
use crate::position::Position;

/// https://adventofcode.com/2021/day/2
/// use input.txt

fn main() {
    let my_vecs = parse_instructions();

    println!("Testing Position struct method");
    let mut pos = Position::new();
    println!("{:?}", pos);

    println!("updating struct cell values");
    pos.update_horizontal(10);
    pos.update_depth(104);

    println!("{:?}", pos);

    println!("Using Position parse_forward");
    let mag1: i8 = 5;
    println!("Adding {}", mag1);
    pos.parse_forward(mag1);

    println!("{:?}", pos);

    println!("Adding 7 to depth");

    let down1: i8 = 7;
    pos.parse_down(down1);
    println!("{:?}", pos);

    let up1: i8 = 6;
    println!("Move up -> {}", up1);
    pos.parse_up(up1);
    println!("{:?}", pos);

    let instructions_slice = &my_vecs[..];
    pos.parse_instructions_vec(instructions_slice);
}
