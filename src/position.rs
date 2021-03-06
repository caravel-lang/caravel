#[derive(Clone, Debug)]
pub struct Position {
  pub line: u32,
  pub column: u32,
  pub index: u32,
}

impl Position {
  pub fn start() -> Self {
    Self {
      line: 0,
      column: 0,
      index: 0,
    }
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
