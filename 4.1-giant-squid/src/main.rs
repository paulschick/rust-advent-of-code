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
    pub boards: Vec<BingoBoard>,
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

/// Reads in the input data, returns the BingoGame data structure
///
/// BIGTODO -> need to break this up. This function is insane.
fn parse_bingo_input() -> BingoGame {
    let file = File::open("input.txt")
        .expect("File not found!");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Unable to parse line!"))
        .collect();

    let game_numbers = handle_game_numbers(&lines[0]);
    // println!("{:?}", game_numbers);

    let mut n = true;
    let start_index = 2u32;
    let interval = 4u32;
    let last_index = lines.len() - 1;
    let last_block_start = last_index - 4;
    let mut used_indices: Vec<u32> = vec![];
    let mut game_boards: Vec<BingoBoard> = vec![];

    while n == true {
        if used_indices.len() == 0 {
            // then use start index
            let board = parse_board_block(&lines, start_index);
            game_boards.push(board);
            used_indices.push(start_index);
        } else {
            let last_idx = used_indices.last().unwrap();
            // println!("Last Used Index: {}", last_idx);

            // Need to be able to compare &u32 with &u32
            let last_valid_index: u32 = last_index as u32;
            if last_idx == &last_valid_index {
                // println!("hit last index");
                n = false;
            } else {
                let next_blank = last_idx + interval + 1;
                let next_blank_usize = usize::try_from(next_blank).unwrap();

                if next_blank >= last_index as u32 {
                    // println!("Hit the end");
                    n = false;
                } else {
                    let next_blank_line = &lines[next_blank_usize];

                    if next_blank_line != "" {
                        panic!("Expected a blank line at index {}, got {}", next_blank, next_blank_line);
                    }

                    let next_valid_index = next_blank + 1;
                    let test_next_valid_usize = usize::try_from(next_valid_index).unwrap();
                    let next_valid_line = &lines[test_next_valid_usize];

                    if next_valid_line == "" {
                        panic!("Expected a valid line at index {}, received blank", next_valid_line);
                    }

                    let board = parse_board_block(&lines, next_valid_index);
                    game_boards.push(board);
                    used_indices.push(next_valid_index);
                }
            }
        }
    }

    return BingoGame {
        boards: game_boards,
        game_numbers,
    };
}

/// Goal -> this is going to calculate all of the starting indexes for each of the Bingo
/// Boards. This should take away some of the mess so that the main parsing function
/// doesn't have to do a bunch of nested ifs
///
/// 1. We know that index 2 is the first line of the first board
/// 2. We know that 2 + 4 is the last line of the first board (6)
/// 3. We know that the index after a board's last index is a blank line
/// 4. We know that if our proposed start index or last index for a given board is equal
/// or greater than the length of the vector of strings, then we're done.
fn derive_start_indexes() {

}

fn main() {
    let game = parse_bingo_input();
    println!("{:?}", game.boards[0]);
}
