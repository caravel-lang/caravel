use super::token::{Token, TokenKind};
use super::util;
use crate::error::{Error, ErrorKind, Result};
use crate::position::{Position, Span};
use crate::source_string::SourceString;

pub struct Lexer {
  input: String,
  pos: Position,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(input: &str) -> Self {
    Self {
      input: input.to_owned(),
      pos: Position::start(),
      tokens: Vec::new(),
    }
  }

  pub fn lex(mut self) -> Result<Vec<Token>> {
    // Surround with brackets so that the program
    // is parsed as an entire block
    self.add_token(TokenKind::LBracket, self.pos.clone());

    while self.pos.index < self.input.len() as u32 {
      let start_pos = self.pos.clone();
      let c = self.get();

      // Ignore whitespace
      if c == ' ' || c == '\t' {
        self.eat();
        continue;
      }

      let kind = if util::is_ident_start(c) {
        self.parse_identifier_or_keyword()
      } else if c.is_ascii_digit() {
        self.parse_float_literal()
      } else {
        match self.eat() {
          '\n' => TokenKind::Eol,
          '+' => TokenKind::Add,
          '-' => TokenKind::Subtract,
          '*' => TokenKind::Multiply,
          '/' => TokenKind::Divide,
          '%' => TokenKind::Modulo,
          '(' => TokenKind::LParen,
          ')' => TokenKind::RParen,
          '{' => TokenKind::LBracket,
          '}' => TokenKind::RBracket,
          '=' => TokenKind::Assignment,
          ':' => TokenKind::Colon,
          _ => {
            return Err(Error::new(
              ErrorKind::UnexpectedChar,
              &format!("Unexpected character '{}'", c),
              Span::new(start_pos, 1),
            ))
          }
        }
      };

      self.add_token(kind, start_pos);
    }

    // Surround with brackets so that the program
    // is parsed as an entire block
    self.add_token(TokenKind::RBracket, self.pos.clone());

    Ok(self.tokens)
  }

  // Helpers
  fn get(&self) -> char {
    self
      .input
      .chars()
      .nth(self.pos.index as usize)
      .unwrap_or('\0')
  }

  fn eat(&mut self) -> char {
    let c = self.get();

    if c == '\n' {
      self.pos.advance_ln();
    } else {
      self.pos.advance_col();
    }

    c
  }

  fn make_source_string(&self, str: &str, start_pos: Position) -> SourceString {
    let start_index = start_pos.index;
    SourceString {
      value: str.to_owned(),
      pos: Span {
        start_pos,
        source_len: self.pos.index - start_index,
      },
    }
  }

  fn add_token(&mut self, kind: TokenKind, start_pos: Position) {
    self.tokens.push(Token {
      kind,
      pos: Span::new(start_pos.clone(), self.pos.index - start_pos.index),
    })
  }

  // Token specific parse functions
  fn parse_identifier_or_keyword(&mut self) -> TokenKind {
    let start_pos = self.pos.clone();
    let mut value = self.eat().to_string();

    while util::is_ident_body(self.get()) {
      value.push(self.eat());
    }

    match &value[..] {
      "let" => TokenKind::Let,
      _ => TokenKind::Identifier(self.make_source_string(&value, start_pos)),
    }
  }

  fn parse_float_literal(&mut self) -> TokenKind {
    let start_pos = self.pos.clone();
    let mut value = self.eat().to_string();
    let mut encountered_period = false;

    while util::is_float_literal_body(self.get()) {
      let c = self.eat();

      if c == '.' {
        if encountered_period {
          break;
        }

        encountered_period = true;
      }

      value.push(c);
    }

    TokenKind::FloatLiteral(self.make_source_string(&value, start_pos))
  }
}
