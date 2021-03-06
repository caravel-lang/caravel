use super::ast::{Factor, Leaf, Term};
use crate::lexer::token::{Token, TokenType};
use std::boxed::Box;

pub struct Parser {
  tokens: Vec<Token>,
  index: u32,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, index: 0 }
  }

  pub fn parse(mut self) -> Term {
    // while self.index < self.tokens.len() as u32 {
    //   println!("{:?}", self.eat())
    // }
    self.parse_term()
  }

  // Helpers
  fn get(&self) -> Token {
    self.tokens[self.index as usize].clone()
  }

  fn eat(&mut self) -> Token {
    let tok = self.get();
    self.index += 1;
    tok
  }

  // Parse functions
  // term : factor ( ( '+' | '-' ) factor )*
  fn parse_term(&mut self) -> Term {
    let mut term = Term::Factor(self.parse_factor());

    loop {
      let op = match self.get().token_type {
        TokenType::Add | TokenType::Subtract => self.eat(),
        _ => break,
      };

      let rhs = self.parse_factor();
      term = Term::Operation(Box::new(term), op, rhs);
    }

    term
  }

  // factor : leaf ( ( '*' | '/' | '%' ) leaf )*
  fn parse_factor(&mut self) -> Factor {
    let mut factor = Factor::Leaf(self.parse_leaf());

    loop {
      let op = match self.get().token_type {
        TokenType::Multiply | TokenType::Divide | TokenType::Modulo => self.eat(),
        _ => break,
      };

      let rhs = self.parse_leaf();
      factor = Factor::Operation(Box::new(factor), op, rhs);
    }

    factor
  }

  // leaf : identifier | float_literal
  fn parse_leaf(&mut self) -> Leaf {
    let tok = self.eat();

    match tok.token_type {
      TokenType::Identifier(value) => Leaf::Identifier(value),
      TokenType::FloatLiteral(value) => Leaf::FloatLiteral(value),
      _ => panic!("Unexpected token"),
    }
  }
}
