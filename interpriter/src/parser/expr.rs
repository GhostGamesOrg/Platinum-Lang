use crate::lexer::token::{Token, TokenType};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    String,
    Char,
    Integer,
    Float,
    Bool,

}

impl Type {
    fn to_string(&self) -> String {
        match self {
            Type::String => "String".to_string(),
            Type::Char => "char".to_string(),
            Type::Integer => "int".to_string(),
            Type::Float => "Float".to_string(),
            Type::Bool => "bool".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Equalty { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Ternary { result: Box<Expression>, true_expression: Box<Expression>, false_expression: Box<Expression> },
    Unary { operator: Token, right: Box<Expression> },
    Grouping { expression: Box<Expression> },
    Literal { value: Token }
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Binary { left, operator, right} => {
                format!(
                    "({} {} {})",
                    operator.lexeme.clone(),
                    left.to_string(),
                    right.to_string()
                )
            }
            Expression::Equalty { left, operator, right} => {
                format!(
                    "(equalty {} {} {})",
                    operator.lexeme.clone(),
                    left.to_string(),
                    right.to_string()
                )
            }
            Expression::Ternary { result, true_expression, false_expression } => {
                format!(
                    "(ternary {} ? {} : {})",
                    (*result).to_string(),
                    (*true_expression).to_string(),
                    (*false_expression).to_string()
                )
            }
            Expression::Grouping { expression } => {
                format!("(group {})", (*expression).to_string())
            }
            Expression::Literal { value } => {
                format!("{}", value.lexeme.clone())
            }
            Expression::Unary { operator, right } => {
                format!("({} {})", operator.lexeme.clone(), (*right).to_string())
            }
        }
    }

    pub fn check_and_get_type(&self) -> Result<Type, String> {
        match self {
            Expression::Binary { left, operator, right } => {
                let left_result = (*left).check_and_get_type()?;
                let right_result = (*right).check_and_get_type()?;
                match (left_result, right_result) {
                    (Type::String, _) => {
                        if match_token(operator.token_type.clone(), TokenType::Plus) {
                            return Ok(Type::String);
                        }
                        Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    (Type::Integer, Type::Integer) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::Plus, TokenType::Minus,
                                TokenType::Star, TokenType::Slash,
                                TokenType::LessLess, TokenType::GreaterGreater,
                                TokenType::GreaterGreaterGreater, TokenType::LessEqual,
                                TokenType::Less, TokenType::GreaterEqual, TokenType::Greater, TokenType::QuestionQuestion,
                                TokenType::Or, TokenType::And, TokenType::Bar, TokenType::Caret, TokenType::Ampersant
                                ]
                            ) {
                            return Ok(Type::Integer);
                        }
                        Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    (Type::Float, Type::Integer) |
                    (Type::Float, Type::Float) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::Plus, TokenType::Minus,
                                TokenType::Star, TokenType::Slash,
                                TokenType::LessLess, TokenType::GreaterGreater,
                                TokenType::GreaterGreaterGreater, TokenType::LessEqual,
                                TokenType::Less, TokenType::GreaterEqual, TokenType::Greater, TokenType::QuestionQuestion,
                                TokenType::Or, TokenType::And, TokenType::Bar, TokenType::Caret, TokenType::Ampersant
                                ]
                            ) {
                            return Ok(Type::Float);
                        }
                        Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    _ => Err("Unexpected binary operator".to_string())
                }
            }
            Expression::Equalty { left, operator, right } => {
                Ok(Type::Bool)
            }
            Expression::Ternary { result, true_expression, false_expression } => {
                let res = (*result).check_and_get_type()?;
                if res != Type::Bool {
                    return Err("Condition should be bool value".to_string());
                }

                let true_result = (*true_expression).check_and_get_type()?;
                let false_result = (*false_expression).check_and_get_type()?;
                if true_result != false_result {
                    Err("Expression #1 and expression #2 should return the same type!".to_string())
                } else {
                    Ok(true_result)
                }

            }
            Expression::Unary { operator, right } => {
                let result = (*right).check_and_get_type()?;
                match operator.token_type {
                    TokenType::Bang => {
                        if result == Type::Bool {
                            return Ok(Type::Bool);
                        }
                        return Err(format!("You can't use `{}` with unary operator `!`", result.to_string()));
                    }
                    TokenType::Minus => {
                        if result == Type::Integer {
                            return Ok(Type::Integer);
                        } else if result == Type::Float {
                            return Ok(Type::Float);
                        }
                        return Err(format!("You can't use `{}` with unary operator `!`", result.to_string()));
                    }
                    _ => Err("Unexpected binary operator".to_string())
                }
            }
            Expression::Grouping { expression } => (*expression).check_and_get_type(),
            Expression::Literal { value } => {
                match value.token_type {
                    TokenType::StringT {..} => Ok(Type::String),
                    TokenType::Char {..} => Ok(Type::Char),
                    TokenType::Int {..} => Ok(Type::Integer),
                    TokenType::Float {..} => Ok(Type::Float),
                    TokenType::BoolT {..} => Ok(Type::Bool),
                    _ => Err("Parser error".to_string())
                }
            }
        }
    }

    pub fn optimize_expression(&mut self) -> Result<Expression, String> {
        let _ = self.check_and_get_type()?;
        match self {
            Expression::Binary { left, operator, right } => {
                let left_result = (*left).check_and_get_type()?;
                let right_result = (*right).check_and_get_type()?;
                *left = Box::from((*left).optimize_expression()?);
                *right = Box::from((*right).optimize_expression()?);
                match (left_result, right_result) {
                    (Type::String, _) => {
                        if match_token(operator.token_type.clone(), TokenType::Plus) {
                            let mut left_value = match *left.clone() {
                                Expression::Literal { value } => {
                                    match value.token_type {
                                        TokenType::StringT { value } => value,
                                        TokenType::BoolT { value } => value.to_string(),
                                        TokenType::Int { value,..} => value.to_string(),
                                        TokenType::Float { value,..} => value.to_string(),
                                        TokenType::Char { value} => value.to_string(),
                                        _ => return Err("Unexpected AST error".to_string())
                                    }
                                }
                                _ => {
                                    return Ok(Expression::Binary { left: left.clone(), operator: operator.clone(), right: right.clone() });
                                }
                            };
                            let right_value = match *right.clone() {
                                Expression::Literal { value } => {
                                    match value.token_type {
                                        TokenType::StringT { value } => value,
                                        TokenType::BoolT { value } => value.to_string(),
                                        TokenType::Int { value,..} => value.to_string(),
                                        TokenType::Float { value,..} => value.to_string(),
                                        TokenType::Char { value} => value.to_string(),
                                        _ => return Err("Unexpected AST error".to_string())
                                    }
                                }
                                _ => {
                                    return Ok(Expression::Binary { left: left.clone(), operator: operator.clone(), right: right.clone() });
                                }
                            };
                            left_value.push_str(&right_value);
                            return Ok(Expression::Literal { value: Token::new(TokenType::StringT { value: left_value.clone() }, left_value.clone(), operator.possition)});
                        }
                        return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()));
                    }
                    (Type::Integer, Type::Integer) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::Plus, TokenType::Minus,
                                TokenType::Star, TokenType::Slash,
                                TokenType::LessLess, TokenType::GreaterGreater,
                                TokenType::GreaterGreaterGreater, TokenType::LessEqual,
                                TokenType::Less, TokenType::GreaterEqual, TokenType::Greater, TokenType::QuestionQuestion,
                                TokenType::Or, TokenType::And, TokenType::Bar, TokenType::Caret, TokenType::Ampersant
                                ]
                            ) {
                            todo!()
                        }
                        return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    (Type::Float, Type::Integer) |
                    (Type::Float, Type::Float) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::Plus, TokenType::Minus,
                                TokenType::Star, TokenType::Slash,
                                TokenType::LessLess, TokenType::GreaterGreater,
                                TokenType::GreaterGreaterGreater, TokenType::LessEqual,
                                TokenType::Less, TokenType::GreaterEqual, TokenType::Greater, TokenType::QuestionQuestion,
                                TokenType::Or, TokenType::And, TokenType::Bar, TokenType::Caret, TokenType::Ampersant
                                ]
                            ) {
                            todo!()
                        }
                        return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()));
                    }
                    _ => return Err("Unexpected binary operator".to_string())
                }
            }
            Expression::Equalty { left, operator, right } => {
                (*left).optimize_expression()?;
                (*right).optimize_expression()?;
            }
            Expression::Ternary { result, true_expression, false_expression } => {
                (*result).optimize_expression()?;
                (*true_expression).optimize_expression()?;
                (*false_expression).optimize_expression()?;
            }
            Expression::Unary { operator, right } => {
                (*right).optimize_expression()?;
            }
            Expression::Grouping { expression } => {
                return (*expression).optimize_expression();
            }
            Expression::Literal { value } => return Ok(Expression::Literal { value: value.clone() }),
        }
        todo!()
    }
}

fn match_token(token: TokenType, token_type: TokenType) -> bool {
    if token.eq_token(token_type) {
        true
    } else {
        false
    }
}

fn match_tokens(token: Token, token_types: Vec<TokenType>) -> bool {
    for token_type in token_types {
        if match_token(token.token_type.clone(), token_type) {
            return true;
        }
    }
    false
}
