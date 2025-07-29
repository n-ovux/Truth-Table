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

    // Generate AST
    let mut ast = Tree::new(Grammar::Root);
    ast.create_ast(&tokens);

    // Evaluate Truth Table
    for variable in &variables {
        print!("{} ", variable);
    }
    println!("{}", text.concat());
    for index in 0..2_usize.pow(variables.len().try_into().unwrap()) {
        let mut ast_valued = ast.clone();
        for (position, variable) in variables.iter().enumerate() {
            if (index >> position) & 1 == 1 {
                ast_valued.find_replace(Grammar::Value(*variable), Grammar::Value('c'));
                print!("c ");
            } else if (index >> position) & 1 == 0 {
                ast_valued.find_replace(Grammar::Value(*variable), Grammar::Value('t'));
                print!("t ");
            }
        }
        if ast_valued.evaluate(0) {
            println!("{}t", " ".repeat(text.len() / 2));
        } else {
            println!("{}c", " ".repeat(text.len() / 2));
        }
    }
}
