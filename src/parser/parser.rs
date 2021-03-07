use super::ast::*;
use crate::lexer::token::{Token, TokenType};
use crate::position::Position;

const eofToken: Token = Token {
  start_pos: Position {
    line: 0,
    column: 0,
    index: 0,
  },
  source_len: 0,
  token_type: TokenType::Eof,
};

pub struct Parser {
  tokens: Vec<Token>,
  index: u32,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, index: 0 }
  }

  pub fn parse(mut self) -> Block {
    self.parse_block()
  }

  // Helpers
  fn get(&self) -> Token {
    self
      .tokens
      .get(self.index as usize)
      .unwrap_or(&eofToken)
      .clone()
  }

  fn peek(&self) -> Token {
    self
      .tokens
      .get(self.index as usize + 1)
      .unwrap_or(&eofToken)
      .clone()
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
        // Last EOL in a block is optional
        TokenType::RBracket => break,
        _ => panic!("Unexpected token"),
      }
    }

    expressions
  }

  // assignment : let identifier ':' identifier '=' term    ; Assignment
  //            | let identifier ':' identifier             ; DefaultAssignment
  //            | identifier '=' term                       ; Reassignment
  //            | term
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
      TokenType::Assignment | TokenType::Colon => (),
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

    let mut assig_type = String::new();
    if has_let {
      match self.eat().token_type {
        TokenType::Colon => (),
        _ => panic!("Expected colon"),
      }

      match self.eat().token_type {
        TokenType::Identifier(typ) => assig_type = typ,
        _ => panic!("Expected type"),
      }
    }

    match self.get().token_type {
      TokenType::Assignment => self.eat(),
      _ => return Assignment::DefaultAssignment(identifier, assig_type),
    };

    let rhs = self.parse_term();

    match has_let {
      true => Assignment::Assignment(identifier, assig_type, rhs),
      false => Assignment::Reassignment(identifier, rhs),
    }
  }

  // term : factor (('+' | '-') factor)*
  fn parse_term(&mut self) -> Term {
    let mut term = Term::Factor(self.parse_factor());

    loop {
      let op = match self.get().token_type {
        TokenType::Add => TermOp::Add,
        TokenType::Subtract => TermOp::Subtract,
        _ => break,
      };

      self.eat();

      let rhs = self.parse_factor();
      term = Term::Operation(Box::new(term), op, rhs);
    }

    term
  }

  // factor : leaf (('*' | '/' | '%') leaf)*
  fn parse_factor(&mut self) -> Factor {
    let mut factor = Factor::Leaf(self.parse_leaf());

    loop {
      let op = match self.get().token_type {
        TokenType::Multiply => FactorOp::Multiply,
        TokenType::Divide => FactorOp::Divide,
        TokenType::Modulo => FactorOp::Modulo,
        _ => break,
      };

      self.eat();

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
