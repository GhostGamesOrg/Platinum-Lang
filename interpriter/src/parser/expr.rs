use crate::lexer::token::{Token, TokenType::*, LiteralValue, LiteralValue::*};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: Token, right: Box<Expr> }
}


macro_rules! cant_use_expression_for_types {
    ($lit:expr, $expr_type:expr, $( $literal_type:ident ),*) => {
        match $lit {
            $(
                LiteralValue::$literal_type(_) => panic!("Can't use {} expression for `{}` type.", $expr_type, $lit.get_type()),

            )*
            _ => {}
        }
    };
}

macro_rules! gen_binary_expressions {
    ($left:expr, $right:expr, $operator:ident, $( $lit_type1:ident | $lit_type2:ident => $lit_type3:ident, $out_type:ident ), *) => {
        match ($left, $right, $operator.token_type) {
            $(
                ($lit_type1(value1), $lit_type2(value2), Plus) => $lit_type3((value1 as $out_type) + (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), Minus) => $lit_type3((value1 as $out_type) - (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), Star) => $lit_type3((value1 as $out_type) * (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), Slash) => $lit_type3((value1 as $out_type) / (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), EqualEqual) => Bool((value1 as $out_type) == (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), BangEqual) => Bool((value1 as $out_type) != (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), Greater) => Bool((value1 as $out_type) > (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), GreaterEqual) => Bool((value1 as $out_type) >= (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), Less) => Bool((value1 as $out_type) < (value2 as $out_type)),
                ($lit_type1(value1), $lit_type2(value2), LessEqual) => Bool((value1 as $out_type) <= (value2 as $out_type)),
            )*
            $(
                ($lit_type1(_), _, token_type) => {
                    if vec![Plus, Minus, Star, Slash, EqualEqual, BangEqual, Greater, GreaterEqual, Less, LessEqual].contains(&token_type) {
                        panic!("Operator `{}` isn't implemented for {} and {} types", $operator.lexeme, $left.get_type(), $right.get_type());
                    }
                    panic!("Operator `{}` isn't implemented for {} type", $operator.lexeme, $left.get_type());
                }
            )*
            _ => {
                cant_use_expression_for_types!(
                    $left, "binary",
                    CharValue,
                    IdentifierValue
                );
                cant_use_expression_for_types!(
                    $right, "binary",
                    CharValue,
                    IdentifierValue
                );
                NullLiteral(false)
            }
        }
    };
}

