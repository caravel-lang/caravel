use super::ast::*;
use crate::lexer::token::{Token, TokenType};
use crate::position::Position;

const EOF_TOKEN: Token = Token {
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
  fn get(&self) -> &Token {
    self.tokens.get(self.index as usize).unwrap_or(&EOF_TOKEN)
  }

  fn peek(&self) -> &Token {
    self
      .tokens
      .get(self.index as usize + 1)
      .unwrap_or(&EOF_TOKEN)
  }

  fn eat(&mut self) -> &Token {
    self.index += 1;
    self
      .tokens
      .get(self.index as usize - 1)
      .unwrap_or(&EOF_TOKEN)
  }

  // Parse functions
  // expression : assignment
  //            | block
  //            | term
  fn parse_expression(&mut self) -> Expression {
    match self.get().token_type {
      TokenType::LBracket => Expression::Block(self.parse_block()),
      TokenType::Let => Expression::Assignment(self.parse_assignment()),
      TokenType::Identifier(_) => {
        if let TokenType::Assignment = self.peek().token_type {
          Expression::Assignment(self.parse_assignment())
        } else {
          Expression::Term(self.parse_term())
        }
      }
      _ => Expression::Term(self.parse_term()),
    }
  }

  // block : '{' ((expression)? EOL)* (expression)? '}'
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

  // assignment : let identifier ':' identifier '=' expression    ; Initialization
  //            | let identifier ':' identifier                   ; Initialization
  //            | identifier '=' expression                       ; Reassignment
  fn parse_assignment(&mut self) -> Assignment {
    match &self.eat().token_type.clone() {
      TokenType::Let => {
        let ident = match &self.eat().token_type {
          TokenType::Identifier(ident) => ident.clone(),
          _ => panic!("Expected identifier"),
        };

        if !matches!(self.eat().token_type, TokenType::Colon) {
          panic!("Expected colon");
        }

        let type_ident = match &self.eat().token_type {
          TokenType::Identifier(ident) => ident.clone(),
          _ => panic!("Expected identifier"),
        };

        if !matches!(self.get().token_type, TokenType::Assignment) {
          return Assignment::Initialization(ident, type_ident, None);
        };

        self.eat(); // eat '='

        let value = self.parse_expression();
        Assignment::Initialization(ident, type_ident, Some(Box::new(value)))
      }
      TokenType::Identifier(ident) => {
        if !matches!(self.eat().token_type, TokenType::Assignment) {
          panic!("Expected assignment operator");
        };

        let value = self.parse_expression();
        Assignment::Reassignment(ident.to_owned(), Box::new(value))
      }
      _ => panic!("Unexpected token {:?}", self.get()),
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

  // leaf : identifier
  //      | float_literal
  fn parse_leaf(&mut self) -> Leaf {
    let tok = self.eat();

    match &tok.token_type {
      TokenType::Identifier(value) => Leaf::Identifier(value.to_owned()),
      TokenType::FloatLiteral(value) => Leaf::FloatLiteral(value.to_owned()),
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
