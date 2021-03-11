use super::ast::*;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::token::{Token, TokenKind};
use crate::position::source_position::DEFAULT_REAL_SPAN;

const EOF_TOKEN: Token = Token {
  kind: TokenKind::Eof,
  pos: DEFAULT_REAL_SPAN,
};

pub struct Parser<'a> {
  tokens: &'a Vec<Token>,
  index: usize,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: &'a Vec<Token>) -> Self {
    Self { tokens, index: 0 }
  }

  pub fn parse(mut self) -> Result<Block> {
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
    self.get()
  }

  #[allow(unused_variables)]
  fn eat_if_get(&mut self, kind: TokenKind, expected: &str) -> Result<&Token> {
    if !matches!(self.get(), kind) {
      self.throw_unexpected_token_msg(&format!("expected {}", expected))?;
    };
    Ok(self.eat())
  }

  fn throw_unexpected_token(&self) -> Result<!> {
    self.throw_unexpected_token_msg("unexpected token")
  }

  fn throw_unexpected_token_msg(&self, msg: &str) -> Result<!> {
    Err(Error::new(
      ErrorKind::UnexpectedToken,
      msg,
      self.get().pos.clone(),
    ))
  }

  fn get_and_eat(&mut self) -> &Token {
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
  fn parse_expression(&mut self) -> Result<Expression> {
    Ok(match self.get().kind {
      TokenKind::LBracket => Expression::Block(self.parse_block()?),
      TokenKind::Let => Expression::Assignment(self.parse_assignment()?),
      TokenKind::Identifier(_) => {
        if let TokenKind::Assignment = self.peek().kind {
          Expression::Assignment(self.parse_assignment()?)
        } else {
          Expression::Term(self.parse_term()?)
        }
      }
      _ => Expression::Term(self.parse_term()?),
    })
  }

  // block : '{' ((expression)? EOL)* (expression)? '}'
  fn parse_block(&mut self) -> Result<Block> {
    let start_index = self.index;
    self.eat(); // '{'

    let mut expressions = Vec::new();
    loop {
      match self.get().kind {
        TokenKind::RBracket => break,
        // Allow empty lines
        TokenKind::Eol => {
          self.eat();
          continue;
        }
        _ => (),
      };

      expressions.push(self.parse_expression()?);

      match self.get().kind {
        TokenKind::Eol => self.eat(),
        // Last EOL in a block is optional
        TokenKind::RBracket => break,
        _ => self.throw_unexpected_token()?,
      };
    }

    Ok(Block {
      expressions,
      start_index,
    })
  }

  // assignment : let identifier ':' identifier '=' expression    ; Initialization
  //            | let identifier ':' identifier                   ; Initialization
  //            | identifier '=' expression                       ; Reassignment
  fn parse_assignment(&mut self) -> Result<Assignment> {
    Ok(match self.get_and_eat().kind.clone() {
      TokenKind::Let => {
        let start_index = self.index - 1;

        let ident = match &self.get().kind {
          TokenKind::Identifier(ident) => ident.clone(),
          _ => self.throw_unexpected_token_msg("expected identifier")?,
        };

        self.eat();

        self.eat_if_get(TokenKind::Colon, "':'")?;

        let type_ident = match &self.get().kind {
          TokenKind::Identifier(ident) => ident.clone(),
          _ => self.throw_unexpected_token_msg("expected identifier")?,
        };

        self.eat();

        if !matches!(self.get().kind, TokenKind::Assignment) {
          return Ok(Assignment::Initialization(
            ident,
            type_ident,
            None,
            start_index,
          ));
        };

        self.eat();

        let value = self.parse_expression()?;
        Assignment::Initialization(ident, type_ident, Some(Box::new(value)), start_index)
      }
      TokenKind::Identifier(ident) => {
        self.eat_if_get(TokenKind::Assignment, "'='")?;
        let value = self.parse_expression()?;
        Assignment::Reassignment(ident.to_owned(), Box::new(value))
      }
      _ => self.throw_unexpected_token()?,
    })
  }

  // term : factor (('+' | '-') factor)*
  fn parse_term(&mut self) -> Result<Term> {
    let mut term = Term::Factor(self.parse_factor()?);

    loop {
      let op = match self.get().kind {
        TokenKind::Add => TermOp::Add,
        TokenKind::Subtract => TermOp::Subtract,
        _ => break,
      };

      self.eat();

      let rhs = self.parse_factor()?;
      term = Term::Operation(Box::new(term), op, rhs);
    }

    Ok(term)
  }

  // factor : leaf (('*' | '/' | '%') leaf)*
  fn parse_factor(&mut self) -> Result<Factor> {
    let mut factor = Factor::Leaf(self.parse_leaf()?);

    loop {
      let op = match self.get().kind {
        TokenKind::Multiply => FactorOp::Multiply,
        TokenKind::Divide => FactorOp::Divide,
        TokenKind::Modulo => FactorOp::Modulo,
        _ => break,
      };

      self.eat();

      let rhs = self.parse_leaf()?;
      factor = Factor::Operation(Box::new(factor), op, rhs);
    }

    Ok(factor)
  }

  // leaf : identifier
  //      | float_literal
  fn parse_leaf(&mut self) -> Result<Leaf> {
    Ok(match self.get_and_eat().kind.clone() {
      TokenKind::Identifier(value) => Leaf::Identifier(value.to_owned(), self.index - 1),
      TokenKind::FloatLiteral(value) => Leaf::FloatLiteral(value.to_owned(), self.index - 1),
      TokenKind::LParen => {
        let term = self.parse_term()?;
        self.eat_if_get(TokenKind::RParen, "')'")?;
        Leaf::Term(Box::new(term))
      }
      _ => self.throw_unexpected_token()?,
    })
  }
}
