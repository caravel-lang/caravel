use crate::ansi::{style, BOLD, RED};
use crate::position::source_position::SourceSpan;

pub type Result<T> = std::result::Result<T, Error>;

pub enum ErrorKind {
  UnexpectedChar,
  UnexpectedToken,
  UndeclaredVariable,
  TypeMismatch,
  Redeclaration,
}

pub struct Error {
  kind: ErrorKind,
  msg: String,
  pos: SourceSpan,
}

impl Error {
  pub fn new(kind: ErrorKind, msg: &str, pos: SourceSpan) -> Self {
    Self {
      kind,
      msg: msg.to_owned(),
      pos,
    }
  }
}

pub fn print_error(error: &Error, source: &str) {
  let err_name = match error.kind {
    ErrorKind::UnexpectedChar => "unexpected character",
    ErrorKind::UnexpectedToken => "unexpected token",
    ErrorKind::UndeclaredVariable => "use of undeclared variable",
    ErrorKind::TypeMismatch => "type mismatch",
    ErrorKind::Redeclaration => "multiple declarations for single variable",
  };

  let line_num = error.pos.start_pos.line as usize;
  let preceding_lines = source.split('\n').take(line_num + 1);
  let preceding_chars = preceding_lines
    .clone()
    .take(line_num)
    .fold(0, |acc, cur| acc + cur.len())
    + line_num;
  let line = preceding_lines.last().unwrap();

  let index_on_line = error.pos.start_pos.index - preceding_chars;

  println!("{} {}", style("error:", RED), style(err_name, BOLD),);
  println!("      {}", line);
  println!(
    "      {}{} {}",
    " ".repeat(index_on_line),
    style(&"^".repeat(error.pos.source_len as usize), RED),
    style(&error.msg, RED)
  )
}
