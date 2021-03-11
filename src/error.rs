use crate::ansi::{style, BOLD, RED};
use crate::position::Span;

pub type Result<T> = std::result::Result<T, Error>;

pub enum ErrorKind {
  UnexpectedChar,
  UnexpectedToken,
}

pub struct Error {
  kind: ErrorKind,
  msg: String,
  pos: Span,
}

impl Error {
  pub fn new(kind: ErrorKind, msg: &str, pos: Span) -> Self {
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
  };

  let line_num = error.pos.start_pos.line as usize;
  let preceding_lines = source.split('\n').take(line_num + 1);
  let preceding_chars = preceding_lines
    .clone()
    .take(line_num)
    .fold(String::new(), |acc, cur| acc + cur)
    .len();
  let line = preceding_lines.last().unwrap();

  let index_on_line = error.pos.start_pos.index as usize - preceding_chars;

  println!("{} {}", style("error:", RED), style(err_name, BOLD),);
  println!("      {}", line);
  println!(
    "      {}{} {}",
    " ".repeat(index_on_line),
    style(&"^".repeat(error.pos.source_len as usize), RED),
    style(&error.msg, RED)
  )
}
