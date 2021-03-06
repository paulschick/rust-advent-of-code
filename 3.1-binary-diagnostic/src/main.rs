#![allow(dead_code)]
#![allow(unused_variables)]

pub mod file_contents {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    pub fn file_input() -> Vec<String> {
        let file = File::open("input.txt").expect("File not found!!");
        let buf = BufReader::new(file);
        return buf
            .lines()
            .map(|l| l.expect("Unable to parse lines!"))
            .collect();
    }
}

fn get_gamma() -> Vec<u32> {
    let contents = file_contents::file_input();     // Vec<String>
    let row_length = contents.len();                // 1000
    let col_length = &contents[0].len();            // 12
    
    let mut columns_most_common: Vec<u32> = vec![];

    for i in 0..*col_length {
        let mut num_1 = 0;
        let mut num_0 = 0;

        for a in 0..row_length {
            let row = &contents[a];
            let curr_char = row.chars().nth(i).unwrap();
            let curr_int: u32 = curr_char.to_digit(10).unwrap();

            if curr_int == 1u32 {
                num_1 += 1;
            } else if curr_int == 0u32 {
                num_0 += 1;
            }
        }

        if num_1 > num_0 {
            columns_most_common.push(1u32);
        } else {
            columns_most_common.push(0u32);
        }
    }

    return columns_most_common;
}

fn get_epsilon(gamma: &Vec<u32>) -> Vec<u32> {
    let length = gamma.len();
    let mut ep: Vec<u32> = vec![];
    for i in 0..length {
        if *&gamma[i] == 1u32 {
            ep.push(0u32);
        } else {
            ep.push(1u32);
        }
    }
    return ep;
}

fn get_decimal(values: &Vec<u32>) -> u32 {
    let highest_power = values.len() - 1;
    let mut val: u32 = 0;
    for i in 0..=highest_power {
        let power = highest_power - i;
        let base: u32 = 2;
        let powered_base = base.pow(power.try_into().unwrap());
        let curr_binary: u32 = values[i];
        let curr_result = curr_binary * powered_base;
        val += curr_result;
    }
    return val;
}

fn main() {
    let gamma = get_gamma();
    println!("Gamma Values: {:?}", gamma);
    let ep = get_epsilon(&gamma);
    println!("Epsilon Values: {:?}", ep);

    let gamma_decimal = get_decimal(&gamma);
    let ep_decimal = get_decimal(&ep);
    println!("Gamma Decimal: {}", gamma_decimal);
    println!("Epsilon Decimal: {}", ep_decimal);

    let multiplied = gamma_decimal * ep_decimal;
    println!("Gamma * Epsilon: {}", multiplied);
}
