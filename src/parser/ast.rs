use crate::lexer::token::Token;
use std::boxed::Box;

#[derive(Debug)]
pub enum Term {
  Factor(Factor),
  Operation(Box<Term>, Token, Factor),
}

#[derive(Debug)]
pub enum Factor {
  Leaf(Leaf),
  Operation(Box<Factor>, Token, Leaf),
}

#[derive(Debug)]
pub enum Leaf {
  Identifier(String),
  FloatLiteral(String),
  Term(Box<Term>),
}
