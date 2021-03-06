pub mod backend;
pub mod lexer;
pub mod parser;
pub mod position;

use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.cv").unwrap();

    let lexer = Lexer::new(&input);
    let tokens = lexer.lex();

    let parser = Parser::new(tokens);
    let term = parser.parse();

    backend::print::print(term);
}
