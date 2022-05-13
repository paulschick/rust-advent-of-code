#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

/// Read in the input file and return lines
fn parse_file_lines() -> Vec<String> {
    BufReader::new(File::open("input.txt").expect("File not found!"))
        .lines()
        .map(|l| l.expect("unable to parse line!"))
        .collect()
}

fn parse_selected_answers(row: &str) -> Vec<u32> {
    row.split(",")
        .map(|v| v.trim().to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

/// Creates an array of fixed length given a row as a string slice
fn create_row_array(row: &str) -> [u32; 5] {
    row.split(" ")
        .filter(|v| v != &"")
        .map(|v| v.trim().to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
}

/// Must pass in a Vec<&str> that contains only the lines for a given board.
/// The length of the Vec should be 5, and the length of each &str should be five.
/// This function is not responsible for determining the indices of the master Vec.
fn parse_board_lines(lines: Vec<&str>) -> [[u32; 5]; 5] {
    let mut board: [[u32; 5]; 5] = [[0u32; 5]; 5];
    for i in 0..lines.len() {
        board[i] = create_row_array(lines[i]);
    }
    return board;
}

/// Each board is [[u32; 5]; 5]
/// So I need all boards to be Vec<[[u32; 5]; 5]>
/// I still don't know how to track this.
fn main() {
    // let test_row_data: &str =
    //     "31 88 71 23 61";
    // let array = create_row_array(test_row_data);
    // println!("{:?}", array);

    let lines = parse_file_lines();
    let selected_answers_row = &lines[0];
    let selected_answers_vec = parse_selected_answers(&selected_answers_row);
    println!("Selected Answers {:?}", selected_answers_vec);

    // board start, board end indices
    let test_start = usize::try_from(2).unwrap();
    let test_end = usize::try_from(6).unwrap();

    let mut test_board_lines: Vec<&str> = vec![];

    for i in test_start..=test_end {
        test_board_lines.push(&lines[i].trim());
    }

    println!("Test Board Strings {:?}", test_board_lines);

    let test_board = parse_board_lines(test_board_lines);

    println!("{:?}", test_board);
}
