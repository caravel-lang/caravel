pub mod analyzer;
pub mod backend;
pub mod lexer;
pub mod parser;
pub mod position;
pub mod symbol_table;
pub mod types;

use analyzer::analyze;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.cv").unwrap();

    println!("Lexing...");
    let lexer = Lexer::new(&input);
    let tokens = lexer.lex();

    println!("Parsing...");
    let parser = Parser::new(tokens);
    let block = parser.parse();

    println!("Analyzing...");
    let prog_type = analyze(&block);
    println!("Type: {:?}", prog_type);

    backend::print::print(&block);
}
