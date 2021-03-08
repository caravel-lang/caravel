use super::token::Token;
use super::util;
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
      // Surround with brackets so that the program
      // is parsed as an entire block
      tokens: vec![Token::LBracket],
    }
  }

  pub fn lex(mut self) -> Vec<Token> {
    while self.pos.index < self.input.len() as u32 {
      let c = self.get();

      // Ignore whitespace
      if c == ' ' || c == '\t' {
        self.eat();
        continue;
      }

      let token = if util::is_ident_start(c) {
        self.parse_identifier_or_keyword()
      } else if c.is_ascii_digit() {
        self.parse_float_literal()
      } else {
        match self.eat() {
          '\n' => Token::Eol,
          '+' => Token::Add,
          '-' => Token::Subtract,
          '*' => Token::Multiply,
          '/' => Token::Divide,
          '%' => Token::Modulo,
          '(' => Token::LParen,
          ')' => Token::RParen,
          '{' => Token::LBracket,
          '}' => Token::RBracket,
          '=' => Token::Assignment,
          ':' => Token::Colon,
          _ => panic!("Unexpected character '{}'", self.get()),
        }
      };

      self.tokens.push(token);
    }

    // Surround with brackets so that the program
    // is parsed as an entire block
    self.tokens.push(Token::RBracket);
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

  // Token specific parse functions
  fn parse_identifier_or_keyword(&mut self) -> Token {
    let start_pos = self.pos.clone();
    let mut value = self.eat().to_string();

    while util::is_ident_body(self.get()) {
      value.push(self.eat());
    }

    match &value[..] {
      "let" => Token::Let,
      _ => Token::Identifier(self.make_source_string(&value, start_pos)),
    }
  }

  fn parse_float_literal(&mut self) -> Token {
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

    Token::FloatLiteral(self.make_source_string(&value, start_pos))
  }
}
