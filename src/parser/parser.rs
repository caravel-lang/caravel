use super::ast::*;
use crate::lexer::token::{Token, TokenType};

pub struct Parser {
  tokens: Vec<Token>,
  index: u32,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, index: 0 }
  }

  pub fn parse(mut self) -> Expression {
    // while self.index < self.tokens.len() as u32 {
    //   println!("{:?}", self.eat())
    // }
    self.parse_expression()
  }

  // Helpers
  fn get(&self) -> Token {
    self.tokens[self.index as usize].clone()
  }

  fn peek(&self) -> Token {
    self.tokens[self.index as usize + 1].clone()
  }

  fn eat(&mut self) -> Token {
    let tok = self.get();
    self.index += 1;
    tok
  }

  // Parse functions
  // expression : assignment
  fn parse_expression(&mut self) -> Expression {
    match self.get().token_type {
      TokenType::LBracket => Expression::Block(self.parse_block()),
      _ => Expression::Assignment(self.parse_assignment()),
    }
  }

  // block : '{' (((expression)? EOL)* | expression) '}'
  fn parse_block(&mut self) -> Block {
    self.eat();

    let mut expressions = Vec::new();
    loop {
      match self.get().token_type {
        TokenType::RBracket => break,
        // Allow empty lines
        TokenType::Eol => {
          self.eat();
          continue;
        }
        _ => (),
      };

      expressions.push(self.parse_expression());

      match self.eat().token_type {
        TokenType::Eol => (),
        // Allow single line blocks
        TokenType::RBracket => break,
        _ => panic!("Unexpected token"),
      }
    }

    expressions
  }

  // assignment : (let)? identifier '=' term | term
  fn parse_assignment(&mut self) -> Assignment {
    let has_let = match self.get().token_type {
      TokenType::Let => {
        self.eat();
        true
      }
      TokenType::Identifier(_) => false,
      _ => return Assignment::Term(self.parse_term()),
    };

    match self.peek().token_type {
      TokenType::Assignment => (),
      _ => {
        if !has_let {
          return Assignment::Term(self.parse_term());
        }
      }
    };

    let identifier = match self.eat().token_type.clone() {
      TokenType::Identifier(val) => val,
      _ => panic!("Unexpected token"),
    };

    match self.eat().token_type {
      TokenType::Assignment => (),
      _ => return Assignment::Declaration(identifier),
    };

    let rhs = self.parse_term();

    match has_let {
      true => Assignment::Assignment(identifier, rhs),
      false => Assignment::Reassignment(identifier, rhs),
    }
  }

  // term : factor (('+' | '-') factor)*
  fn parse_term(&mut self) -> Term {
    let mut term = Term::Factor(self.parse_factor());

    loop {
      let op = match self.get().token_type {
        TokenType::Add | TokenType::Subtract => self.eat(),
        _ => break,
      };

      let rhs = self.parse_factor();
      term = Term::Operation(Box::new(term), op.clone(), rhs);
    }

    term
  }

  // factor : leaf (('*' | '/' | '%') leaf)*
  fn parse_factor(&mut self) -> Factor {
    let mut factor = Factor::Leaf(self.parse_leaf());

    loop {
      let op = match self.get().token_type {
        TokenType::Multiply | TokenType::Divide | TokenType::Modulo => self.eat(),
        _ => break,
      };

      let rhs = self.parse_leaf();
      factor = Factor::Operation(Box::new(factor), op.clone(), rhs);
    }

    factor
  }

  // leaf : identifier | float_literal
  fn parse_leaf(&mut self) -> Leaf {
    let tok = self.eat();

    match tok.token_type {
      TokenType::Identifier(value) => Leaf::Identifier(value),
      TokenType::FloatLiteral(value) => Leaf::FloatLiteral(value),
      TokenType::LParen => {
        let term = self.parse_term();
        match self.eat().token_type {
          TokenType::RParen => (),
          _ => panic!("Unexpected token"),
        };
        Leaf::Term(Box::new(term))
      }
      _ => panic!("Unexpected token"),
    }
  }
}
