#![allow(dead_code)]
#![allow(unused)]
use std::{env::args, fs};

use lexer::Lexer;
use parser::Parser;
use token::TokenType;

use crate::emitter::Emitter;

mod lexer;
mod parser;
mod emitter;
mod token;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments provided.");
    }

    let contents = fs::read_to_string(&args[1]).expect("Could not open file");

    let mut lexer = Lexer::new(contents);
    let mut emitter = Emitter::new(format!("{}.c", &args[1]));
    let mut parser = Parser::new(lexer, &mut emitter);

    parser.program();
    emitter.write_file();
    println!("Compiling completed.")
}
