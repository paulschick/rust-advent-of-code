use std::fs::{File};
use std::io::{BufRead, BufReader};


/// https://adventofcode.com/2021/day/1


fn count_increases(vec: &Vec<i64>) -> i32 {
    let mut count = 0;
    for i in 0..vec.len() {
        let current_value = vec[i];
        if i != 0 {
            let previous_value = vec[i - 1];
            if current_value > previous_value {
                count += 1;
            }
        }
    }

    return count;
}

/// https://stackoverflow.com/questions/65100493/how-to-read-a-list-of-numbers-from-a-file-into-a-vec
fn load_from_file() -> Vec<i64> {
    let file = File::open("input.txt").expect("file was not found");
    let reader = BufReader::new(file);
    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    return numbers;
}

fn main() {
    let from_file = load_from_file();
    let count = count_increases(&from_file);
    println!("Total Count: {}", count);
}
