use crate::lexer::token::TokenType;
use crate::parser::ast::{Factor, Leaf, Term};
use std::convert::From;

/// Display AST as a tree
pub fn print(term: Term) {
  let node = &term.into();
  print_tree(node, String::new(), true);
}

// Adapted from https://vallentin.dev/2019/05/14/pretty-print-tree
enum Node {
  Leaf(String),
  Tree(String, Vec<Node>),
}

impl From<Term> for Node {
  fn from(term: Term) -> Self {
    match term {
      Term::Factor(factor) => factor.into(),
      Term::Operation(lhs, op, rhs) => {
        let op_name = match op.token_type {
          TokenType::Add => "Add",
          TokenType::Subtract => "Sub",
          _ => unreachable!(),
        };

        Node::Tree(op_name.to_owned(), vec![Node::from(*lhs), rhs.into()])
      }
    }
  }
}

impl From<Factor> for Node {
  fn from(factor: Factor) -> Self {
    match factor {
      Factor::Leaf(leaf) => leaf.into(),
      Factor::Operation(lhs, op, rhs) => {
        let op_name = match op.token_type {
          TokenType::Multiply => "Mul",
          TokenType::Divide => "Div",
          TokenType::Modulo => "Mod",
          _ => unreachable!(),
        };

        Node::Tree(op_name.to_owned(), vec![Node::from(*lhs), rhs.into()])
      }
    }
  }
}

impl From<Leaf> for Node {
  fn from(leaf: Leaf) -> Self {
    match leaf {
      Leaf::Identifier(val) => Node::Leaf(val),
      Leaf::FloatLiteral(val) => Node::Leaf(val),
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
