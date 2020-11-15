use super::ast;
use crate::compilation_error::CompilationError;
use crate::lexer::token::{Token, TokenValue, TokenValueKind};
use crate::position::Position;
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    index: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: -1 }
    }

    pub fn parse(&mut self) -> Result<ast::BodyNode, CompilationError> {
        self.parse_program()
    }

    //==============================================//
    // Token Retrieval Functions                    //
    //==============================================//

    /// Don't call directly! Use get_token() or peek()
    fn get_token_with_offset(&self, offset: i32) -> Result<Token, CompilationError> {
        let mut tokens = self.tokens.iter().cloned();
        let token = tokens.nth((self.index + offset) as usize);

        let token = match token {
            None => {
                return Err(CompilationError::new(
                    &format!("Expected token at index {}", self.index + offset)[..],
                    Position::pre(),
                    1,
                ))
            }

            Some(token) => token,
        };

        Ok(token)
    }

    fn get_token(&self) -> Result<Token, CompilationError> {
        self.get_token_with_offset(0)
    }

    /// Advances index and returns token
    fn eat(&mut self) -> Result<Token, CompilationError> {
        self.index += 1;
        Ok(self.get_token()?)
    }

    fn eat_expect(&mut self, ttype: TokenValueKind) -> Result<Token, CompilationError> {
        let tok = self.eat()?;

        if tok.value.get_kind() != ttype {
            return Err(CompilationError::new(
                "Found token of unexpected kind.",
                Position::pre(),
                1,
            ));
        };

        Ok(tok)
    }

    fn eat_if_match(&mut self, kinds: Vec<TokenValueKind>) -> Result<bool, CompilationError> {
        let next = self.peek()?;

        if kinds.contains(&next.value.get_kind()) {
            self.eat()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn peek(&self) -> Result<Token, CompilationError> {
        self.get_token_with_offset(1)
    }

    //==============================================//
    // Parse functions                              //
    //==============================================//

    /// program = body
    fn parse_program(&mut self) -> Result<ast::BodyNode, CompilationError> {
        self.parse_body()
    }

    /// body = (expression EOL)*
    fn parse_body(&mut self) -> Result<ast::BodyNode, CompilationError> {
        let mut statements = vec![];

        while self.index + 1 < self.tokens.len() as i32 {
            statements.push(self.parse_expression()?);
            self.eat_expect(TokenValueKind::EOL)?;
        }

        Ok(ast::BodyNode::new(statements))
    }

    /// expression = term
    fn parse_expression(&mut self) -> Result<ast::ExpressionNode, CompilationError> {
        self.parse_term()
    }

    /// term = factor (('+' | '-') factor)*
    fn parse_term(&mut self) -> Result<ast::ExpressionNode, CompilationError> {
        let mut lhs = self.parse_factor()?;

        while self.eat_if_match(vec![TokenValueKind::Addition, TokenValueKind::Subtraction])? {
            let op = self.get_token()?;
            let rhs = self.parse_factor()?;

            lhs = ast::ExpressionNode::new(ast::ExpressionValue::BinaryOp(
                Rc::new(lhs.clone()),
                op,
                Rc::new(rhs),
            ));
        }

        Ok(lhs)
    }

    /// factor = power (('*' | '/' | '%') power)*
    fn parse_factor(&mut self) -> Result<ast::ExpressionNode, CompilationError> {
        let mut lhs = self.parse_power()?;

        while self.eat_if_match(vec![
            TokenValueKind::Multiplication,
            TokenValueKind::Division,
            TokenValueKind::Modulo,
        ])? {
            let op = self.get_token()?;
            let rhs = self.parse_power()?;

            lhs = ast::ExpressionNode::new(ast::ExpressionValue::BinaryOp(
                Rc::new(lhs.clone()),
                op,
                Rc::new(rhs),
            ));
        }

        Ok(lhs)
    }

    // right-recursion so that exponents are right-associative
    /// power = leaf ('^' power)*
    fn parse_power(&mut self) -> Result<ast::ExpressionNode, CompilationError> {
        let mut lhs = self.parse_leaf()?;

        while self.eat_if_match(vec![TokenValueKind::Exponentiation])? {
            let op = self.get_token()?;
            let rhs = self.parse_power()?;

            lhs = ast::ExpressionNode::new(ast::ExpressionValue::BinaryOp(
                Rc::new(lhs.clone()),
                op,
                Rc::new(rhs),
            ));
        }

        Ok(lhs)
    }

    /// leaf = INT_LITERAL
    ///      | FLOAT_LITERAL
    ///      | STRING_LITERAL
    ///      | CHAR_LITERAL
    ///      | '(' expression ')'
    fn parse_leaf(&mut self) -> Result<ast::ExpressionNode, CompilationError> {
        if self.eat_if_match(vec![
            TokenValueKind::IntLiteral,
            TokenValueKind::FloatLiteral,
            TokenValueKind::StringLiteral,
            TokenValueKind::CharLiteral,
        ])? {
            let expr_value = match self.get_token()?.value {
                TokenValue::IntLiteral(val) => ast::ExpressionValue::IntLiteral(val),
                TokenValue::FloatLiteral(val) => ast::ExpressionValue::FloatLiteral(val),
                TokenValue::StringLiteral(val) => ast::ExpressionValue::StringLiteral(val),
                TokenValue::CharLiteral(val) => ast::ExpressionValue::CharLiteral(val),

                _ => unreachable!(),
            };

            Ok(ast::ExpressionNode::new(expr_value))
        } else if self.eat_if_match(vec![TokenValueKind::OpenParen])? {
            let expr = self.parse_expression()?;
            self.eat_expect(TokenValueKind::CloseParen)?;

            Ok(expr)
        } else {
            return Err(CompilationError::new(
                &format!("Unexpected token {}", self.eat()?.value)[..],
                Position::pre(),
                1,
            ));
        }
    }
}
