use crate::position::Position;
use std::fmt;

#[derive(Debug)]
pub struct CompilationError {
    message: String,
    position: Position,
    length: i32,
}

impl CompilationError {
    pub fn new(message: &str, position: Position, length: i32) -> CompilationError {
        return CompilationError {
            message: message.to_owned(),
            position,
            length,
        };
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_length(&self) -> i32 {
        self.length
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.position)
    }
}
