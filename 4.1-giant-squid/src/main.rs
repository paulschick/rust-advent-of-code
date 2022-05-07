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
    let lines = buf
        .lines()
        .map(|l| l.expect("Unable to parse line!"))
        .collect();
}

fn main() {
    println!("Hello, world!");
}
