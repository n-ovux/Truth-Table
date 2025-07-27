use token_error::ErrorType;

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

impl Token {
    pub fn tokenize(text: &Vec<String>) -> Result<Vec<Token>, token_error::ErrorType> {
        let mut tokens: Vec<Token> = Vec::new();
        for item in text {
            if item.len() != 1 {
                return Err(ErrorType::SymbolTooLong(item.to_string()));
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
                't' | 'f' | 'c' => {
                    if symbol == 'f' {
                        tokens.push(Token::Value('c'));
                    } else {
                        tokens.push(Token::Value(symbol));
                    }
                }
                _ => {
                    tokens.push(Token::Value(symbol));
                }
            }
        }
        return Ok(tokens);
    }

    pub fn verify(tokens: &Vec<Token>) -> Result<(), token_error::ErrorType> {
        if tokens.is_empty() {
            return Err(ErrorType::TokensHasNoElements);
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
                        return Err(ErrorType::NoOperatorBetweenValues);
                    }
                    *values_in_scope.last_mut().unwrap() += 1;
                    if *values_in_scope.last().unwrap() > 2 {
                        return Err(ErrorType::AmbiguousExpression);
                    }
                }
                Token::OpeningBracket => {
                    if let Token::Value(_) = *last_token {
                        return Err(ErrorType::ValueBeforeOpeningBracket);
                    }
                    brackets += 1;
                    values_in_scope.push(0);
                }
                Token::ClosingBracket => {
                    if let Token::Operator(_) = *last_token {
                        return Err(ErrorType::OperatorBeforeClosingBracket);
                    }
                    // if let Token::ClosingBracket = *last_token {
                    //     return Err(ErrorType::NegationBeforeClosingBracket);
                    // }
                    if brackets == 0 {
                        return Err(ErrorType::NoOpeningBracketToMatchClosing);
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
                        return Err(ErrorType::NoValueBeforeOperator);
                    }
                    if let Token::Negation = *last_token {
                        return Err(ErrorType::NoValueBeforeOperator);
                    }
                }
                Token::Negation => {
                    if let Token::Operator(_) = *last_token {
                    } else {
                        let Token::Negation = *last_token else {
                            return Err(ErrorType::NoOperatorBeforeNegation);
                        };
                    };
                }
            }
            last_token = token;
        }
        if brackets != 0 {
            return Err(ErrorType::MissingOpeningOrClosingBracket);
        }
        match tokens.last().unwrap() {
            Token::Operator(_) | Token::Negation | Token::OpeningBracket => {
                return Err(ErrorType::DoesNotEnd)
            }
            _ => {}
        }

        Ok(())
    }
}

pub mod token_error {
    use std::fmt::{self, Formatter};

    #[derive(Debug)]
    pub enum ErrorType {
        SymbolTooLong(String),
        TokensHasNoElements,
        NoOperatorBetweenValues,
        AmbiguousExpression,
        ValueBeforeOpeningBracket,
        OperatorBeforeClosingBracket,
        NegationBeforeClosingBracket,
        NoOpeningBracketToMatchClosing,
        NoValueBeforeOperator,
        MissingOpeningOrClosingBracket,
        NoOperatorBeforeNegation,
        DoesNotEnd,
    }

    impl std::fmt::Display for ErrorType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                ErrorType::SymbolTooLong(symbol) => write!(f, "Symbol too long: {}", symbol),
                _ => write!(f, "{:?}", self),
            }
        }
    }
}
