use super::ast::*;
use crate::lexer::token::Token;

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
    self.tokens.get(self.index as usize).unwrap_or(&Token::Eof)
  }

  fn peek(&self) -> &Token {
    self
      .tokens
      .get(self.index as usize + 1)
      .unwrap_or(&Token::Eof)
  }

  fn eat(&mut self) -> &Token {
    self.index += 1;
    self
      .tokens
      .get(self.index as usize - 1)
      .unwrap_or(&Token::Eof)
  }

  // Parse functions
  // expression : assignment
  //            | block
  //            | term
  fn parse_expression(&mut self) -> Expression {
    match self.get() {
      Token::LBracket => Expression::Block(self.parse_block()),
      Token::Let => Expression::Assignment(self.parse_assignment()),
      Token::Identifier(_) => {
        if let Token::Assignment = self.peek() {
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
      match self.get() {
        Token::RBracket => break,
        // Allow empty lines
        Token::Eol => {
          self.eat();
          continue;
        }
        _ => (),
      };

      expressions.push(self.parse_expression());

      match self.eat() {
        Token::Eol => (),
        // Last EOL in a block is optional
        Token::RBracket => break,
        _ => panic!("Unexpected token"),
      }
    }

    expressions
  }

  // assignment : let identifier ':' identifier '=' expression    ; Initialization
  //            | let identifier ':' identifier                   ; Initialization
  //            | identifier '=' expression                       ; Reassignment
  fn parse_assignment(&mut self) -> Assignment {
    match &self.eat().clone() {
      Token::Let => {
        let ident = match &self.eat() {
          Token::Identifier(ident) => ident.clone(),
          _ => panic!("Expected identifier"),
        };

        if !matches!(self.eat(), Token::Colon) {
          panic!("Expected colon");
        }

        let type_ident = match &self.eat() {
          Token::Identifier(ident) => ident.clone(),
          _ => panic!("Expected identifier"),
        };

        if !matches!(self.get(), Token::Assignment) {
          return Assignment::Initialization(ident, type_ident, None);
        };

        self.eat(); // eat '='

        let value = self.parse_expression();
        Assignment::Initialization(ident, type_ident, Some(Box::new(value)))
      }
      Token::Identifier(ident) => {
        if !matches!(self.eat(), Token::Assignment) {
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
      let op = match self.get() {
        Token::Add => TermOp::Add,
        Token::Subtract => TermOp::Subtract,
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
      let op = match self.get() {
        Token::Multiply => FactorOp::Multiply,
        Token::Divide => FactorOp::Divide,
        Token::Modulo => FactorOp::Modulo,
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

    match &tok {
      Token::Identifier(value) => Leaf::Identifier(value.to_owned()),
      Token::FloatLiteral(value) => Leaf::FloatLiteral(value.to_owned()),
      Token::LParen => {
        let term = self.parse_term();
        match self.eat() {
          Token::RParen => (),
          _ => panic!("Unexpected token"),
        };
        Leaf::Term(Box::new(term))
      }
      _ => panic!("Unexpected token"),
    }
  }
}
