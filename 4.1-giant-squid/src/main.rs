#![allow(dead_code)]

use std::{fs::File, io::{prelude::*, BufReader}};

/// Read in the input file and return lines
fn parse_file_lines() -> Vec<String> {
    BufReader::new(
        File::open("input.txt")
            .expect("File not found!")
        )
        .lines()
        .map(|l| l.expect("unable to parse line!"))
        .collect()
}

fn parse_selected_answers(row: &str) -> Vec<u32> {
    row.split(",")
        .map(|v| v.trim()
            .to_string()
            .parse::<u32>()
            .unwrap()
            )
        .collect::<Vec<u32>>()
}

/// Creates an array of fixed length given a row as a string slice
fn create_row_array(row: &str) -> [u32; 5] {
    row.split(" ")
        .map(|v| v.trim()
            .to_string()
            .parse::<u32>()
            .unwrap()
            )
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
}


fn main() {
    let test_row_data: &str = 
        "31 88 71 23 61";
    let array = create_row_array(test_row_data);
    println!("{:?}", array);
}
