use crate::position::token_position::TokenSpan;

pub trait Node {
  fn pos(&self) -> TokenSpan;
}

pub enum Expression {
  Assignment(Assignment),
  Block(Block),
  Term(Term),
}

impl Node for Expression {
  fn pos(&self) -> TokenSpan {
    match self {
      Self::Assignment(assig) => assig.pos(),
      Self::Block(block) => block.pos(),
      Self::Term(term) => term.pos(),
    }
  }
}

pub struct Block {
  pub expressions: Vec<Expression>,
  pub start_index: usize,
}

impl Node for Block {
  fn pos(&self) -> TokenSpan {
    if self.expressions.len() == 0 {
      return TokenSpan {
        start: self.start_index,
        len: 2,
      };
    }

    let mut pos = self.expressions[0].pos();

    match self.expressions.iter().last() {
      Some(last) => pos = pos + last.pos(),
      None => (),
    }

    pos
  }
}

pub enum Assignment {
  Initialization(String, String, Option<Box<Expression>>, usize),
  Reassignment(String, Box<Expression>),
}

impl Node for Assignment {
  fn pos(&self) -> TokenSpan {
    match self {
      Self::Initialization(_, _, rhs, start) => TokenSpan::new(
        *start,
        5 + match rhs {
          Some(rhs) => rhs.pos().len,
          None => 0,
        },
      ),
      Self::Reassignment(_, expr) => expr.pos() - 2,
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
  fn pos(&self) -> TokenSpan {
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
  fn pos(&self) -> TokenSpan {
    match self {
      Self::Leaf(leaf) => leaf.pos(),
      Self::Operation(lhs, _, rhs) => lhs.pos() + rhs.pos(),
    }
  }
}

pub enum Leaf {
  Identifier(String, usize),
  FloatLiteral(String, usize),
  Term(Box<Term>),
}

impl Node for Leaf {
  fn pos(&self) -> TokenSpan {
    match self {
      Self::Identifier(_, start) | Self::FloatLiteral(_, start) => TokenSpan::new(*start, 1),
      Self::Term(term) => term.pos(),
    }
  }
}
