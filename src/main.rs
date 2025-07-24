mod ast;

use crate::ast::*;

fn main() {
    let mut arguments: std::env::Args = std::env::args();
    if arguments.len() == 1 {
        example_exit("Not Enough Arguments", 1);
    }

    // Error Checker and Tokenizer
    let mut tokens: Vec<(Token, char)> = Vec::new();
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
                }
            }
        }
    }
    if brackets != 0 {
        example_exit("Unequal amount of brackets!", 9);
    }

    println!("{:?}", tokens);

    let mut ast: Node = Node::new(Grammar::ROOT);
    let mut current_node: &mut Node = &mut ast;
    for token in tokens {
        match token.0 {
            Token::VARIABLE => current_node.add_child(Grammar::VALUE),
            Token::CONSTANT => current_node.add_child(Grammar::VALUE),
            Token::OPERATOR => current_node.add_child(Grammar::OPERATOR),
            Token::NEGATION => current_node.add_child(Grammar::OPERATOR),
            Token::BRACKET => {
                if token.1 == '[' {
                    last_node = current_node;
                    current_node = current_node.children.last_mut().unwrap();
                } else {
                    current_node = last_node;
                }
            }
        }
    }
    println!("{ast}");
}

fn example_exit(error_text: &str, exit_code: i32) {
    println!("{error_text}");
    println!("Example Input: [ p + q ] . r");
    std::process::exit(exit_code);
}
