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
      Expression::Term(term) => term.into(),
    }
  }
}

impl From<&Block> for Node {
  fn from(block: &Block) -> Self {
    let expressions = block.expressions.iter().map(|expr| expr.into()).collect();
    Node::Tree("Block".to_owned(), expressions)
  }
}

impl From<&Assignment> for Node {
  fn from(assignment: &Assignment) -> Self {
    match assignment {
      Assignment::Initialization(ident, _, expr, _) => {
        let mut children = vec![Node::Leaf(ident.clone())];

        if let Some(expr) = expr {
          children.push(Node::from(&**expr));
        }

        Node::Tree("Initialization".to_owned(), children)
      }
      Assignment::Reassignment(ident, expr) => Node::Tree(
        "Reassignment".to_owned(),
        vec![Node::Leaf(ident.clone()), Node::from(&**expr)],
      ),
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
          TermOp::Subtract => "Subtract",
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
          FactorOp::Multiply => "Multiply",
          FactorOp::Divide => "Divide",
          FactorOp::Modulo => "Modulo",
        };

        Self::Tree(op_name.to_owned(), vec![Self::from(&**lhs), rhs.into()])
      }
    }
  }
}

impl From<&Leaf> for Node {
  fn from(leaf: &Leaf) -> Self {
    match leaf {
      Leaf::Identifier(val, _) => Self::Leaf(val.clone()),
      Leaf::FloatLiteral(val, _) => Self::Leaf(val.clone()),
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
