use crate::parser::ast::*;
use crate::symbol_table::SymbolTable;
use crate::types::Type;

pub fn analyze(program: &Block) -> Type {
  analyze_block(program, &mut SymbolTable::new(None))
}

fn analyze_expression(expr: &Expression, table: &mut SymbolTable) -> Type {
  match expr {
    Expression::Assignment(assig) => analyze_assignment(assig, table),
    Expression::Block(block) => analyze_block(block, table),
  }
}

fn analyze_block(block: &Block, table: &mut SymbolTable) -> Type {
  for (i, expr) in block.iter().enumerate() {
    let expr_type = analyze_expression(expr, table);
    if i == block.len() - 1 {
      return expr_type;
    }
  }

  // Void type if no expressions in block
  Type::Void
}

fn analyze_assignment(assig: &Assignment, table: &mut SymbolTable) -> Type {
  match assig {
    Assignment::DefaultAssignment(ident, typ_ident) => {
      let typ = match &typ_ident[..] {
        "float" => Type::Float,
        "void" => Type::Void,
        _ => panic!("Unknown type"),
      };

      if table.has(ident) {
        panic!("Can't redeclare variable")
      }

      typ
    }
    Assignment::Assignment(ident, typ_ident, value) => {
      let typ = match &typ_ident[..] {
        "float" => Type::Float,
        "void" => Type::Void,
        _ => panic!("Unknown type"),
      };
      let term_type = analyze_term(value, table);
      if term_type != typ {
        panic!("Type mismatch")
      }

      if table.has(ident) {
        panic!("Can't redeclare variable")
      }

      table.set(ident, typ);
      typ
    }
    Assignment::Reassignment(ident, value) => {
      let cur_type = match table.get(ident) {
        Some(typ) => typ,
        None => panic!("Uknown identifier"),
      };

      let typ = analyze_term(value, table);
      if typ != cur_type {
        panic!("Can't change type of variable")
      }

      table.set(ident, typ);
      typ
    }
    Assignment::Term(term) => analyze_term(term, table),
  }
}

fn analyze_term(term: &Term, table: &mut SymbolTable) -> Type {
  match term {
    Term::Factor(factor) => analyze_factor(factor, table),
    Term::Operation(lhs, op, rhs) => {
      let lhs_type = analyze_term(lhs, table);
      let rhs_type = analyze_factor(rhs, table);

      if lhs_type != rhs_type {
        panic!("Type mismatch")
      }

      lhs_type
    }
  }
}

fn analyze_factor(factor: &Factor, table: &mut SymbolTable) -> Type {
  match factor {
    Factor::Leaf(leaf) => analyze_leaf(leaf, table),
    Factor::Operation(lhs, op, rhs) => {
      let lhs_type = analyze_factor(lhs, table);
      let rhs_type = analyze_leaf(rhs, table);

      if lhs_type != rhs_type {
        panic!("Type mismatch")
      }

      lhs_type
    }
  }
}

fn analyze_leaf(leaf: &Leaf, table: &mut SymbolTable) -> Type {
  match leaf {
    Leaf::FloatLiteral(_) => Type::Float,
    Leaf::Identifier(ident) => match table.get(ident) {
      Some(typ) => typ,
      None => panic!("Unknown identifier"),
    },
    Leaf::Term(term) => analyze_term(term, table),
  }
}
