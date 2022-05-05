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
/// Step 2:
///         Read the number of times that the number increases from the previous number.
///         Okay so this is more challenging (when you don't know rust) than it sounds like.
///
/// Current count -> mutable integer
/// Previous number -> mutable integer
/// Could use recursion through the vector. Have a function (parent function) hold the mutable
/// variables. This then calls another function recursively. That function will return a boolean.
///     True -> number increased
///     False -> number did not increase
///

fn read_input_file() -> Result<Vec<u8>> {
    let mut file = File::open("input.txt")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    return Ok(data);
}

fn count_increases(vec: &Vec<u8>) -> i32 {
    let mut count = 0;
    for i in 0..vec.len() {
        let current_value = vec[i];
        println!("Test current value");
        println!("{}", current_value);
        if i != 0 {
            let previous_value = vec[i - 1];
            println!("Current: {}, Previous: {}", current_value, previous_value);
            if current_value > previous_value {
                count += 1;
            }
        }
    }

    return count;
}

fn main() {
    let data = read_input_file();
    let count = count_increases(&data.unwrap());
    println!("Final Count: {}", count);
}
