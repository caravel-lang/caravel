pub mod lexer;
pub mod position;

use lexer::lexer::Lexer;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.cv").unwrap();

    let lexer = Lexer::new(&input);
    let tokens = lexer.lex();

    tokens
        .iter()
        .for_each(|tok| println!("{:?}", tok.token_type));
}
