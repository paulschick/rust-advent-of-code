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

    pub fn update_horizontal(&mut self, val: i16) {
        self.horizontal.set(val);
    }

    pub fn update_depth(&mut self, val: i16) {
        self.depth.set(val);
    }

    pub fn parse_forward(&mut self, val: i8) {
        self.horizontal.set(self.horizontal.get() + i16::from(val));
    }

    pub fn parse_down(&mut self, val: i8) {
        self.depth.set(self.depth.get() + i16::from(val));
    }

    pub fn parse_up(&mut self, val: i8) {
        self.depth.set(self.depth.get() - i16::from(val));
    }
}
