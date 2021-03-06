use super::token::{Token, TokenType};
use super::util;
use crate::position::Position;

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
      tokens: vec![],
    }
  }

  pub fn lex(mut self) -> Vec<Token> {
    while self.pos.index < self.input.len() as u32 {
      let c = self.get();
      let start_pos = self.pos.clone();

      if util::is_ident_start(c) {
        self.parse_identifier_or_keyword();
      } else if c.is_ascii_digit() {
        self.parse_float_literal();
      } else if c == ' ' || c == '\t' {
        self.eat(); // Ignore whitespace
      } else {
        let token_type = match c {
          '\n' => TokenType::Eol,
          '+' => TokenType::Add,
          '-' => TokenType::Subtract,
          '*' => TokenType::Multiply,
          '/' => TokenType::Divide,
          '%' => TokenType::Modulo,
          '(' => TokenType::LParen,
          ')' => TokenType::RParen,
          _ => panic!("Unexpected character '{}'", self.get()),
        };

        self.eat();

        self.add_token(token_type, start_pos);
      }
    }

    // Always append an EOL
    self.add_token(TokenType::Eol, self.pos.clone());
    self.tokens
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

  fn add_token(&mut self, token_type: TokenType, start_pos: Position) {
    let start_index = start_pos.index;
    self.tokens.push(Token {
      token_type,
      start_pos,
      source_len: self.pos.index - start_index,
    })
  }

  // Token specific parse functions
  fn parse_identifier_or_keyword(&mut self) {
    let start_pos = self.pos.clone();
    let mut value = self.eat().to_string();

    while util::is_ident_body(self.get()) {
      value.push(self.eat());
    }

    let token_type = match &value[..] {
      "let" => TokenType::Let,
      _ => TokenType::Identifier(value),
    };

    self.add_token(token_type, start_pos);
  }

  fn parse_float_literal(&mut self) {
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

    self.add_token(TokenType::FloatLiteral(value), start_pos);
  }
}
