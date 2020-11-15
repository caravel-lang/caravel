use crate::lexer::token::Token;
use std::rc::Rc;

#[derive(Clone)]
pub enum ExpressionValue {
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),

    BinaryOp(Rc<ExpressionNode>, Token, Rc<ExpressionNode>),
}

#[derive(Clone)]
pub struct ExpressionNode {
    value: ExpressionValue,
}

impl ExpressionNode {
    pub fn new(value: ExpressionValue) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> ExpressionValue {
        self.value.clone()
    }
}

pub struct BodyNode {
    statements: Vec<ExpressionNode>,
}

impl BodyNode {
    pub fn new(statements: Vec<ExpressionNode>) -> Self {
        BodyNode { statements }
    }

    pub fn get_statements(&self) -> Vec<ExpressionNode> {
        self.statements.clone()
    }
}
