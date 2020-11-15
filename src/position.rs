use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    row: i32,
    column: i32,
    index: i32,
}

impl Position {
    pub fn new(row: i32, column: i32, index: i32) -> Position {
        Position { row, column, index }
    }

    pub fn pre() -> Self {
        Position::new(0, -1, -1)
    }

    pub fn next_row(&mut self) {
        self.row += 1;
        self.column = 0;
        self.index += 1;
    }

    pub fn next_column(&mut self) {
        self.index += 1;
        self.column += 1;
    }

    pub fn get_row(&self) -> i32 {
        self.row
    }

    pub fn get_column(&self) -> i32 {
        self.column
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}
