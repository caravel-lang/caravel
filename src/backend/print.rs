use crate::lexer::token::TokenType;
use crate::parser::ast::*;
use std::convert::From;

/// Display AST as a tree
pub fn print(block: &Block) {
  let node = block.into();
  print_tree(&node, String::new(), true);
}

// Adapted from https://vallentin.dev/2019/05/14/pretty-print-tree
enum Node {
  Leaf(String),
  Tree(String, Vec<Node>),
}

impl From<&Expression> for Node {
  fn from(expression: &Expression) -> Self {
    match expression {
      Expression::Assignment(assignment) => assignment.into(),
      Expression::Block(block) => block.into(),
    }
  }
}

impl From<&Block> for Node {
  fn from(block: &Block) -> Self {
    let expressions = block.iter().map(|expr| expr.into()).collect();
    Node::Tree("Block".to_owned(), expressions)
  }
}

impl From<&Assignment> for Node {
  fn from(assignment: &Assignment) -> Self {
    match assignment {
      Assignment::Assignment(identifier, typ, rhs) => Self::Tree(
        "Assig".to_owned(),
        vec![Node::Leaf(identifier.to_owned()), rhs.into()],
      ),
      Assignment::DefaultAssignment(identifier, typ) => Self::Tree(
        "DeAssig".to_owned(),
        vec![Node::Leaf(identifier.to_owned())],
      ),
      Assignment::Reassignment(identifier, rhs) => Self::Tree(
        "Reass".to_owned(),
        vec![Node::Leaf(identifier.to_owned()), rhs.into()],
      ),
      Assignment::Term(term) => term.into(),
    }
  }
}

impl From<&Term> for Node {
  fn from(term: &Term) -> Self {
    match term {
      Term::Factor(factor) => factor.into(),
      Term::Operation(lhs, op, rhs) => {
        let op_name = match op {
          TermOp::Add => "Add",
          TermOp::Subtract => "Sub",
        };

        Self::Tree(op_name.to_owned(), vec![Self::from(&**lhs), rhs.into()])
      }
    }
  }
}

impl From<&Factor> for Node {
  fn from(factor: &Factor) -> Self {
    match factor {
      Factor::Leaf(leaf) => leaf.into(),
      Factor::Operation(lhs, op, rhs) => {
        let op_name = match op {
          FactorOp::Multiply => "Mul",
          FactorOp::Divide => "Div",
          FactorOp::Modulo => "Mod",
        };

        Self::Tree(op_name.to_owned(), vec![Self::from(&**lhs), rhs.into()])
      }
    }
  }
}

impl From<&Leaf> for Node {
  fn from(leaf: &Leaf) -> Self {
    match leaf {
      Leaf::Identifier(val) => Self::Leaf(val.to_owned()),
      Leaf::FloatLiteral(val) => Self::Leaf(val.to_owned()),
      Leaf::Term(term) => Self::from(&**term),
    }
  }
}

fn print_tree(node: &Node, prefix: String, last: bool) {
  let prefix_current = if last { "`- " } else { "|- " };

  print!("{}{}", prefix, prefix_current);

  let prefix_child = if last { "   " } else { "|  " };
  let prefix = prefix + prefix_child;

  match node {
    Node::Leaf(value) => println!("{}", value),
    Node::Tree(name, children) => {
      println!("{}", name);
      for (i, child) in children.iter().enumerate() {
        print_tree(child, prefix.to_string(), i + 1 == children.len())
      }
    }
  }
}
