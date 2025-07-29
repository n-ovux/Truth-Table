use crate::lexer::Token;
use crate::tree::Tree;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Grammar {
    Root,
    Value(char),
    Operator(char),
    Negation,
}

pub trait AST {
    fn create_ast(&mut self, tokens: &[Token]) -> usize;
    fn evaluate(&self, index: usize) -> bool;
}

impl AST for Tree<Grammar> {
    fn create_ast(&mut self, tokens: &[Token]) -> usize {
        let mut current_operator: Option<usize> = None;
        let mut current_value: Option<usize> = None;
        let mut current_value_head: Option<usize> = None;
        let mut blacklisted_indices: Vec<usize> = Vec::new();
        for (index, token) in tokens.iter().enumerate() {
            if blacklisted_indices.contains(&index) {
                continue;
            }
            match token {
                Token::Operator(operator) => {
                    current_operator = Some(self.add_child(0, Grammar::Operator(*operator)));
                    self.reparent(current_value_head.unwrap(), current_operator.unwrap());
                    current_value = None;
                    current_value_head = None;
                }
                Token::Value(value) => {
                    if let Some(past_index) = current_value {
                        current_value = Some(self.add_child(past_index, Grammar::Value(*value)));
                    } else {
                        current_value_head = Some(self.add_child(0, Grammar::Value(*value)));
                        current_value = Some(*current_value_head.as_ref().unwrap());
                    }
                }
                Token::Negation => {
                    if let Some(past_index) = current_value {
                        current_value = Some(self.add_child(past_index, Grammar::Negation));
                    } else {
                        current_value_head = Some(self.add_child(0, Grammar::Negation));
                        current_value = Some(*current_value_head.as_ref().unwrap());
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
                    let sub_index = self.create_ast(&tokens[index + 1..ending_index + 1]);
                    if let Some(past_index) = current_value {
                        self.reparent(sub_index, past_index);
                        current_value = Some(sub_index);
                    } else {
                        self.reparent(sub_index, 0);
                        current_value_head = Some(sub_index);
                        current_value = Some(*current_value_head.as_ref().unwrap());
                    }
                }
                Token::ClosingBracket => {}
            }
        }
        if let Some(operator) = current_operator {
            self.reparent(current_value_head.unwrap(), operator);
            operator
        } else {
            self.reparent(current_value_head.unwrap(), 0);
            0
        }
    }

    fn evaluate(&self, index: usize) -> bool {
        if let Grammar::Value(value) = self.get_vertices()[index] {
            if value == 't' {
                return true;
            } else {
                return false;
            }
        }

        let mut children: Vec<usize> = Vec::new();
        for edge in self.get_edges() {
            if edge.0 == index {
                children.push(edge.1);
            }
        }

        let mut values: Vec<bool> = Vec::new();
        for child in &children {
            values.push(self.evaluate(*child));
        }

        if values.len() != 1 && values.len() != 2 {
            println!("{:?}", values);
            panic!("incorrect numers of values");
        }

        match self.get_vertices()[index] {
            Grammar::Root => values[0],
            Grammar::Negation => !values[0],
            Grammar::Operator(value) if value == '.' => values[0] && values[1],
            Grammar::Operator(value) if value == '+' => values[0] || values[1],
            _ => panic!("impossible"),
        }
    }
}
