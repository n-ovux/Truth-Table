mod ast;
mod tokenizer;

use crate::tokenizer::Token;

fn main() {
    let mut arguments: std::env::Args = std::env::args();
    if arguments.len() <= 1 {
        panic!("Not enough arguments!");
    }
    arguments.next();

    // Tokenize Input
    let text = arguments.collect();
    let tokens = Token::tokenize(&text).unwrap_or_else(|error| {
        panic!("Could not tokenize input: {}", error);
    });

    // Error Checker
    Token::verify(&tokens).unwrap_or_else(|error| {
        panic!("Invalid Input: {}", error);
    });

    println!("{:?}", tokens);

    // Generate AST
}
