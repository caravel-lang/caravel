use std::cmp;
use std::ops::Add;

pub const DEFAULT_POSITION: Position = Position {
  line: 0,
  column: 0,
  index: 0,
};

pub const DEFAULT_SPAN: Span = Span {
  start_pos: DEFAULT_POSITION,
  source_len: 0,
};

// Represents a single position in the input
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
  // must be first field so that Ord/PartialOrd
  // consider it first in their comparison algorithm
  pub index: u32,
  pub line: u32,
  pub column: u32,
}

impl Position {
  pub fn start() -> Self {
    DEFAULT_POSITION
  }

  pub fn advance_ln(&mut self) {
    self.index += 1;
    self.column = 0;
    self.line += 1;
  }

  pub fn advance_col(&mut self) {
    self.index += 1;
    self.column += 1;
  }
}

/// Represents a group of consecutive positions in the input
#[derive(Clone, Debug)]
pub struct Span {
  pub start_pos: Position,
  pub source_len: u32,
}

impl Add for Span {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    let min = cmp::min(self.start_pos, other.start_pos);
    let source_len = self.source_len + other.source_len;
    Self {
      start_pos: min,
      source_len,
    }
  }
}
