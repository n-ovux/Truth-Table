mod tokenizer;
mod tree;

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

    // println!("{:?}", tokens);

    // Generate AST
    let ast: Tree<i32> = Tree::new(1);
    ast.add_child(2);
    ast.add_child(3);
    let child = ast.add_child(4);
    child.add_child(5);
    let grandchild = child.add_child(6);
    ast.add_child(7);
    println!("{}", ast);

    grandchild.reparent(&ast);
    println!("{}", ast);
}
