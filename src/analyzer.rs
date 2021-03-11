use crate::error::{Error, ErrorKind, Result};
use crate::lexer::token::Token;
use crate::parser::ast::*;
use crate::symbol_table::SymbolTable;
use crate::types::Type;
use std::convert::TryFrom;

pub struct Analyzer<'a> {
  symbol_table: SymbolTable,
  tokens: &'a Vec<Token>,
}

impl<'a> Analyzer<'a> {
  pub fn new(tokens: &'a Vec<Token>) -> Self {
    Self {
      symbol_table: SymbolTable::new(None),
      tokens,
    }
  }

  pub fn analyze(&mut self, program: &Block) -> Result<Type> {
    self.analyze_block(program)
  }

  fn analyze_expression(&mut self, expr: &Expression) -> Result<Type> {
    match expr {
      Expression::Assignment(assig) => self.analyze_assignment(assig),
      Expression::Block(block) => self.analyze_block(block),
      Expression::Term(term) => self.analyze_term(term),
    }
  }

  fn analyze_block(&mut self, block: &Block) -> Result<Type> {
    for (i, expr) in block.expressions.iter().enumerate() {
      let expr_type = self.analyze_expression(expr);
      if i == block.expressions.len() - 1 {
        return expr_type;
      }
    }
    // Void type if no expressions in block
    Ok(Type::Void)
  }

  fn analyze_assignment(&mut self, assig: &Assignment) -> Result<Type> {
    match assig {
      Assignment::Initialization(ident, type_ident, val, _) => {
        if self.symbol_table.has(&ident) {
          return Err(Error::new(
            ErrorKind::Redeclaration,
            &format!(r#"declaration of previously declared variable "{}""#, ident),
            assig.pos().as_source_span(self.tokens),
          ));
        };
        let typ = Type::try_from(type_ident.to_owned()).unwrap();
        if let Some(val) = val {
          let val_type = self.analyze_expression(val)?;
          if val_type != typ {
            return Err(Error::new(
              ErrorKind::TypeMismatch,
              "type mismatch",
              assig.pos().as_source_span(self.tokens),
            ));
          }
        };
        self.symbol_table.set(&ident, typ);
        Ok(typ)
      }
      Assignment::Reassignment(ident, val) => {
        let typ = self.analyze_expression(val)?;
        match self.symbol_table.get(&ident) {
          None => {
            return Err(Error::new(
              ErrorKind::UndeclaredVariable,
              &format!(r#"use of undeclared variable "{}""#, ident),
              assig.pos().as_source_span(self.tokens),
            ))
          }
          Some(cur_type) => {
            if cur_type != typ {
              return Err(Error::new(
                ErrorKind::TypeMismatch,
                "type mismatch",
                assig.pos().as_source_span(self.tokens),
              ));
            }
          }
        }
        Ok(typ)
      }
    }
  }

  fn analyze_term(&self, term: &Term) -> Result<Type> {
    match term {
      Term::Factor(factor) => self.analyze_factor(factor),
      Term::Operation(lhs, _, rhs) => {
        let lhs_type = self.analyze_term(lhs)?;
        let rhs_type = self.analyze_factor(rhs)?;
        if lhs_type != rhs_type {
          return Err(Error::new(
            ErrorKind::TypeMismatch,
            "type mismatch",
            term.pos().as_source_span(self.tokens),
          ));
        }
        Ok(lhs_type)
      }
    }
  }

  fn analyze_factor(&self, factor: &Factor) -> Result<Type> {
    match factor {
      Factor::Leaf(leaf) => self.analyze_leaf(leaf),
      Factor::Operation(lhs, _, rhs) => {
        let lhs_type = self.analyze_factor(lhs)?;
        let rhs_type = self.analyze_leaf(rhs)?;
        if lhs_type != rhs_type {
          return Err(Error::new(
            ErrorKind::TypeMismatch,
            "type mismatch",
            factor.pos().as_source_span(self.tokens),
          ));
        }
        Ok(lhs_type)
      }
    }
  }

  fn analyze_leaf(&self, leaf: &Leaf) -> Result<Type> {
    Ok(match leaf {
      Leaf::FloatLiteral(_, _) => Type::Float,
      Leaf::Identifier(ident, _) => match self.symbol_table.get(&ident) {
        Some(typ) => typ,
        None => {
          return Err(Error::new(
            ErrorKind::UndeclaredVariable,
            &format!(r#"use of undeclared variable "{}""#, ident),
            leaf.pos().as_source_span(self.tokens),
          ));
        }
      },
      Leaf::Term(term) => self.analyze_term(term)?,
    })
  }
}
