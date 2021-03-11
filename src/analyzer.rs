use crate::error::{Error, ErrorKind, Result};
use crate::parser::ast::*;
use crate::symbol_table::SymbolTable;
use crate::types::Type;
use std::convert::TryFrom;

pub fn analyze(program: &Block) -> Result<Type> {
  analyze_block(program, &mut SymbolTable::new(None))
}

fn analyze_expression(expr: &Expression, table: &mut SymbolTable) -> Result<Type> {
  match expr {
    Expression::Assignment(assig) => analyze_assignment(assig, table),
    Expression::Block(block) => analyze_block(block, table),
    Expression::Term(term) => analyze_term(term, table),
  }
}

fn analyze_block(block: &Block, table: &mut SymbolTable) -> Result<Type> {
  for (i, expr) in block.iter().enumerate() {
    let expr_type = analyze_expression(expr, table);
    if i == block.len() - 1 {
      return expr_type;
    }
  }

  // Void type if no expressions in block
  Ok(Type::Void)
}

//
fn analyze_assignment(assig: &Assignment, table: &mut SymbolTable) -> Result<Type> {
  match assig {
    Assignment::Initialization(ident, type_ident, val) => {
      if table.has(&ident.value) {
        panic!("Declaration of previously declared variable")
      };

      let typ = Type::try_from(type_ident.value.to_owned()).unwrap();

      if let Some(val) = val {
        let val_type = analyze_expression(val, table)?;

        if val_type != typ {
          panic!("Type mismatch")
        }
      };

      table.set(&ident.value, typ);
      Ok(typ)
    }
    Assignment::Reassignment(ident, val) => {
      let typ = analyze_expression(val, table)?;
      match table.get(&ident.value) {
        None => panic!("Use of undeclared variable"),
        Some(cur_type) => {
          if cur_type != typ {
            panic!("Type mismatch")
          }
        }
      }

      Ok(typ)
    }
  }
}

fn analyze_term(term: &Term, table: &mut SymbolTable) -> Result<Type> {
  match term {
    Term::Factor(factor) => analyze_factor(factor, table),
    Term::Operation(lhs, _, rhs) => {
      let lhs_type = analyze_term(lhs, table)?;
      let rhs_type = analyze_factor(rhs, table)?;

      if lhs_type != rhs_type {
        panic!("Type mismatch")
      }

      Ok(lhs_type)
    }
  }
}

fn analyze_factor(factor: &Factor, table: &mut SymbolTable) -> Result<Type> {
  match factor {
    Factor::Leaf(leaf) => analyze_leaf(leaf, table),
    Factor::Operation(lhs, _, rhs) => {
      let lhs_type = analyze_factor(lhs, table)?;
      let rhs_type = analyze_leaf(rhs, table)?;

      if lhs_type != rhs_type {
        panic!("Type mismatch")
      }

      Ok(lhs_type)
    }
  }
}

fn analyze_leaf(leaf: &Leaf, table: &mut SymbolTable) -> Result<Type> {
  Ok(match leaf {
    Leaf::FloatLiteral(_) => Type::Float,
    Leaf::Identifier(ident) => match table.get(&ident.value) {
      Some(typ) => typ,
      None => panic!("Unknown identifier"),
    },
    Leaf::Term(term) => analyze_term(term, table)?,
  })
}
