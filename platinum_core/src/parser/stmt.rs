use crate::lexer::token::Token;

use super::expr::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block { statements: Vec<Statement> },
    Assigment { expression: Expression },
    Let { mutable: bool, defined: bool, _type: Token, name: Token, value: Box<Statement>},
    Loop { block: Box<Statement> },
    While { condition: Expression, block: Box<Statement> },
    DoWhile { block: Box<Statement>, condition: Expression },
}

impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            Statement::Block { statements } => {
                let mut result = "(block \n".to_string();
                for statement in statements {
                    result.push_str(&statement.to_string());
                    result.push('\n');
                }
                result.push(')');
                return result;
            }
            Statement::Let { mutable, defined, _type, name, value } => {
                format!(
                    "(let{} {}: {} = {})",
                    mutable.then_some(" mut").unwrap_or(""),
                    name.to_string(),
                    _type.to_string(),
                    value.to_string()
                )
            }
            Statement::Assigment { expression } => {
                format!(
                    "(assigment {})",
                    expression.to_string()
                )
            }
            Statement::Loop { block } => {
                format!(
                    "(loop {})",
                    block.to_string()
                )
            }
            Statement::While { condition, block } => {
                format!(
                    "(while {} {})",
                    condition.to_string(),
                    block.to_string()
                )
            }
            Statement::DoWhile { block, condition } => {
                format!(
                    "(do {} while {})",
                    block.to_string(),
                    condition.to_string()
                )
            }
        }
    }
}
