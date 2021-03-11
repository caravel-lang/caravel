pub const DEFAULT_SOURCE_POSITION: SourcePosition = SourcePosition {
  line: 0,
  column: 0,
  index: 0,
};

pub const DEFAULT_REAL_SPAN: SourceSpan = SourceSpan {
  start_pos: DEFAULT_SOURCE_POSITION,
  source_len: 0,
};

// Represents a single position in the input
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourcePosition {
  // must be first field so that Ord/PartialOrd
  // consider it first in their comparison algorithm
  pub index: usize,
  pub line: usize,
  pub column: usize,
}

impl SourcePosition {
  pub fn start() -> Self {
    DEFAULT_SOURCE_POSITION
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
pub struct SourceSpan {
  pub start_pos: SourcePosition,
  pub source_len: usize,
}

impl SourceSpan {
  pub fn new(start_pos: SourcePosition, source_len: usize) -> Self {
    Self {
      start_pos,
      source_len,
    }
  }
}
