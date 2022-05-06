use std::fs::{File};
use std::io::{prelude::*, BufReader};

/// https://adventofcode.com/2021/day/2
/// use input.txt
///
///two directions -> horizontal, depth
///direction + number adds/removes that amount from position
///forward -> add to horizontal
///down -> add to depth
///up -> decrease depth
///
///That's all. No backwards or anything
///
///

// https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
fn read_from_file() -> Vec<String> {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines()
        .map(|l| l.expect("Could not parse lines"))
        .collect();
}

fn split_by_space(words: &String) -> Vec<&str> {
    return words.split(" ").collect();
}


fn main() {
    let lines = read_from_file();
    for line in lines {
        println!("{:?}", line);
        let strings = split_by_space(&line);
        println!("From main -> {}, {}", strings[0], strings[1]);
    }
}
