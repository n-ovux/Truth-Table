mod ast;
mod lexer;
mod tree;

use ast::{Grammar, AST};
use tree::Tree;

fn main() {
    let mut arguments: std::env::Args = std::env::args();
    if arguments.len() <= 1 {
        panic!("Not enough arguments!");
    }
    arguments.next();

    // Tokenize Input
    let text = arguments.collect();
    let (tokens, variables) = lexer::lexer(&text).unwrap_or_else(|error| {
        panic!("Invalid input: {}", error);
    });

    println!("{:?}", tokens);
    println!("{:?}", variables);

    // Generate AST
    let mut ast = Tree::new(Grammar::Root);
    ast.create_ast(&tokens);
    println!("\nFinal!\n{}", ast);
}