// (I8Value(value1), I8Value(value2), Plus) => expr = I8Value(value1 + value2),
// (I8Value(value1), I8Value(value2), Minus) => expr = I8Value(value1 - value2),
// (I8Value(value1), I8Value(value2), Star) => expr = I8Value(value1 * value2),
// (I8Value(value1), I8Value(value2), Slash) => expr = I8Value(value1 / value2),

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, operator, right} => {
                format!(
                    "({} {} {})",
                    left.to_string(),
                    operator.lexeme.clone(),
                    right.to_string()
                )
            }
            Expr::Grouping { expression } => {
                format!("(group {})", (*expression).to_string())
            }
            Expr::Literal { value } => {
                format!("({})", value.to_string())
            }
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme.clone(), (*right).to_string())
            }
        }
    }

    pub fn evaluate(&self) -> LiteralValue {
        match self {
            Expr::Literal { value } => {
                return value.clone();
            }
            Expr::Grouping { expression } => {
                return (*expression).evaluate();
            }
            Expr::Unary { operator, right } => {
                let mut lit = (*right).evaluate();
                match operator.token_type {
                    Minus => {
                        match lit {
                            I8Value(value) => lit = I8Value(-value),
                            I16Value(value) => lit = I16Value(-value),
                            I32Value(value) => lit = I32Value(-value),
                            I64Value(value) => lit = I64Value(-value),
                            I128Value(value) => lit = I128Value(-value),
                            ISizeValue(value) => lit = ISizeValue(-value),

                            UndefinedIntValue(value) => lit = UndefinedIntValue(-value),
                            UndefinedFloatValue(value) => lit = UndefinedFloatValue(-value),

                            F32Value(value) => lit = F32Value(-value),
                            F64Value(value) => lit = F64Value(-value),
                            _ => {
                                cant_use_expression_for_types!(
                                    lit, "unary minus",
                                    U8Value,
                                    U16Value,
                                    U32Value,
                                    U64Value,
                                    U128Value,
                                    USizeValue,
                                    Bool,
                                    StringValue,
                                    CharValue,
                                    NullLiteral,
                                    IdentifierValue
                                )
                            }
                        }
                    }
                    Bang => {
                        match lit {

                            Bool(value) => lit = Bool(!value),
                            _ => {
                                cant_use_expression_for_types!(
                                    lit, "unary bang",
                                    I8Value,
                                    I16Value,
                                    I32Value,
                                    I64Value,
                                    I128Value,
                                    ISizeValue,

                                    U8Value,
                                    U16Value,
                                    U32Value,
                                    U64Value,
                                    U128Value,
                                    USizeValue,

                                    F32Value,
                                    F64Value,

                                    UndefinedIntValue,
                                    UndefinedFloatValue,
                                    
                                    StringValue,
                                    CharValue,
                                    NullLiteral,
                                    IdentifierValue
                                )
                            }
                        }
                    }
                    _ => ()
                }
                return lit;
            }
            Expr::Binary { left, operator, right } => {
                let expr;
                let left = (*left).evaluate();
                let right = (*right).evaluate();
                
                match (left.clone(), right.clone(), operator.token_type) {
                    (StringValue(value1), _, Plus) => {
                        let mut result = value1;
                        match right {
                            Bool(value) => result.push_str(&value.to_string()),
                            I8Value(value) => result.push_str(&value.to_string()),
                            I16Value(value) => result.push_str(&value.to_string()),
                            I32Value(value) => result.push_str(&value.to_string()),
                            I64Value(value) => result.push_str(&value.to_string()),
                            I128Value(value) => result.push_str(&value.to_string()),
                            ISizeValue(value) => result.push_str(&value.to_string()),
                            U8Value(value) => result.push_str(&value.to_string()),
                            U16Value(value) => result.push_str(&value.to_string()),
                            U32Value(value) => result.push_str(&value.to_string()),
                            U64Value(value) => result.push_str(&value.to_string()),
                            U128Value(value) => result.push_str(&value.to_string()),
                            USizeValue(value) => result.push_str(&value.to_string()),
                            F32Value(value) => result.push_str(&value.to_string()),
                            F64Value(value) => result.push_str(&value.to_string()),
                            UndefinedIntValue(value) => result.push_str(&value.to_string()),
                            UndefinedFloatValue(value) => result.push_str(&value.to_string()),
                            StringValue(value) => result.push_str(value.as_str()),
                            CharValue(value) => result.push(value),
                            _ => {
                                cant_use_expression_for_types!(
                                    right, "binary plus",
                                    NullLiteral,
                                    IdentifierValue
                                )
                            }
                        }
                        expr = StringValue(result);
                    }
                    (StringValue(value1), StringValue(value2), EqualEqual) => expr = Bool(value1 == value2),
                    (StringValue(value1), StringValue(value2), BangEqual) => expr = Bool(value1 != value2),
                    (StringValue(_), _, token_type) => {
                        if vec![Plus, EqualEqual, BangEqual].contains(&token_type) {
                            panic!("Operator `{}` isn't implemented for {} and {} types", operator.lexeme, left.get_type(), right.get_type());
                        }
                        panic!("Operator `{}` isn't implemented for {} type", operator.lexeme, left.get_type());
                    }
                    (CharValue(value1), CharValue(value2), EqualEqual) => expr = Bool(value1 == value2),
                    (CharValue(value1), CharValue(value2), BangEqual) => expr = Bool(value1 != value2),
                    (CharValue(value1), CharValue(value2), Greater) => expr = Bool(value1 > value2),
                    (CharValue(value1), CharValue(value2), Less) => expr = Bool(value1 < value2),
                    (CharValue(value1), CharValue(value2), GreaterEqual) => expr = Bool(value1 >= value2),
                    (CharValue(value1), CharValue(value2), LessEqual) => expr = Bool(value1 <= value2),
                    (CharValue(_), _, token_type) => {
                        if vec![EqualEqual, BangEqual, Greater, Less, GreaterEqual, LessEqual].contains(&token_type) {
                            panic!("Operator `{}` isn't implemented for {} and {} types", operator.lexeme, left.get_type(), right.get_type());
                        }
                        panic!("Operator `{}` isn't implemented for {} type", operator.lexeme, left.get_type());
                    }
                    
                    (Bool(value1), Bool(value2), EqualEqual) => expr = Bool(value1 == value2),
                    (Bool(value1), Bool(value2), BangEqual) => expr = Bool(value1 != value2),
                    (Bool(value1), NullLiteral(_), EqualEqual) => expr = Bool(value1 == false),
                    (Bool(value1), NullLiteral(_), BangEqual) => expr = Bool(value1 != false),
                    (Bool(_), _, token_type) => {
                        if vec![EqualEqual, BangEqual].contains(&token_type) {
                            panic!("Operator `{}` isn't implemented for {} and {} types", operator.lexeme, left.get_type(), right.get_type());
                        }
                        panic!("Operator `{}` isn't implemented for {} type", operator.lexeme, left.get_type());
                    }

                    (NullLiteral(_), NullLiteral(_), EqualEqual) => expr = Bool(true),
                    (NullLiteral(_), NullLiteral(_), BangEqual) => expr = Bool(false),
                    (NullLiteral(_), _, token_type) => {
                        if vec![EqualEqual, BangEqual].contains(&token_type) {
                            panic!("Operator `{}` isn't implemented for {} and {} types", operator.lexeme, left.get_type(), right.get_type());
                        }
                        panic!("Operator `{}` isn't implemented for {} type", operator.lexeme, left.get_type());
                    }
                    
                    (_, NullLiteral(_), EqualEqual) => expr = Bool(false),
                    (_, NullLiteral(_), BangEqual) => expr = Bool(true),
                    
                    _ => {
                        expr = gen_binary_expressions!(
                            left.clone(), right.clone(), operator,
                            I8Value             | I8Value             => I8Value,       i8,
                            I16Value            | I16Value            => I16Value,      i16,
                            I32Value            | I32Value            => I32Value,      i32,
                            I64Value            | I64Value            => I64Value,      i64,
                            I128Value           | I128Value           => I128Value,     i128,
                            ISizeValue          | ISizeValue          => ISizeValue,    isize,

                            U8Value             | U8Value             => U8Value,       u8,
                            U16Value            | U16Value            => U16Value,      u16,
                            U32Value            | U32Value            => U32Value,      u32,
                            U64Value            | U64Value            => U64Value,      u64,
                            U128Value           | U128Value           => U128Value,     u128,
                            USizeValue          | USizeValue          => USizeValue,    usize,

                            F32Value            | F32Value            => F32Value,      f32,
                            F64Value            | F64Value            => F64Value,      f64,

                            F64Value            | UndefinedFloatValue => F64Value,      f64,
                            I128Value           | UndefinedIntValue   => I128Value,     i128,

                            I8Value             | UndefinedIntValue   => I8Value,       i8,
                            I16Value            | UndefinedIntValue   => I16Value,      i16,
                            I32Value            | UndefinedIntValue   => I32Value,      i32,
                            I64Value            | UndefinedIntValue   => I64Value,      i64,
                            I128Value           | UndefinedIntValue   => I128Value,     i128,
                            ISizeValue          | UndefinedIntValue   => ISizeValue,    isize,

                            U8Value             | UndefinedIntValue   => U8Value,       u8,
                            U16Value            | UndefinedIntValue   => U16Value,      u16,
                            U32Value            | UndefinedIntValue   => U32Value,      u32,
                            U64Value            | UndefinedIntValue   => U64Value,      u64,
                            U128Value           | UndefinedIntValue   => U128Value,     u128,
                            USizeValue          | UndefinedIntValue   => USizeValue,    usize,

                            UndefinedFloatValue | F32Value            => F32Value,      f32,
                            UndefinedFloatValue | F64Value            => F64Value,      f64,

                            UndefinedIntValue   | I8Value             => I8Value,       i8,
                            UndefinedIntValue   | I16Value            => I16Value,      i16,
                            UndefinedIntValue   | I32Value            => I32Value,      i32,
                            UndefinedIntValue   | I64Value            => I64Value,      i64,
                            UndefinedIntValue   | I128Value           => I128Value,     i128,
                            UndefinedIntValue   | ISizeValue          => ISizeValue,    isize,

                            UndefinedIntValue   | U8Value             => U8Value,       u8,
                            UndefinedIntValue   | U16Value            => U16Value,      u16,
                            UndefinedIntValue   | U32Value            => U32Value,      u32,
                            UndefinedIntValue   | U64Value            => U64Value,      u64,
                            UndefinedIntValue   | U128Value           => U128Value,     u128,
                            UndefinedIntValue   | USizeValue          => USizeValue,    usize,
                            
                            UndefinedFloatValue | UndefinedFloatValue => UndefinedFloatValue,   f64,
                            UndefinedIntValue   | UndefinedIntValue   => UndefinedIntValue,  i128
                        );
                    }
                }
                return expr;
            }
        }
    }
}
