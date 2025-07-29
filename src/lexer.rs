use std::fmt::{self, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Value(char),
    OpeningBracket,
    ClosingBracket,
    Operator(char),
    Negation,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn lexer(text: &Vec<String>) -> Result<(Vec<Token>, Vec<char>), LexerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut variables: Vec<char> = Vec::new();
    for item in text {
        if item.len() != 1 {
            return Err(LexerError::SymbolTooLong(item.to_string()));
        }

        let symbol = item.chars().next().unwrap();

        match symbol {
            '[' => {
                tokens.push(Token::OpeningBracket);
            }
            ']' => {
                tokens.push(Token::ClosingBracket);
            }
            '+' | '.' => {
                tokens.push(Token::Operator(symbol));
            }
            '-' => {
                tokens.push(Token::Negation);
            }
            't' | 'c' => tokens.push(Token::Value(symbol)),
            'f' => tokens.push(Token::Value('c')),
            _ => {
                variables.push(symbol);
                tokens.push(Token::Value(symbol));
            }
        }
    }
    if variables.is_empty() {
        return Err(LexerError::NoVariables);
    }
    verify(&tokens)?;

    loop {
        let position: Option<usize> = tokens.windows(3).position(|window| {
            window[0] == Token::OpeningBracket && window[2] == Token::ClosingBracket
        });
        if let Some(position) = position {
            tokens.remove(position);
            tokens.remove(position + 1);
        } else {
            break;
        }
    }

    verify(&tokens)?;

    Ok((tokens, variables))
}

pub fn verify(tokens: &Vec<Token>) -> Result<(), LexerError> {
    if tokens.is_empty() {
        return Err(LexerError::TokensHasNoElements);
    }

    let mut tokens_iter = tokens.iter();
    let mut last_token = tokens_iter.next().unwrap();
    let mut values_in_scope: Vec<i8> = if let Token::Value(_) = *last_token {
        vec![1]
    } else {
        vec![0]
    };
    let mut brackets: u8 = if *last_token == Token::OpeningBracket {
        1
    } else {
        0
    };
    for token in tokens_iter {
        match token {
            Token::Value(_) => {
                if let Token::Value(_) = *last_token {
                    return Err(LexerError::NoOperatorBetweenValues);
                }
                *values_in_scope.last_mut().unwrap() += 1;
                if *values_in_scope.last().unwrap() > 2 {
                    return Err(LexerError::AmbiguousExpression);
                }
            }
            Token::OpeningBracket => {
                if let Token::Value(_) = *last_token {
                    return Err(LexerError::ValueBeforeOpeningBracket);
                }
                brackets += 1;
                values_in_scope.push(0);
            }
            Token::ClosingBracket => {
                if let Token::Operator(_) = *last_token {
                    return Err(LexerError::OperatorBeforeClosingBracket);
                }
                if brackets == 0 {
                    return Err(LexerError::NoOpeningBracketToMatchClosing);
                }
                brackets -= 1;
                if values_in_scope.len() == 1 {
                    values_in_scope[0] = 1;
                } else {
                    values_in_scope.pop();
                    *values_in_scope.last_mut().unwrap() += 1;
                }
            }
            Token::Operator(_) => {
                if let Token::Operator(_) = *last_token {
                    return Err(LexerError::NoValueBeforeOperator);
                }
                if let Token::Negation = *last_token {
                    return Err(LexerError::NoValueBeforeOperator);
                }
            }
            Token::Negation => {
                if let Token::Operator(_) = *last_token {
                } else {
                    if let Token::OpeningBracket = *last_token {
                    } else {
                        let Token::Negation = *last_token else {
                            return Err(LexerError::NoOperatorBeforeNegation);
                        };
                    }
                };
            }
        }
        last_token = token;
    }
    if brackets != 0 {
        return Err(LexerError::MissingOpeningOrClosingBracket);
    }
    match tokens.last().unwrap() {
        Token::Operator(_) | Token::Negation | Token::OpeningBracket => {
            return Err(LexerError::DoesNotEnd)
        }
        _ => {}
    }
    Ok(())
}

#[derive(Debug)]
pub enum LexerError {
    SymbolTooLong(String),
    TokensHasNoElements,
    NoOperatorBetweenValues,
    AmbiguousExpression,
    ValueBeforeOpeningBracket,
    OperatorBeforeClosingBracket,
    NoOpeningBracketToMatchClosing,
    NoValueBeforeOperator,
    MissingOpeningOrClosingBracket,
    NoOperatorBeforeNegation,
    DoesNotEnd,
    NoVariables,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::SymbolTooLong(symbol) => write!(f, "Symbol too long: {}", symbol),
            _ => write!(f, "{:?}", self),
        }
    }
}
