#![feature(never_type)]

pub mod analyzer;
pub mod ansi;
pub mod backend;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod position;
pub mod symbol_table;
pub mod types;

use analyzer::Analyzer;
use error::{print_error, Result};
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::fs;

const FILENAME: &'static str = "input/input.cv";

fn compile(source: &str) -> Result<()> {
  let lexer = Lexer::new(source);
  let tokens = lexer.lex()?;

  let parser = Parser::new(&tokens);
  let block = parser.parse()?;

  let mut analyzer = Analyzer::new(&tokens);
  analyzer.analyze(&block)?;

  backend::print::print(&block);

  Ok(())
}

fn main() {
  let source = fs::read_to_string(FILENAME).unwrap();

  let result = compile(&source);

  if let Err(error) = result {
    print_error(&error, &source);
  }
}
