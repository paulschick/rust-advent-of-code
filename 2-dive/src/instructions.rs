#[derive(Debug)]
pub struct Instruction {
    direction: String,
    magnitude: i8,
}

impl Instruction {
    /// Create new instance of Instruction
    pub fn new(value: &String) -> Self {
        let strings = Instruction::split_by_space(&value);
        let direction = strings[0].to_string();
        let magnitude = Instruction::parse_magnitude(&strings[1]);
        return Instruction {
            direction,
            magnitude
        };
    }

    fn split_by_space(words: &String) -> Vec<&str> {
        return words.split(" ").collect();
    }

    /// Takes a string slice
    /// Attempts to return an i8
    fn parse_magnitude(magnitude: &str) -> i8 {
        return magnitude.to_string().parse::<i8>().unwrap();
    }
}


