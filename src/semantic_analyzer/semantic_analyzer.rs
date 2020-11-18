use crate::compilation_error::CompilationError;
use crate::lexer::token::TokenValueKind;
use crate::parser::ast;
use crate::parser::ast::{ExpressionValue, ExpressionValueKind};
use crate::position::Position;

pub trait SemanticAnalysis {
    fn analyze_semantics(&self) -> Result<ExpressionValueKind, CompilationError>;
}

impl SemanticAnalysis for ast::ExpressionNode {
    fn analyze_semantics(&self) -> Result<ExpressionValueKind, CompilationError> {
        Ok(match self.get_value() {
            ExpressionValue::BinaryOp(lhs, op, rhs) => {
                let lhs_ty = ExpressionValueKind::from(lhs.get_value());
                let rhs_ty = ExpressionValueKind::from(rhs.get_value());

                if lhs_ty != rhs_ty {
                    return Err(CompilationError::new("Type mismatch", Position::pre(), 1));
                }

                if op.value.get_kind() == TokenValueKind::Equals {
                    return Ok(ExpressionValueKind::BoolLiteral);
                }

                lhs_ty
            }

            ExpressionValue::IntLiteral(_) => ExpressionValueKind::IntLiteral,
            ExpressionValue::FloatLiteral(_) => ExpressionValueKind::FloatLiteral,
            ExpressionValue::StringLiteral(_) => ExpressionValueKind::StringLiteral,
            ExpressionValue::CharLiteral(_) => ExpressionValueKind::CharLiteral,
            ExpressionValue::BoolLiteral(_) => ExpressionValueKind::BoolLiteral,

            ExpressionValue::Debug(expr) => ExpressionValueKind::from(expr.get_value()),
            ExpressionValue::Assignment(_, expr) => ExpressionValueKind::from(expr.get_value()),

            ExpressionValue::Identifier(_) => ExpressionValueKind::Identifier,
        })
    }
}

impl SemanticAnalysis for ast::BlockNode {
    fn analyze_semantics(&self) -> Result<ExpressionValueKind, CompilationError> {
        let statements = self.get_statements();

        for statement in self.get_statements() {
            statement.analyze_semantics()?;
        }

        Ok(statements[statements.len() - 1].analyze_semantics()?)
    }
}
