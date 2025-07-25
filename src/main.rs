mod ast;

use crate::ast::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut arguments: std::env::Args = std::env::args();
    if arguments.len() == 1 {
        example_exit("Not Enough Arguments", 1);
    }

    // Error Checker and Tokenizer
    let mut tokens: Vec<(Token, char)> = Vec::new();
    let mut variables: Vec<char> = Vec::new();
    let mut brackets: u8 = 0;
    arguments.next();
    for argument in arguments {
        if argument.chars().count() != 1 {
            example_exit("Symbol {symbol} too long!", 2);
        }

        let symbol = argument.chars().next().unwrap();
        match symbol {
            '[' | ']' => {
                if symbol == '[' {
                    brackets += 1;
                    if tokens.last().is_some()
                        && (tokens.last().unwrap().0 == Token::CONSTANT
                            || tokens.last().unwrap().0 == Token::VARIABLE
                            || tokens.last().unwrap().1 == ']')
                    {
                        example_exit("No operators between value and bracket", 3);
                    }
                } else {
                    if tokens.last().is_some() && (tokens.last().unwrap().0 == Token::NEGATION) {
                        example_exit("Negation before closing bracket", 4);
                    }
                    if brackets == 0 {
                        example_exit("Closed bracket before opening bracket!", 5);
                    }
                    brackets -= 1;
                }

                tokens.push((Token::BRACKET, symbol));
            }
            '+' | '.' => {
                if tokens.last().is_some() && (tokens.last().unwrap().0 == Token::OPERATOR) {
                    example_exit("No value between operators", 6);
                }

                tokens.push((Token::OPERATOR, symbol));
            }
            '-' => {
                if tokens.last().is_some() && (tokens.last().unwrap().0 != Token::OPERATOR) {
                    example_exit("No operator before negation", 7);
                }

                tokens.push((Token::NEGATION, symbol));
            }
            _ => {
                if tokens.last().is_some()
                    && (tokens.last().unwrap().0 == Token::VARIABLE
                        || tokens.last().unwrap().0 == Token::CONSTANT)
                {
                    example_exit("No operator between values", 8);
                }

                if symbol == 't' || symbol == 'c' || symbol == 'c' {
                    tokens.push((Token::CONSTANT, symbol));
                } else {
                    tokens.push((Token::VARIABLE, symbol));
                    if !variables.contains(&symbol) {
                        variables.push(symbol);
                    }
                }
            }
        }
    }
    if brackets != 0 {
        example_exit("Unequal amount of brackets!", 9);
    }

    if variables.is_empty() {
        example_exit("No variables found in expression", 10);
    }

    println!("{:?}", tokens);
    println!("{:?}", variables);

    let ast = Rc::new(RefCell::new(Node::new(Grammar::ROOT)));
    let mut current_node = ast.clone();
    let mut last_node = ast.clone();
    for token in tokens {
        match token.0 {
            Token::VARIABLE => {
                last_node = Node::add_child(&current_node, Grammar::VARIABLE(token.1))
            }
            Token::CONSTANT => last_node = Node::add_child(&current_node, Grammar::VALUE(token.1)),
            Token::OPERATOR => {
                last_node = Node::add_child(&current_node, Grammar::OPERATOR(token.1))
            }
            Token::NEGATION => last_node = Node::add_child(&current_node, Grammar::NEGATION),
            Token::BRACKET => {
                if token.1 == '[' {
                    current_node = last_node.clone();
                } else {
                    let node = current_node.clone();
                    current_node = node.borrow().get_parent().unwrap().clone();
                }
            }
        }
    }
    println!("{}", ast.borrow());

    let mut truth_table: Vec<bool> = Vec::new();
    truth_table.reserve(2_usize.pow(variables.len().try_into().unwrap()));

    replace_all(&ast, Grammar::VARIABLE('p'), Grammar::VALUE('t'));
    println!("{}", ast.borrow());
}

fn replace_all(tree: &Rc<RefCell<Node>>, target: Grammar, value: Grammar) {
    let mut node = tree.borrow_mut();
    node.replace_if_match(target, value);
    for child in node.get_children() {
        replace_all(child, target, value);
    }
}

fn example_exit(error_text: &str, exit_code: i32) {
    println!("{error_text}");
    println!("Example Input: [ p + q ] . r");
    std::process::exit(exit_code);
}
