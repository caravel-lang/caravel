use crate::position::{Span, DEFAULT_SPAN};
use crate::source_string::SourceString;

pub trait Node {
  fn pos(&self) -> Span;
}

pub enum Expression {
  Assignment(Assignment),
  Block(Block),
  Term(Term),
}

impl Node for Expression {
  fn pos(&self) -> Span {
    match self {
      Self::Assignment(node) => node.pos(),
      Self::Block(node) => node.pos(),
      Self::Term(node) => node.pos(),
    }
  }
}

pub type Block = Vec<Expression>;

impl Node for Block {
  fn pos(&self) -> Span {
    let mut iter = self.iter();
    let first_expr = match iter.next() {
      Some(first) => first,
      None => return DEFAULT_SPAN,
    };
    iter
      .map(|expr| expr.pos())
      .fold(first_expr.pos(), |acc, pos| acc + pos)
  }
}

pub enum Assignment {
  Initialization(SourceString, SourceString, Option<Box<Expression>>),
  Reassignment(SourceString, Box<Expression>),
}

impl Node for Assignment {
  fn pos(&self) -> Span {
    match self {
      Self::Initialization(ident, type_ident, rhs) => {
        let mut pos = ident.pos.clone() + type_ident.pos.clone();
        if let Some(rhs) = rhs {
          pos = pos + rhs.pos();
        }
        pos
      }
      Self::Reassignment(ident, rhs) => ident.pos.clone() + rhs.pos(),
    }
  }
}

pub enum TermOp {
  Add,
  Subtract,
}

pub enum Term {
  Factor(Factor),
  Operation(Box<Term>, TermOp, Factor),
}

impl Node for Term {
  fn pos(&self) -> Span {
    match self {
      Self::Factor(factor) => factor.pos(),
      Self::Operation(lhs, _, rhs) => lhs.pos() + rhs.pos(),
    }
  }
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

impl Node for Factor {
  fn pos(&self) -> Span {
    match self {
      Self::Leaf(leaf) => leaf.pos(),
      Self::Operation(lhs, _, rhs) => lhs.pos() + rhs.pos(),
    }
  }
}

pub enum Leaf {
  Identifier(SourceString),
  FloatLiteral(SourceString),
  Term(Box<Term>),
}

impl Node for Leaf {
  fn pos(&self) -> Span {
    match self {
      Self::Identifier(str) | Self::FloatLiteral(str) => str.pos.clone(),
      Self::Term(term) => term.pos(),
    }
  }
}
