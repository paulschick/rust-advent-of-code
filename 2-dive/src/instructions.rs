use std::fs::{File};
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone)]
pub struct Instruction {
    pub direction: String,
    pub magnitude: i8,
}

fn read_from_file() -> Vec<String> {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines()
        .map(|l| l.expect("Could not parse lines"))
        .collect();
}

pub fn parse_instructions() -> Vec<Instruction> {
    let direction_lines = read_from_file();
    let mut data: Vec<Instruction> = Vec::new();
    for line in direction_lines {
        let instruction = Instruction::new(&line); 
        data.push(instruction);
    }

    return data;
}

impl Instruction {
    /// Create new instance of Instruction
    pub fn new(value: &String) -> Self {
        let strings = Instruction::split_by_space(&value);
        let direction = strings[0].to_string();
        let magnitude = Instruction::parse_magnitude(&strings[1]);
        return Instruction {
            direction,
            magnitude
        };
    }

    fn split_by_space(words: &String) -> Vec<&str> {
        return words.split(" ").collect();
    }

    /// Takes a string slice
    /// Attempts to return an i8
    fn parse_magnitude(magnitude: &str) -> i8 {
        return magnitude.to_string().parse::<i8>().unwrap();
    }
}


