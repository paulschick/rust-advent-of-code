#![allow(dead_code)]
#![allow(unused_variables)]

mod instructions;

//use crate::instructions::{Instruction, parse_instructions};
use crate::instructions::{parse_instructions};

/// https://adventofcode.com/2021/day/2
/// use input.txt

fn main() {
    let my_vecs = parse_instructions();
    for v in my_vecs {
        println!("{:?}", v);
    }
}
