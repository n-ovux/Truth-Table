mod ast;
mod tokenizer;
mod tree;

use crate::ast::{Grammar, AST};
use crate::tokenizer::Token;
use crate::tree::Tree;

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
    let ast: Tree<Grammar> = Tree::<Grammar>::create_ast(&tokens);
    println!("{}", ast);
}
