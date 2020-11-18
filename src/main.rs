#[macro_use]
extern crate enum_kinds;

mod codegen;
mod compilation_error;
mod lexer;
mod parser;
mod position;
mod print;
mod semantic_analyzer;

use clap::{App, Arg};
use codegen::codegen::codegen_program;
use compilation_error::CompilationError;
use lexer::lexer::Lexer;
use parser::ast_json::JsonBuilder;
use parser::parser::Parser;
use semantic_analyzer::semantic_analyzer::SemanticAnalysis;
use std::env;
use std::fs;
use std::process;
use std::rc::Rc;

fn print_error(e: CompilationError, input: &str) {
    print::print_message_with_context(
        &e.get_message()[..],
        print::Status::Error,
        e.get_position(),
        e.get_length(),
        input,
    );
}

fn main() {
    let args = App::new("Caravel Compiler")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sawyer Herbst <herbst.sawyer@gmail.com>")
        .before_help(&*format!("\n{}", include_str!("../logo.txt")))
        .arg(
            Arg::with_name("input")
                .help("The file to compile")
                .index(1)
                .required(true),
        )
        .get_matches();

    let filename = args.value_of("input").expect("Must specify filename");
    let input = fs::read_to_string(filename).expect("Failed to read file.");

    print::print_message(&format!("Compiling {}.", filename)[..], print::Status::Ok);

    let mut lexer = Lexer::new(&input[..]);
    let tokens = lexer.lex();

    let tokens = match tokens {
        Ok(tokens) => tokens,
        Err(e) => {
            print_error(e, &input[..]);
            process::exit(1);
        }
    };
    print::print_message(
        &format!("Generated {} token(s)", tokens.len())[..],
        print::Status::Info,
    );

    for token in tokens.as_slice() {
        println!("Token: {} ({})", token.value, token.position);
    }

    let mut parser = Parser::new(tokens);
    let block_node = parser.parse();

    let block_node = match block_node {
        Ok(node) => node,
        Err(e) => {
            print_error(e, &input[..]);
            process::exit(1);
        }
    };

    fs::write("debug.json", block_node.to_json()).unwrap();

    print::print_message("Saved AST json", print::Status::Ok);

    block_node.analyze_semantics().unwrap();
    codegen_program(block_node);

    print::print_message(
        &format!("Done compiling {}", filename)[..],
        print::Status::Ok,
    );
}
