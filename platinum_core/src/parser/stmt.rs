use crate::lexer::token::Token;

use super::expr::Expression;

#[derive(Debug, PartialEq)]
pub enum Argument {
    NotOptional { name: Token, _type: Token },
    Optional { name: Token, _type: Token, value: Expression},
}

#[derive(Debug, PartialEq)]
pub enum UseArgument {
    Expr { value: Expression},
    Optional { name: Token, value: Expression},
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block { statements: Vec<Statement> },
    Assigment { expression: Expression },
    Let { mutable: bool, defined: bool, _type: Token, name: Token, value: Box<Statement>},
    Function { name: Token, _type: Token, arguments: Vec<Argument>, block: Box<Statement>},
    FunctionUse { name: Token, arguments: Vec<UseArgument>},
    IfElse { condition: Box<Statement>, if_block: Box<Statement>, else_block: Option<Box<Statement>> },
    Loop { block: Box<Statement> },
    For { var: Token, container: Box<Statement>, block: Box<Statement> },
    RangeIter { start_num: Box<Statement>, end_num: Box<Statement> },
    While { condition: Box<Statement>, block: Box<Statement> },
    DoWhile { block: Box<Statement>, condition: Box<Statement> },
    Break,
    Continue,
    Return { returned: Box<Statement> }
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
            Statement::Function { name, _type, arguments, block } => {
                format!(
                    "(fun {}({:?}) -> {} {})",
                    name.to_string(),
                    arguments,
                    _type.to_string(),
                    block.to_string()
                )
            }
            Statement::FunctionUse { name, arguments } => {
                format!(
                    "(functionUse {}({:?})",
                    name.to_string(),
                    arguments,
                )
            }
            Statement::IfElse { condition, if_block, else_block } => {
                match else_block {
                    Some(block) => {
                        format!(
                            "(if ({}) {} else {})",
                            condition.to_string(),
                            if_block.to_string(),
                            block.to_string()
                        )
                    }
                    None => {
                        format!(
                            "(if ({}) {})",
                            condition.to_string(),
                            if_block.to_string()
                        )
                    }
                }
            }
            Statement::Loop { block } => {
                format!(
                    "(loop {})",
                    block.to_string()
                )
            }
            Statement::RangeIter { start_num, end_num } => {
                format!(
                    "(range {}..{})",
                    start_num.to_string(),
                    end_num.to_string()
                )
            }
            Statement::For { var, container, block } => {
                format!(
                    "(for ({} in {}) {})",
                    var.to_string(),
                    container.to_string(),
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
            Statement::Break => "(break)".to_string(),
            Statement::Continue => "(continue)".to_string(),
            Statement::Return { returned } => {
                format!(
                    "(return {})",
                    returned.to_string()
                )
            }
        }
    }
}
