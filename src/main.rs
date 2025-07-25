mod ast;

use crate::ast::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut arguments: std::env::Args = std::env::args();
    if arguments.len() == 1 {
        example_exit("Not Enough Arguments");
    }

    // Tokenizer
    let mut tokens: Vec<(Token, char)> = Vec::new();
    let mut variables: Vec<char> = Vec::new();
    arguments.next();
    for argument in arguments {
        if argument.chars().count() != 1 {
            example_exit("Invalid Symbol!");
        }

        let symbol = argument.chars().next().unwrap();
        match symbol {
            '[' | ']' => {
                tokens.push((Token::BRACKET, symbol));
            }
            '+' | '.' => {
                tokens.push((Token::OPERATOR, symbol));
            }
            '-' => {
                tokens.push((Token::NEGATION, symbol));
            }
            _ => {
                if symbol == 't' {
                    tokens.push((Token::VALUE, symbol));
                } else if symbol == 'f' || symbol == 'c' {
                    tokens.push((Token::VALUE, 'c'));
                } else {
                    tokens.push((Token::VALUE, symbol));
                    if !variables.contains(&symbol) {
                        variables.push(symbol);
                    }
                }
            }
        }
    }
    if variables.is_empty() {
        example_exit("No variables found in expression");
    }

    println!("{:?}", tokens);
    println!("{:?}", variables);

    // Error Checker
    let mut tokens_iter = tokens.iter();
    let mut last_token = tokens_iter.next().unwrap();
    let mut values_in_scope: Vec<i8> = if last_token.0 == Token::VALUE {
        vec![1]
    } else {
        vec![0]
    };
    let mut brackets: u8 = if last_token.1 == '[' { 1 } else { 0 };
    for token in tokens_iter {
        match token.0 {
            Token::VALUE => {
                if last_token.0 == Token::VALUE {
                    example_exit("No operator between values!");
                }
                *values_in_scope.last_mut().unwrap() += 1;
                if *values_in_scope.last().unwrap() > 2 {
                    example_exit("Ambiguous expression!");
                }
            }
            Token::BRACKET => {
                if token.1 == '[' {
                    if last_token.0 == Token::VALUE {
                        example_exit("Value before opening bracket!");
                    }
                    brackets += 1;
                    values_in_scope.push(0);
                } else {
                    if last_token.0 == Token::OPERATOR {
                        example_exit("Operator or negation before closing bracket!");
                    }
                    if brackets == 0 {
                        example_exit("No opening bracket to match closing bracket!");
                    }
                    brackets -= 1;
                    if values_in_scope.len() == 1 {
                        values_in_scope[0] = 1;
                    } else {
                        values_in_scope.pop();
                        *values_in_scope.last_mut().unwrap() += 1;
                    }
                }
            }
            Token::OPERATOR => {
                if last_token.0 == Token::OPERATOR || last_token.0 == Token::NEGATION {
                    example_exit("No value before operator!");
                }
            }
            Token::NEGATION => {
                if last_token.0 != Token::OPERATOR {
                    example_exit("No operator before negation!");
                }
            }
        }
        last_token = token;
    }

    if brackets != 0 {
        example_exit("Unequal amount of brackets!");
    }

    let ast = Rc::new(RefCell::new(Node::new(Grammar::ROOT)));
    let mut current_node = ast.clone();
    let mut last_node = ast.clone();
    for token in tokens {
        match token.0 {
            Token::VALUE => {
                if token.1 == 't' || token.1 == 'c' {
                    last_node = Node::add_child(&current_node, Grammar::VALUE(token.1));
                } else {
                    last_node = Node::add_child(&current_node, Grammar::VARIABLE(token.1));
                }
            }
            Token::OPERATOR => {
                last_node = Node::add_child(&current_node, Grammar::OPERATOR(token.1))
            }
            Token::NEGATION => last_node = Node::add_child(&current_node, Grammar::NEGATION),
            Token::BRACKET => {
                if token.1 == '[' {
                    current_node = last_node.clone();
                } else {
                    let node = current_node.clone().borrow().get_parent();
                    if let Some(parent) = node {
                        current_node = parent.clone();
                    }
                }
            }
        }
    }
    println!("{}", ast.borrow());

    let mut truth_table: Vec<bool> = Vec::new();
    truth_table.reserve(2_usize.pow(variables.len().try_into().unwrap()));

    // ast.borrow_mut()
    //     .find_replace(Grammar::VARIABLE('p'), Grammar::VALUE('t'));
    // println!("{}", ast.borrow());
}

fn example_exit(error_text: &str) {
    println!("{error_text}");
    println!("Example Input: [ p + q ] . r");
    std::process::exit(1);
}
