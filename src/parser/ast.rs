use crate::lexer::token::Token;
use std::rc::Rc;

#[derive(Clone, EnumKind)]
#[enum_kind(ExpressionValueKind)]
pub enum ExpressionValue {
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),

    BinaryOp(Rc<ExpressionNode>, Token, Rc<ExpressionNode>),

    Debug(Rc<ExpressionNode>),
    Assignment(String, Rc<ExpressionNode>),

    Identifier(String),
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

// We could have made this part of the ExpressionValue enum
// but this way some functions can only accept expressions
// and vice versa
pub struct BlockNode {
    statements: Vec<ExpressionNode>,
}

impl BlockNode {
    pub fn new(statements: Vec<ExpressionNode>) -> Self {
        BlockNode { statements }
    }

    pub fn get_statements(&self) -> Vec<ExpressionNode> {
        self.statements.clone()
    }
}
