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
    let mut pos = Position::new();
    let instructions_slice = &my_vecs[..];
    pos.parse_instructions_vec(instructions_slice);
}
