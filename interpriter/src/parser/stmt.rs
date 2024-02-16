use super::expr::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block { statements: Vec<Statement> },
    Assigment { expression: Expression }
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
            Statement::Assigment { expression } => {
                format!(
                    "(assigment {})",
                    expression.to_string()
                )
            }
        }
    }
}
