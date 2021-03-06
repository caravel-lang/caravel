use crate::lexer::token::Token;
use std::boxed::Box;

pub enum Expression {
  Assignment(Assignment),
  Block(Block),
}

pub type Block = Vec<Expression>;

pub enum Assignment {
  Assignment(String, Term),
  Reassignment(String, Term),
  Declaration(String),
  Term(Term),
}

pub enum Term {
  Factor(Factor),
  Operation(Box<Term>, Token, Factor),
}

pub enum Factor {
  Leaf(Leaf),
  Operation(Box<Factor>, Token, Leaf),
}

pub enum Leaf {
  Identifier(String),
  FloatLiteral(String),
  Term(Box<Term>),
}
