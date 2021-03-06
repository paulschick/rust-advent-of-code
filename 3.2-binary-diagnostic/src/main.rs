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

/// TODO_ -> I should use recursion here.
/// Just pass in the updated Vec into the function and keep doing that.
/// Check on the length to see whether to continue or exit.
fn get_oxygen_gen_ratingv1() -> Vec<u32> {
    let contents: Vec<String> = file_contents::file_input();
    let row_length = contents.len();                // 1000
    let col_length = &contents[0].len();            // 12
    
    for i in 0..*col_length {
        // going to remove ROWS not columns
        let mut reduced_contents: Vec<String> = vec![];

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

        let most_common: u32;

        if num_1 > num_0 {
            most_common = 1u32;
        } else {
            most_common = 0u32;
        }
        

        if i == 0 {
            // then we know that we are only matching the most common
            for a in 0..row_length {
                let row = &contents[a];
                let first_val = row.chars().nth(0).unwrap();
                let first_int: u32 = first_val.to_digit(10).unwrap();

                // if first_int == 0u32 {
                //     reduced_contents.push(row.to_owned());
                // }
                if first_int == most_common {
                    reduced_contents.push(row.to_owned());
                }
            }
        } else {
            println!("column is NOT 0");
        }

        println!("{:?}", reduced_contents);

    } 

    // TEMP
    return vec![1u32];
}

fn copy_vec<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    let vec = vec.clone();
    return vec;
}

fn get_val_from_row(row: &str, idx: u32) -> u32 {
    let val = row.chars().nth(idx.try_into().unwrap()).unwrap();
    let val_int: u32 = val.to_digit(10).unwrap();
    return val_int; 
}

fn get_max_in_col(values: &Vec<String>, col_index: u32) -> u32 {
    let mut num_1 = 0;
    let mut num_0 = 0;
    let row_length = values.len();
    for i in 0..row_length {
        let row = &values[i];
        let current_val = get_val_from_row(&row, col_index);
        if current_val == 1u32 {
            num_1 += 1;
        } else {
            num_0 += 1;
        }
    }

    if num_1 >= num_0 {
        return 1u32;
    } else {
        return 0u32;
    }
}

fn get_min_in_col(values: &Vec<String>, col_index: u32) -> u32 {
    let mut num_1 = 0;
    let mut num_0 = 0;
    let row_length = values.len();

    for i in 0..row_length {
        let row = &values[i];
        let current_val = get_val_from_row(&row, col_index);
        if current_val == 1u32 {
            num_1 += 1;
        } else {
            num_0 += 1;
        }
    }

    if num_0 < num_1 {
        return 0u32;
    } else if num_0 == num_1 {
        return 0u32;
    } else {
        return 1u32;
    }
}

///
///num_to_keep -> 1 or 0
fn trim_rows(values: &Vec<String>, col_index: u32, num_to_keep: u32) -> Vec<String> {
    let mut return_vec: Vec<String> = vec![];
    let length = values.len();
    for i in 0..length {
        let row = &values[i];
        let current_val = get_val_from_row(&row, col_index);
        if current_val == num_to_keep {
            return_vec.push(row.to_owned());
        }
    }
    return return_vec;
}

fn get_oxygen_rating(values: &Vec<String>, col_index: u32) -> String {
    let row_length = values.len();
    let col_length = &values[0].len();

    // compare &usize to &usize
    let test_len = usize::try_from(1).unwrap();
    if row_length == test_len {
        let final_vec = copy_vec(values);
        return final_vec[0].to_owned();
    } else {
        // continue
        let new_vec: Vec<String>;

        let max_num = get_max_in_col(&values, col_index);
        if max_num == 1u32 {
            new_vec = trim_rows(&values, col_index, 1u32);
        } else {
            new_vec = trim_rows(&values, col_index, 0u32);
        }

        let next_index = col_index + 1u32;
        let next_index_comparitor = usize::try_from(next_index).unwrap();
        if &next_index_comparitor < col_length {
            let next_vec = copy_vec(&new_vec);
            return get_oxygen_rating(&next_vec, next_index);
        } else {
            let final_vec = copy_vec(&new_vec);
            return final_vec[0].to_owned();
        }
    }
}

fn get_scrubber_rating(values: &Vec<String>, col_index: u32) -> String {
    let row_length = values.len();
    let col_length = &values[0].len();
    let test_len = usize::try_from(1).unwrap();

    if row_length == test_len {
        let final_vec = copy_vec(values);
        return final_vec[0].to_owned();
    } else {
        let new_vec: Vec<String>;
        let min_num = get_min_in_col(&values, col_index);
        
        if min_num == 0u32 {
            new_vec = trim_rows(&values, col_index, 0u32);
        } else {
            new_vec = trim_rows(&values, col_index, 1u32);
        }

        let next_index = col_index + 1u32;
        let next_comparitor = usize::try_from(next_index).unwrap();
        if &next_comparitor < col_length {
            let next_vec = copy_vec(&new_vec);
            return get_scrubber_rating(&next_vec, next_index);
        } else {
            let final_vec = copy_vec(&new_vec);
            return final_vec[0].to_owned();
        }
    }
}

fn string_to_u32_vec(value: &str) -> Vec<u32> {
    let length = &value.len();              // number of chars
    let mut values: Vec<u32> = vec![];      // initialize 
    for i in 0..*length {                   // try parse to u32
        let curr_char = value.chars().nth(i).unwrap();
        let curr_int: u32 = curr_char.to_digit(10).unwrap();
        values.push(curr_int);
    }
    return values;
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

fn get_final_decimal() -> u32 {
    let contents: Vec<String> = file_contents::file_input();
    let oxygen_rating = get_oxygen_rating(&contents, 0u32);
    println!("Oxygen Rating -> {}", oxygen_rating);
    let oxygen_vec = string_to_u32_vec(&oxygen_rating);
    let oxygen_decimal = get_decimal(&oxygen_vec);
    println!("Oxygen Decimal -> {}", oxygen_decimal);
    let scrub_rating_2 = get_scrubber_rating(&contents, 0u32);
    let scrub_vec_2 = string_to_u32_vec(&scrub_rating_2);
    let scrub_dec_2 = get_decimal(&scrub_vec_2);
    println!("Scrubber Rating -> {}", scrub_rating_2);
    println!("Scrubber Decimal -> {}", scrub_dec_2);
    let mult = scrub_dec_2 * oxygen_decimal;
    println!("New Multiplied Value of {} and {} -> {}", oxygen_decimal, scrub_dec_2, mult);

    return mult;
}

fn main() {
    let solution: u32 = get_final_decimal();
    println!("Solution.. main -> {}", solution);
}
