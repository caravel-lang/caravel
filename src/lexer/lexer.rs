use super::token::{Token, TokenValue};
use crate::compilation_error::CompilationError;
use crate::position::Position;
use std::vec::Vec;

pub struct Lexer {
    input: String,
    position: Position,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut input_with_newline = input.to_owned();
        input_with_newline.push('\n');

        return Lexer {
            input: input_with_newline,
            position: Position::pre(),
        };
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, CompilationError> {
        let mut tokens = Vec::<Token>::new();

        loop {
            match self.parse_token()? {
                Some(token) => tokens.push(token),
                None => (),
            };

            if self.position.get_index() as usize + 1 == self.input.chars().count() {
                break;
            }
        }

        Ok(tokens)
    }

    //==============================================//
    // Char Retrieval Functions                     //
    //==============================================//

    fn get_char(&self) -> Result<char, CompilationError> {
        let char = self.input.chars().nth(self.position.get_index() as usize);

        match char {
            Some(char) => Ok(char),
            None => Err(CompilationError::new(
                &format!("Expected char at index {}.", self.position.get_index())[..],
                self.position,
                1,
            )),
        }
    }

    /// Advances position and return char
    fn eat(&mut self) -> Result<char, CompilationError> {
        // If at -1 index simply advance column
        if self.position.get_index() == -1 {
            self.position.next_column();
            return Ok(self.get_char()?);
        }

        let char = self.get_char()?;

        if char == '\n' {
            self.position.next_row();
        } else {
            self.position.next_column()
        }

        Ok(self.get_char()?)
    }

    fn eat_expect(&mut self, expected: char) -> Result<char, CompilationError> {
        let pos = self.position;
        let found = self.eat()?;

        if expected != found {
            Err(CompilationError::new(
                &format!("Expected char '{}', found '{}'.", expected, found)[..],
                pos,
                1,
            ))
        } else {
            Ok(found)
        }
    }

    fn peek(&self) -> Result<char, CompilationError> {
        self.input
            .chars()
            .nth(self.position.get_index() as usize + 1)
            .ok_or(CompilationError::new(
                &format!("Expected char at index {}", self.position.get_index() + 1)[..],
                self.position,
                1,
            ))
    }

    fn log_char(&self) {
        println!("Char: {}", self.get_char().unwrap());
    }

    //==============================================//
    // Parse functions                              //
    //==============================================//

    fn parse_token(&mut self) -> Result<Option<Token>, CompilationError> {
        let char = self.eat()?;

        if char == ' ' {
            return Ok(None);
        }

        let token = if char == '\'' {
            self.parse_string_literal()?
        } else if char == '"' {
            self.parse_char_literal()?
        } else if char.is_digit(10) {
            self.parse_number_literal()?
        } else {
            self.parse_single_char_token()?
        };

        Ok(Some(token))
    }

    /// Function to parse all single character tokens
    fn parse_single_char_token(&mut self) -> Result<Token, CompilationError> {
        let char = self.get_char()?;

        let value = match char {
            '\n' => TokenValue::EOL,

            '+' => TokenValue::Addition,
            '-' => TokenValue::Subtraction,
            '*' => TokenValue::Multiplication,
            '/' => TokenValue::Division,
            '%' => TokenValue::Modulo,
            '^' => TokenValue::Exponentiation,

            '(' => TokenValue::OpenParen,
            ')' => TokenValue::CloseParen,

            _ => {
                return Err(CompilationError::new(
                    &format!("Unexpected char {}.", char)[..],
                    self.position,
                    1,
                ))
            }
        };

        Ok(Token {
            value,
            position: self.position,
            length: 1,
            representation: char.to_string(),
        })
    }

    fn parse_string_literal(&mut self) -> Result<Token, CompilationError> {
        // Parse starts at the opening quote
        // Capture the start position
        let position = self.position;

        let mut value = "".to_owned();

        while self.eat()? != '\'' {
            value.push(self.get_char()?);
        }

        Ok(Token {
            value: TokenValue::StringLiteral(value.clone()),
            position,
            representation: value.clone(),
            length: value.chars().count() as i32 + 2,
        })
    }

    fn parse_char_literal(&mut self) -> Result<Token, CompilationError> {
        // Parse starts at the opening quote
        // Capture the start position
        let position = self.position;

        let char = self.eat()?;

        self.eat_expect('"')?;

        Ok(Token {
            value: TokenValue::CharLiteral(char),
            position,
            representation: char.to_string(),
            length: 3,
        })
    }

    /// Parses IntLiteral or FloatLiteral
    fn parse_number_literal(&mut self) -> Result<Token, CompilationError> {
        // start at the first digit
        // Capture the start position
        let position = self.position;

        let mut value = self.get_char()?.to_string();

        while self.peek()?.is_digit(10) || self.peek()? == '.' {
            value.push(self.eat()?);
        }

        if value.contains('.') {
            let floatval = value.parse::<f64>().or(Err(CompilationError::new(
                &format!("")[..],
                position,
                value.chars().count() as i32,
            )))?;

            Ok(Token {
                value: TokenValue::FloatLiteral(floatval),
                representation: value.clone(),
                position,
                length: value.chars().count() as i32,
            })
        } else {
            let intval = value.parse::<i64>().or(Err(CompilationError::new(
                &format!("")[..],
                position,
                value.chars().count() as i32,
            )))?;

            Ok(Token {
                value: TokenValue::IntLiteral(intval),
                representation: value.clone(),
                position,
                length: value.chars().count() as i32,
            })
        }
    }
}
