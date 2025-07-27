use std::cell::RefCell;
use std::rc::Rc;

use crate::tokenizer::Token;
use crate::tree::Tree;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Grammar {
    Body,
    Value(char),
    Operator(char),
    Negation,
}

pub trait AST {
    fn create_ast(tokens: &[Token]) -> Self;
}

impl AST for Tree<Grammar> {
    fn create_ast(tokens: &[Token]) -> Self {
        let root = Tree::new(Grammar::Body);

        let mut current_operator: Option<RefCell<Tree<Grammar>>> = None;
        let mut current_value: Option<Rc<Tree<Grammar>>> = None;
        let mut current_value_head: Option<Rc<Tree<Grammar>>> = None;
        let mut blacklisted_indices: Vec<usize> = Vec::new();
        for (index, token) in tokens.iter().enumerate() {
            if blacklisted_indices.contains(&index) {
                continue;
            }
            match token {
                Token::Operator(operator) => {
                    current_operator =
                        Some(RefCell::new(root.add_child(Grammar::Operator(*operator))));
                    current_value_head
                        .take()
                        .unwrap()
                        .reparent(&current_operator.as_ref().unwrap().borrow());
                    current_value = None;
                }
                Token::Value(value) => {
                    if let Some(_) = &current_value_head {
                        let new = current_value
                            .take()
                            .unwrap()
                            .add_child(Grammar::Value(*value));
                        current_value = Some(Rc::new(new));
                    } else {
                        let new = Rc::new(Tree::new(Grammar::Value(*value)));
                        current_value = Some(Rc::clone(&new));
                        current_value_head = Some(Rc::clone(&new));
                    }
                }
                Token::Negation => {
                    if let Some(_) = &current_value_head {
                        let new = current_value.as_ref().unwrap().add_child(Grammar::Negation);
                        current_value = Some(Rc::new(new));
                    } else {
                        let new = Rc::new(Tree::new(Grammar::Negation));
                        current_value = Some(Rc::clone(&new));
                        current_value_head = Some(Rc::clone(&new));
                    }
                }
                Token::OpeningBracket => {
                    let mut opens: i32 = 1;
                    let ending_index = tokens
                        .iter()
                        .enumerate()
                        .find(|(sub_index, sub_token)| {
                            if *sub_index > index && **sub_token == Token::OpeningBracket {
                                opens += 1;
                            }
                            if *sub_index > index && **sub_token == Token::ClosingBracket {
                                opens -= 1;
                            }
                            *sub_index > index && **sub_token == Token::ClosingBracket && opens == 0
                        })
                        .unwrap()
                        .0;
                    for sub_index in index..ending_index {
                        blacklisted_indices.push(sub_index);
                    }
                    if let Some(_) = &current_value_head {
                        let new = Tree::<Grammar>::create_ast(&tokens[index + 1..ending_index + 1]);
                        new.reparent(current_value.as_ref().unwrap());
                        current_value = Some(Rc::new(new));
                    } else {
                        let new = Rc::new(Tree::<Grammar>::create_ast(
                            &tokens[index + 1..ending_index + 1],
                        ));
                        current_value = Some(Rc::clone(&new));
                        current_value_head = Some(Rc::clone(&new));
                    }
                }
                Token::ClosingBracket => {}
            }
        }
        if let Some(operator) = current_operator {
            current_value_head
                .take()
                .unwrap()
                .reparent(&operator.borrow());
        } else {
            current_value_head.take().unwrap().reparent(&root);
        }
        root
    }
}
