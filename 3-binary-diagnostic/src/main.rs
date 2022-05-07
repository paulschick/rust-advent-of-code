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

pub mod nested_array {
    /// bool - 1 = true, 0 = false
    /// Implementation:
    /// https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    pub fn create_array(file_data: &Vec<String>) -> Vec<Vec<bool>> {
        let height = file_data.len();
        let first_val = &file_data[0];
        let width = first_val.len();
        let mut array = vec![vec![true; width]; height];
        let mut data_array: Vec<Vec<bool>> = vec![];

        for (i, row) in array.iter_mut().enumerate() {

            let current_string = &file_data[i];
            let mut line_vec: Vec<bool> = vec![];

            for (j, col) in row.iter_mut().enumerate() {
                let current_char = current_string.chars().nth(j).unwrap();
                if current_char == '1' {
                    line_vec.push(true);
                } else {
                    line_vec.push(false);
                }
            }
            data_array.push(line_vec);
        }
        return data_array;
    }

    pub fn columns_most_common(data: &mut Vec<Vec<bool>>) -> Vec<i8> {
        let col_length = *&data[0].len();
        let row_length = *&data.len();
        let mut return_array: Vec<i8> = vec![];

        for i in 0..col_length {
            let mut num_true = 0;
            let mut num_false = 0;

            for a in 0..row_length {
                if &data[a][i] == &true {
                    num_true += 1;
                } else {
                    num_false += 1;
                }
            }
            if num_true > num_false {
                return_array.push(1i8);
            } else {
                return_array.push(0i8);
            }
        }
        return return_array;
    }

    pub fn epsilon_vec(gamma: &Vec<i8>) -> Vec<i8> {
        let length = gamma.len();
        let mut epsilon: Vec<i8> = vec![];

        for i in 0..length {
            if *&gamma[i] == 1 {
                epsilon.push(0i8);
            } else {
                epsilon.push(1i8);
            }
        }
        return epsilon;
    }
}

fn create_binary_string(data: &Vec<i8>) -> String {
    let mut bin_string: String = "".to_owned();
    for i in 0..data.len() {
        bin_string.push_str(&data[i].to_string());
    }
    return bin_string;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::file_contents;
    use crate::nested_array;

    #[test]
    fn test_bit_width() {
        let file_vec = file_contents::file_input();
        let first_value = &file_vec[0];
        let real_width = first_value.len();
        assert_eq!(real_width, 12);
    }

    #[test]
    fn test_create_array() {
        let file_vec = file_contents::file_input();
        let expected_nested_length = file_vec.len();
        let array = nested_array::create_array(&file_vec);
        let received_length = array.len();
        assert_eq!(expected_nested_length, received_length);
    }

    #[test]
    fn test_gamma_value() {
        let row1 = vec![true, true, false, true];
        let row2 = vec![false, true, false, false];
        let row3 = vec![false, true, false, false];
        let mut test_vector = vec![row1, row2, row3];
        assert_eq!(test_vector.len(), 3);
        let gamma = nested_array::columns_most_common(&mut test_vector);
        let expected = vec![0i8, 1i8, 0i8, 0i8];
        assert_eq!(gamma, expected);
    }

    #[test]
    fn test_epsilon_value() {
        let gamma = vec![0i8, 1i8, 0i8, 0i8];
        let expected = vec![1i8, 0i8, 1i8, 1i8];
        let actual_epsilon = nested_array::epsilon_vec(&gamma);
        assert_eq!(actual_epsilon, expected);
    }

    #[test]
    fn binary_string_conversion() {
        let gamma = vec![0i8, 1i8, 0i8, 0i8];
        let expected: String = "0100".to_owned();
        let resulted: String = create_binary_string(&gamma);
        assert_eq!(resulted, expected);
    }
}

/// Formula to calculate a decimal from a binary number:
/// https://www.rapidtables.com/convert/number/binary-to-decimal.html
///
/// the decimal number is equal to the sum of the binary digits times their power of 2 (2^n)
/// To make it tougher, you have to go backwards... Basically "flip the bits".
///
/// if my number is 111001
/// the this as powers of 2 is: 1(2^5)1(2^4)1(2^3)0(2^2)0(2^1)1(2^0)
/// So the first number gets the highest power
/// And the highest power is length - 1, because there's a 0 index
fn convert_bin_to_decimal(binary_string: &str) -> u32 {
    let bstring: String = binary_string.to_owned();
    let highest_power = bstring.len() - 1;
    let mut decimal_value: u32 = 0;

    for i in 0..=highest_power {
        let current_power = highest_power - i;
        let bchar = bstring.chars().nth(i).unwrap();
        let base: u32 = 2;
        let base_to_power = base.pow(current_power.try_into().unwrap());
        let bint: u32 = bchar.to_digit(10).unwrap();
        let current_result = bint * base_to_power;
        decimal_value += current_result;
    }

    return decimal_value;
}

fn main() {
    println!("Getting file contents");
    let file_vec = file_contents::file_input();
    let mut array = nested_array::create_array(&file_vec);
    let gamma_vec = nested_array::columns_most_common(&mut array);
    println!("Received gamma vec -> {:?}", gamma_vec);
    let epsilon = nested_array::epsilon_vec(&gamma_vec);
    println!("Received epsilon vec -> {:?}", epsilon);

    let gamma_string = create_binary_string(&gamma_vec);
    let epsilon_string = create_binary_string(&epsilon);

    println!("Gamma string: {}, epsilon string: {}", gamma_string, epsilon_string);

    let gamma_decimal = convert_bin_to_decimal(&gamma_string);
    println!("Gamma Decimal - {}", gamma_decimal);
    let epsilon_decimal = convert_bin_to_decimal(&epsilon_string);
    println!("Epsilon Decimal - {}", epsilon_decimal);
}
