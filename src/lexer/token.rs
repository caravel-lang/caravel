use crate::position::source_position::SourceSpan;

#[derive(Clone, Debug)]
pub struct Token {
  pub pos: SourceSpan,
  pub kind: TokenKind,
}

#[derive(Clone, Debug)]
pub enum TokenKind {
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
  Colon,

  // Keywords
  Let,

  /// Special token, used by the parser to signify
  /// the end of the token stream. Will never be
  /// generated by the lexer
  Eof,
}
