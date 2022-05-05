use std::fs::{File};
use std::io::{Read, Result};


/// https://adventofcode.com/2021/day/1
///
/// Antenna strength: 0 - 50
/// Must get 50 starts by 12/25 (Some Christmas stuff)
/// - Collect by solving puzzles
/// - Two are available each day in the Advent calendar
/// - Second puzzle unlocked after completing the first
/// - One challenge = one star
///
/// Step 1:
///         I need to read the input from the input.txt file. That's going to be my first challenge
///          personally...
///

fn read_input_file() -> Result<Vec<u8>> {
    let mut file = File::open("input.txt")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    return Ok(data);
}

fn main() {
    let data = read_input_file();
    println!("{:?}", data);
}
