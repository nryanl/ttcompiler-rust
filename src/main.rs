#![allow(dead_code)]
#![allow(unused)]
use lexer::Lexer;
use token::TokenType;

mod lexer;
mod token;

fn main() {
    let source = "IF+-123 foo*THEN/";
    let mut lexer = Lexer::new(source.to_string());

    let mut token = lexer.get_token();
    while token.kind != TokenType::Eof {
        println!("{:?}", token.kind);
        token = lexer.get_token();
    }
}
