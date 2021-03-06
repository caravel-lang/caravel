use crate::position::Position;

#[derive(Clone, Debug)]
pub enum TokenType {
  Identifier(String),

  FloatLiteral(String),

  // Operators
  // Arithmetic Operators
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  // Assignment Operators
  Assignment,

  Eol,

  LParen,
  RParen,
  RBracket,
  LBracket,

  // Keywords
  Let,
}

#[derive(Clone, Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub start_pos: Position,
  pub source_len: u32,
}
