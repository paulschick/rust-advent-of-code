use std::cell::Cell;

#[derive(Debug)]
pub struct Position {
    horizontal: Cell<i16>,
    depth: Cell<i16>,
}

impl Position {
    pub fn new() -> Self {
        //horizontal: 0,
        //depth: 0,
        return Position {
            horizontal: Cell::new(0),
            depth: Cell::new(0),
        };
    }
}
