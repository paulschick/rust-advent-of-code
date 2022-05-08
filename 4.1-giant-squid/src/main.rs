#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct BingoRow {
    pub data: Vec<u32>,
    pub index: u32,
}

#[derive(Debug)]
pub struct BingoColumn {
    pub data: Vec<u32>,
    pub index: u32,
}

#[derive(Debug)]
pub struct Answer {
    pub value: u32,
    pub row_index: u32,
    pub col_index: u32,
}

#[derive(Debug)]
pub struct BingoBoard {
    pub rows: Vec<BingoRow>,
    pub columns: Vec<BingoColumn>,
    pub selected_answers: Vec<Answer>,
    pub unselected_answers: Vec<Answer>,
}

#[derive(Debug)]
pub struct BingoGame {
    pub borads: Vec<BingoBoard>,
    pub game_numbers: Vec<u32>,
}

fn copy_vec<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    let vec = vec.clone();
    return vec;
}

impl BingoBoard {
    pub fn new(board: &Vec<Vec<u32>>) -> Self {
        let row_length = board.len();
        let col_length = board[0].len();

        let test_length = usize::try_from(5).unwrap();
        if row_length != test_length || col_length != test_length {
            panic!("Unable to process board. Expected length 5 received row: {}, col: {}", row_length, col_length);
        } 

        let mut bingo_rows: Vec<BingoRow> = vec![];
        let mut bingo_cols: Vec<BingoColumn> = vec![];
        let mut unselected_answers: Vec<Answer> = vec![];
        let selected_answers: Vec<Answer> = vec![];

        for i in 0..col_length {
            let mut col: Vec<u32> = vec![];

            for a in 0..row_length {
                let row = &board[a];
                let column_value = &row[i];

                col.push(*column_value);

                if bingo_rows.len() < 5 {
                    let bingo_row = BingoRow {
                        data: copy_vec(&row),
                        index: a as u32,
                    };

                    bingo_rows.push(bingo_row);
                }

                let answer = Answer {
                    value: *column_value,
                    row_index: a as u32,
                    col_index: i as u32,
                };
                unselected_answers.push(answer);
            } 

            let bingo_col = BingoColumn {
                data: copy_vec(&col),
                index: i as u32,
            };
            bingo_cols.push(bingo_col);
        }

        return BingoBoard {
            rows: bingo_rows,
            columns: bingo_cols,
            selected_answers,
            unselected_answers,
        };
    }
}

fn handle_game_numbers(line: &str) -> Vec<u32> {
    return line
        .split(",")
        .map(|v| v.to_owned().parse::<u32>().unwrap())
        .collect();
}

fn board_row_to_ints(line: &str) -> Vec<u32> {
    line
        .split(" ")
        .filter(|v| v != &"")
        .map(|v| v.to_owned().parse::<u32>().unwrap())
        .collect()
}

fn parse_board_block(lines: &Vec<String>, start_index: u32) -> BingoBoard {
    let first_line = &lines[usize::try_from(start_index).unwrap()];
    let last_index = start_index + 4u32;
    let last_line = &lines[usize::try_from(last_index).unwrap()];
    if last_line == "" {
        panic!("Starting index does not correspond to a 5-line matrix");
    }
    let mut board_ints: Vec<Vec<u32>> = vec![];
    for i in start_index..=last_index {
        let line = &lines[usize::try_from(i).unwrap()];
        let row = board_row_to_ints(&line);
        board_ints.push(row);
    }

    return BingoBoard::new(&board_ints);
}

/// This should return some kind of data structure that makes sense
/// in the context of the bingo input
///
/// I might want to use a struct or something. The numbers to be
/// drawn are different from each of the boards. The boards are 5x5.
///
/// Since they're known, I think actually a struct would work decently
/// here.
fn parse_bingo_input() {
    let file = File::open("input.txt")
        .expect("File not found!");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Unable to parse line!"))
        .collect();

    let game_numbers = handle_game_numbers(&lines[0]);
    println!("{:?}", game_numbers);

    // for i in 1..lines.len() {
    //     let line = &lines[i];
    //     if line != "" {
    //         let row_vals = board_row_to_ints(line);
    //         println!("{:?}", row_vals);
    //     }
    // }

    // testing
    // parse_board_block(&lines, 2u32);
    // let first_line = 2u32;
    // let interval = 4u32;


    // Commented to test the parse_board_block function

    // let mut n = true;
    // let start_index = 2u32;
    // let interval = 4u32;
    // let last_index = lines.len() - 1;
    // let last_block_start = last_index - 4;
    // let mut used_indices: Vec<u32> = vec![];
    // let mut game_boards: Vec<BingoBoard> = vec![];

    // while n == true {
    //     if used_indices.len() == 0 {
    //         // then use start index
    //         let board = parse_board_block(&lines, start_index);
    //         game_boards.push(board);
    //         used_indices.push(start_index);
    //     } else {
    //         let last_idx = used_indices.last().unwrap();
    //         println!("Last Used Index: {}", last_idx);
    //
    //         // Need to be able to compare &u32 with &u32
    //         let last_valid_index: u32 = last_index as u32;
    //         if last_idx == &last_valid_index {
    //             println!("hit last index");
    //             n = false;
    //         }
    //
    //         println!("exiting for testing");
    //     }
    // }

    let board_test = parse_board_block(&lines, 2u32);
    println!("{:?}", board_test);
}

fn main() {
    // let mut test_board_values: Vec<Vec<u32>> = vec![];
    //
    // test_board_values.push(vec![1u32,2u32,3u32,4u32,5u32]);
    // test_board_values.push(vec![1u32,2u32,3u32,4u32,5u32]);
    // test_board_values.push(vec![1u32,2u32,3u32,4u32,5u32]);
    // test_board_values.push(vec![1u32,2u32,3u32,4u32,5u32]);
    // test_board_values.push(vec![1u32,2u32,3u32,4u32,5u32]);
    //
    // let test_board = BingoBoard::new(&test_board_values);

    // println!("{:?}", test_board);

    parse_bingo_input();
}
