pub enum Expression {
  Assignment(Assignment),
  Block(Block),
  Term(Term),
}

pub type Block = Vec<Expression>;

pub enum Assignment {
  Initialization(String, String, Option<Box<Expression>>),
  Reassignment(String, Box<Expression>),
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
