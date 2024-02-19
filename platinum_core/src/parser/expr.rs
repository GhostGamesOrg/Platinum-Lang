use crate::lexer::token::{NumberType, Token, TokenType};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    String,
    Char,
    Integer,
    Float,
    Bool,
    Null
}

impl Type {
    fn to_string(&self) -> String {
        match self {
            Type::String => "String".to_string(),
            Type::Char => "char".to_string(),
            Type::Integer => "int".to_string(),
            Type::Float => "Float".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Null => "null".to_string()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> },
    EqualtyComparison { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Ternary { result: Box<Expression>, true_expression: Box<Expression>, false_expression: Box<Expression> },
    Unary { operator: Token, right: Box<Expression> },
    Grouping { expression: Box<Expression> },
    Variable { name: Token },
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
            Expression::EqualtyComparison { left, operator, right} => {
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
            Expression::Variable { name } => {
                format!("{}", name.lexeme.clone())
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
                                TokenType::LessLess, TokenType::GreaterGreater, TokenType::Persent,
                                TokenType::Bar, TokenType::Ampersant, TokenType::Caret
                                ]
                            ) {
                            return Ok(Type::Integer);
                        }
                        Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    (Type::Integer, Type::Float) |
                    (Type::Float, Type::Integer) |
                    (Type::Float, Type::Float) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::Plus, TokenType::Slash, TokenType::Minus,
                                TokenType::Star, TokenType::Persent
                                ]
                            ) {
                            return Ok(Type::Float);
                        }
                        Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    _ => Err("Unexpected binary operator".to_string())
                }
            }
            Expression::EqualtyComparison { left, operator, right } => {
                let left_result = (*left).check_and_get_type()?;
                let right_result = (*right).check_and_get_type()?;
                match (left_result, right_result) {
                    (Type::String, Type::String) => {
                        if match_tokens(operator.clone(), vec![TokenType::EqualEqual, TokenType::BangEqual]) {
                            return Ok(Type::Bool);
                        }
                        return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    (Type::Integer, Type::Integer) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::EqualEqual, TokenType::BangEqual, TokenType::LessEqual,
                                TokenType::Less, TokenType::GreaterEqual, TokenType::Greater, TokenType::QuestionQuestion
                                ]
                            ) {
                            return Ok(Type::Bool);
                        }
                        return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    (Type::Integer, Type::Float) |
                    (Type::Float, Type::Integer) |
                    (Type::Float, Type::Float) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::EqualEqual, TokenType::BangEqual, TokenType::LessEqual,
                                TokenType::Less, TokenType::GreaterEqual, TokenType::Greater, TokenType::QuestionQuestion,
                                TokenType::Or, TokenType::And
                                ]
                            ) {
                            return Ok(Type::Bool);
                        }
                        return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    (Type::Bool, Type::Bool) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::EqualEqual, TokenType::BangEqual,
                                TokenType::Or, TokenType::And
                                ]
                            ) {
                            return Ok(Type::Bool);
                        }
                        return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    (_, Type::Null) |
                    (Type::Null, _) => {
                        if match_tokens(
                            operator.clone(),
                            vec![
                                TokenType::EqualEqual, TokenType::BangEqual
                                ]
                            ) {
                            return Ok(Type::Bool);
                        }
                        return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                    }
                    _ => return Err("Unexpected binary operator".to_string())
                }
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
            Expression::Variable { .. } => todo!(),
            Expression::Literal { value } => {
                match value.token_type {
                    TokenType::StringT {..} => Ok(Type::String),
                    TokenType::Char {..} => Ok(Type::Char),
                    TokenType::Int {..} => Ok(Type::Integer),
                    TokenType::Float {..} => Ok(Type::Float),
                    TokenType::BoolT {..} => Ok(Type::Bool),
                    TokenType::Null => Ok(Type::Null),
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
                        let left_value = match *left.clone() {
                            Expression::Literal { value } => {
                                match value.token_type {
                                    TokenType::Int { value, num_type } => parse_i128(&value)?,
                                    _ => return Err("Unexpected AST error".to_string())
                                }
                            }
                            _ => return Ok(Expression::Binary { left: left.clone(), operator: operator.clone(), right: right.clone() })
                        };
                        let right_value = match *right.clone() {
                            Expression::Literal { value } => {
                                match value.token_type {
                                    TokenType::Int { value, num_type } => parse_i128(&value)?,
                                    _ => return Err("Unexpected AST error".to_string())
                                }
                            }
                            _ => return Ok(Expression::Binary { left: left.clone(), operator: operator.clone(), right: right.clone() })
                        };
                        let value = match operator.token_type {
                            TokenType::Plus => left_value + right_value,
                            TokenType::Minus => left_value - right_value,
                            TokenType::Star => left_value * right_value,
                            TokenType::Slash => left_value / right_value,
                            TokenType::Persent => left_value % right_value,
                            TokenType::LessLess => left_value << right_value,
                            TokenType::GreaterGreater => left_value >> right_value,
                            TokenType::Bar => left_value | right_value,
                            TokenType::Caret => left_value ^ right_value,
                            TokenType::Ampersant => left_value & right_value,
                            _ => return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                        };
                        return Ok(Expression::Literal { value: Token::new(TokenType::Int { value: value.to_string(), num_type: NumberType::UntypedInt }, value.to_string(), operator.possition) })
                    }
                    (Type::Integer, Type::Float) |
                    (Type::Float, Type::Integer) |
                    (Type::Float, Type::Float) => {
                        let left_value = match *left.clone() {
                            Expression::Literal { value } => {
                                match value.token_type {
                                    TokenType::Int { value, .. } => parse_f64(&value)?,
                                    TokenType::Float { value, .. } => parse_f64(&value)?,
                                    _ => return Err("Unexpected AST error".to_string())
                                }
                            }
                            _ => return Ok(Expression::Binary { left: left.clone(), operator: operator.clone(), right: right.clone() })
                        };
                        let right_value = match *right.clone() {
                            Expression::Literal { value } => {
                                match value.token_type {
                                    TokenType::Int { value, .. } => parse_f64(&value)?,
                                    TokenType::Float { value, .. } => parse_f64(&value)?,
                                    _ => return Err("Unexpected AST error".to_string())
                                }
                            }
                            _ => return Ok(Expression::Binary { left: left.clone(), operator: operator.clone(), right: right.clone() })
                        };
                        let value = match operator.token_type {
                            TokenType::Plus => left_value + right_value,
                            TokenType::Minus => left_value - right_value,
                            TokenType::Star => left_value * right_value,
                            TokenType::Slash => left_value / right_value,
                            TokenType::Persent => left_value % right_value,
                            _ => return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                        };
                        return Ok(Expression::Literal { value: Token::new(TokenType::Float { value: value.to_string(), num_type: NumberType::UntypedFloat }, value.to_string(), operator.possition) })
                    }
                    _ => return Err("Unexpected binary operator".to_string())
                }
            }
            Expression::EqualtyComparison { left, operator, right } => {
                let left_result = (*left).check_and_get_type()?;
                let right_result = (*right).check_and_get_type()?;
                *left = Box::from((*left).optimize_expression()?);
                *right = Box::from((*right).optimize_expression()?);
                let left_token = match *left.clone() {
                    Expression::Literal { value } => value,
                    _ => return Ok(Expression::EqualtyComparison { left: left.clone(), operator: operator.clone(), right: right.clone() })
                };
                let right_token = match *right.clone() {
                    Expression::Literal { value } => value,
                    _ => return Ok(Expression::EqualtyComparison { left: left.clone(), operator: operator.clone(), right: right.clone() })
                };
                let result = {
                    if match_token(operator.token_type.clone(), TokenType::EqualEqual) { 
                        match (left_token.token_type, right_token.token_type) {
                            (TokenType::StringT { value: value_left }, TokenType::StringT { value: value_right }) => value_left == value_right,
                            (TokenType::Char { value: value_left }, TokenType::Char { value: value_right }) => value_left == value_right,
                            (TokenType::BoolT { value: value_left }, TokenType::BoolT { value: value_right }) => value_left == value_right,
                            (TokenType::Int { value: value_left, .. }, TokenType::Int { value: value_right, .. }) |
                            (TokenType::Float { value: value_left, .. }, TokenType::Float { value: value_right, .. }) |
                            (TokenType::Int { value: value_left, .. }, TokenType::Float { value: value_right, .. }) |
                            (TokenType::Float { value: value_left, .. }, TokenType::Int { value: value_right, .. }) => parse_f64(&value_left) == parse_f64(&value_right),
                            (TokenType::Null, TokenType::Null) => true,
                            _ => false
                        }
                    } else if match_token(operator.token_type.clone(), TokenType::BangEqual) {
                        match (left_token.token_type, right_token.token_type) {
                            (TokenType::StringT { value: value_left }, TokenType::StringT { value: value_right }) => value_left != value_right,
                            (TokenType::Char { value: value_left }, TokenType::Char { value: value_right }) => value_left != value_right,
                            (TokenType::BoolT { value: value_left }, TokenType::BoolT { value: value_right }) => value_left != value_right,
                            (TokenType::Int { value: value_left, .. }, TokenType::Int { value: value_right, .. }) |
                            (TokenType::Float { value: value_left, .. }, TokenType::Float { value: value_right, .. }) |
                            (TokenType::Int { value: value_left, .. }, TokenType::Float { value: value_right, .. }) |
                            (TokenType::Float { value: value_left, .. }, TokenType::Int { value: value_right, .. }) => parse_f64(&value_left) != parse_f64(&value_right),
                            (TokenType::Null, TokenType::Null) => false,
                            _ => true
                        }
                    } else {
                        match (left_token.token_type, right_token.token_type) {
                            (TokenType::StringT { value: value_left }, TokenType::StringT { value: value_right }) => {
                                match operator.token_type.clone() {
                                    TokenType::QuestionQuestion => todo!(),
                                    _ => return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                                }
                            }
                            (TokenType::Char { value: value_left }, TokenType::Char { value: value_right }) => {
                                match operator.token_type.clone() {
                                    TokenType::LessEqual => value_left <= value_right,
                                    TokenType::Less => value_left < value_right,
                                    TokenType::GreaterEqual => value_left >= value_right,
                                    TokenType::Greater => value_left > value_right,
                                    TokenType::QuestionQuestion => todo!(),
                                    _ => return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                                }
                            }
                            (TokenType::Int { value: value_left, .. }, TokenType::Int { value: value_right, .. }) => {
                                match operator.token_type.clone() {
                                    TokenType::LessEqual => parse_i128(&value_left)? <= parse_i128(&value_right)?,
                                    TokenType::Less => parse_i128(&value_left)? < parse_i128(&value_right)?,
                                    TokenType::GreaterEqual => parse_i128(&value_left)? >= parse_i128(&value_right)?,
                                    TokenType::Greater => parse_i128(&value_left)? > parse_i128(&value_right)?,
                                    TokenType::QuestionQuestion => todo!(),
                                    _ => return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                                }
                            }
                            (TokenType::Int { value: value_left, .. }, TokenType::Float { value: value_right, .. }) |
                            (TokenType::Float { value: value_left, .. }, TokenType::Int { value: value_right, .. }) |
                            (TokenType::Float { value: value_left, .. }, TokenType::Float { value: value_right, .. }) => {
                                match operator.token_type.clone() {
                                    TokenType::LessEqual => parse_f64(&value_left) <= parse_f64(&value_right),
                                    TokenType::Less => parse_f64(&value_left) < parse_f64(&value_right),
                                    TokenType::GreaterEqual => parse_f64(&value_left) >= parse_f64(&value_right),
                                    TokenType::Greater => parse_f64(&value_left) > parse_f64(&value_right),
                                    TokenType::QuestionQuestion => todo!(),
                                    _ => return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                                }
                            }
                            (TokenType::BoolT { value: value_left }, TokenType::BoolT { value: value_right }) => {
                                match operator.token_type.clone() {
                                    TokenType::Or => value_left || value_right,
                                    TokenType::And => value_left && value_right,
                                    TokenType::QuestionQuestion => todo!(),
                                    _ => return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                                }
                            }
                            _ => return Err(format!("Can't use operator `{}` with `{}` type", operator.lexeme, left_result.to_string()))
                        }
                    }
                };
                return Ok(Expression::Literal { value: Token::new(TokenType::BoolT { value: result }, result.to_string(), operator.possition) })
            }
            Expression::Ternary { result, true_expression, false_expression } => {
                let result_type = (*result).check_and_get_type()?;
                let true_expression_result = (*true_expression).check_and_get_type()?;
                let false_expression_result = (*false_expression).check_and_get_type()?;

                if result_type != Type::Bool {
                    return Err(format!("Condition expression should have `{}` value", Type::Bool.to_string()));
                }
                
                if true_expression_result != false_expression_result {
                    return Err("Left expression and right expression should return the same type".to_string());
                }

                *result = Box::from((*result).optimize_expression()?);
                *true_expression = Box::from((*true_expression).optimize_expression()?);
                *false_expression = Box::from((*false_expression).optimize_expression()?);

                if match *result.clone() {
                    Expression::Literal { value } => {
                        match value.token_type.clone() {
                            TokenType::BoolT { value } => value,
                            _ => return Err("Unexpected AST error".to_string())
                        }
                    }
                    _ => return Ok(Expression::Ternary { result: result.clone(), true_expression: true_expression.clone(), false_expression: false_expression.clone() })
                } {
                    return Ok(*true_expression.clone());
                } else {
                    return Ok(*false_expression.clone());
                }
            }
            Expression::Unary { operator, right } => {
                let right_result = (*right).check_and_get_type()?;
                *right = Box::from((*right).optimize_expression()?);
                match right_result {
                    Type::Bool => {
                        match *right.clone() {
                            Expression::Literal { value: token } => {
                                match token.token_type.clone() {
                                    TokenType::BoolT { value } => {
                                        return Ok(
                                            Expression::Literal {
                                                value: Token::new(
                                                    TokenType::BoolT { value: !value },
                                                    (!value).to_string(),
                                                    token.possition.clone()
                                                )
                                            }
                                        );
                                    }
                                    _ => return Err("Unexpected AST error".to_string())
                                }
                            }
                            _ => return Ok(Expression::Unary { operator: operator.clone(), right: right.clone() })
                        }
                    },
                    Type::Integer => {
                        match *right.clone() {
                            Expression::Literal { value } => {
                                match value.token_type.clone() {
                                    TokenType::Int { value: num, ..} => {
                                        return Ok(
                                            Expression::Literal {
                                                value: Token::new(
                                                    TokenType::Int {
                                                        value: (-parse_i128(&num)?).to_string(),
                                                        num_type: NumberType::UntypedInt
                                                    }, 
                                                    (-parse_i128(&num)?).to_string(),
                                                    value.possition
                                                ) 
                                            }
                                        );
                                    },
                                    _ => return Err("Unexpected AST error".to_string())
                                }
                            }
                            _ => return Ok(Expression::Unary { operator: operator.clone(), right: right.clone() })
                        }
                    }
                    Type::Float =>  {
                        match *right.clone() {
                            Expression::Literal { value } => {
                                match value.token_type.clone() {
                                    TokenType::Int { value: num, ..} => {
                                        return Ok(
                                            Expression::Literal {
                                                value: Token::new(
                                                    TokenType::Int {
                                                        value: (-parse_f64(&num)?).to_string(),
                                                        num_type: NumberType::UntypedInt
                                                    }, 
                                                    (-parse_f64(&num)?).to_string(),
                                                    value.possition
                                                ) 
                                            }
                                        );
                                    },
                                    _ => return Err("Unexpected AST error".to_string())
                                }
                            }
                            _ => return Ok(Expression::Unary { operator: operator.clone(), right: right.clone() })
                        }
                    }
                    _ => return Err("Unexpected AST error".to_string())
                }
            }
            Expression::Grouping { expression } => {
                return (*expression).clone().optimize_expression();
            }
            Expression::Variable { .. } => return Ok(self.clone()),
            Expression::Literal { .. } => return Ok(self.clone()),
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

fn parse_i128(num_str: &str) -> Result<i128, String> {
    match num_str.parse::<i128>() {
        Ok(num) => {
            // Если парсинг прошел успешно, проверяем, что число не вызвало переполнение
            if num.overflowing_add(1).1 {
                Err(String::from("Memory overflow detected"))
            } else {
                Ok(num)
            }
        },
        Err(_) => Err(String::from("Failed to parse as i128")),
    }
}

fn parse_u128(num_str: &str) -> Result<u128, String> {
    match num_str.parse::<u128>() {
        Ok(num) => {
            // Если парсинг прошел успешно, проверяем, что число не вызвало переполнение
            if num.overflowing_add(1).1 {
                Err(String::from("Memory overflow detected"))
            } else {
                Ok(num)
            }
        },
        Err(_) => Err(String::from("Failed to parse as u128")),
    }
}

fn parse_f64(num_str: &str) -> Result<f64, String> {
    match num_str.parse::<f64>() {
        Ok(num) => {
            // Check if the number is finite and within a certain range
            if num.is_finite() && num < f64::MAX {
                Ok(num)
            } else {
                Err(String::from("Invalid or out-of-range floating-point number"))
            }
        },
        Err(_) => Err(String::from("Failed to parse as f64")),
    }
}
