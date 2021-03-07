use crate::lexer::token::Token;
use std::boxed::Box;

pub enum Expression {
  Assignment(Assignment),
  Block(Block),
}

pub type Block = Vec<Expression>;

pub enum Assignment {
  DefaultAssignment(String, String),
  Assignment(String, String, Term),
  Reassignment(String, Term),
  Term(Term),
}

pub enum TermOp {
  Add,
  Subtract,
}

pub enum Term {
  Factor(Factor),
  Operation(Box<Term>, TermOp, Factor),
}

pub enum FactorOp {
  Multiply,
  Divide,
  Modulo,
}

pub enum Factor {
  Leaf(Leaf),
  Operation(Box<Factor>, FactorOp, Leaf),
}

pub enum Leaf {
  Identifier(String),
  FloatLiteral(String),
  Term(Box<Term>),
}
