use crate::lexer::token::{Token, TokenType, TokenType::*};

use super::{expr::Expression, stmt::Statement};

pub struct Parser {
    file_path: String,
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(file_path: &str, tokens: Vec<Token>) -> Self {
        Self {
            file_path: file_path.to_string(),
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmts = vec![];
        let mut errors = vec![];
        
        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(msg) => {
                    errors.push(msg);
                    self.synchronize();
                }

            }
            self.advance();
        }

        if errors.len() > 0 {
            let mut joined = "".to_string();
            for msg in errors.iter() {
                joined.push_str(&msg);
                joined.push_str("\n");
            }
            return Err(joined);
        }
        Ok(stmts)
    }


    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_token(LeftCurBrace) {
            return self.block_statement();
        }
        if self.match_token(Fun) {
            return self.func_statement(); // todo
        }
        if self.match_token(For) {
            return self.for_statement(); // todo
        }
        if self.match_token(While) {
            return self.while_statement(); // todo
        }
        if self.match_token(DoWhile) {
            return self.do_while_statement(); // todo
        }
        if self.match_token(Loop) {
            return self.loop_statement(); // todo
        }
        self.assigment_statement()
    }

    fn assigment_statement(&mut self) -> Result<Statement, String> {
        let expression = self.expression()?;
        Ok(Statement::Assigment { expression: expression })
    }

    fn block_statement(&mut self) -> Result<Statement, String> {
        let mut statements = Vec::new();
        let _ = self.consume(LeftCurBrace, "`}` expected");
        while !self.match_token(RightCurBrace) {
            statements.push(self.statement()?);
        }
        Ok(Statement::Block { statements: statements })
    }
    
    fn func_statement(&mut self) -> Result<Statement, String> {
        todo!()
        // self.consume(Fun, "Key `fun` expected");
        // let name = self.consume(Identifier { value: String::new() }, "Identifier expected, for function declaration.")?;
    }

    fn for_statement(&mut self) -> Result<Statement, String> {
        todo!()
    }

    fn while_statement(&mut self) -> Result<Statement, String> {
        todo!()
    }

    fn do_while_statement(&mut self) -> Result<Statement, String> {
        todo!()
    }

    fn loop_statement(&mut self) -> Result<Statement, String> {
        todo!()
    }


    fn expression(&mut self) -> Result<Expression, String> {
        self.assigment()?.optimize_expression()
    }
    
    fn assigment(&mut self) -> Result<Expression, String> {

        self.ternary()
    }

    fn ternary(&mut self) -> Result<Expression, String> {
        let mut result: Expression = self.null_coalesce()?;
        if self.match_token(Question) {
            let true_expression = self.expression()?;

            let _ = self.consume(Colon, "`:` expected after left result");
            let false_expression = self.expression()?;
            
            result = Expression::Ternary {
                result: Box::from(result),
                true_expression: Box::from(true_expression),
                false_expression: Box::from(false_expression)
            };
        }
        Ok(result)
    }

    fn null_coalesce(&mut self) -> Result<Expression, String> {
        let mut result: Expression = self.logical_or()?;
        while self.match_token(QuestionQuestion) {
            let op = self.previous();
            let right = self.logical_or()?;
            result = Expression::EqualtyComparison {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn logical_or(&mut self) -> Result<Expression, String> {
        let mut result = self.logical_and()?;
        while self.match_token(Or) {
            let op = self.previous();
            let right = self.logical_and()?;
            result = Expression::EqualtyComparison {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn logical_and(&mut self) -> Result<Expression, String> {
        let mut result = self.bitwise_or()?;
        while self.match_token(And) {
            let op = self.previous();
            let right = self.bitwise_or()?;
            result = Expression::EqualtyComparison {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn bitwise_or(&mut self) -> Result<Expression, String> {
        let mut result = self.bitwise_xor()?;
        while self.match_token(Bar) {
            let op = self.previous();
            let right = self.bitwise_xor()?;
            result = Expression::Binary {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn bitwise_xor(&mut self) -> Result<Expression, String> {
        let mut result = self.bitwise_and()?;
        while self.match_token(Caret) {
            let op = self.previous();
            let right = self.bitwise_and()?;
            result = Expression::Binary {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn bitwise_and(&mut self) -> Result<Expression, String> {
        let mut result = self.equalty()?;
        while self.match_token(Ampersant) {
            let op = self.previous();
            let right = self.equalty()?;
            result = Expression::Binary {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn equalty(&mut self) -> Result<Expression, String> {
        let mut result = self.comparison()?;
        while self.match_tokens(vec![EqualEqual, BangEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            result = Expression::EqualtyComparison {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut result = self.shift()?;
        while self.match_tokens(vec![LessEqual, Less, GreaterEqual, Greater]) {
            let op = self.previous();
            let right = self.shift()?;
            result = Expression::EqualtyComparison {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn shift(&mut self) -> Result<Expression, String> {
        let mut result = self.term()?;
        while self.match_tokens(vec![LessLess, GreaterGreater]) {
            let op = self.previous();
            let right = self.term()?;
            result = Expression::Binary {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut result = self.factor()?;
        while self.match_tokens(vec![Plus, Minus]) {
            let op = self.previous();
            let right = self.factor()?;
            result = Expression::Binary {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut result = self.unary()?;
        while self.match_tokens(vec![Star, Slash, Persent]) {
            let op = self.previous();
            let right = self.unary()?;
            result = Expression::Binary {
                left: Box::from(result),
                operator: op,
                right: Box::from(right)
            };
        }
        Ok(result)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(vec![Bang, Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            Ok(Expression::Unary {
                operator: op,
                right: Box::from(right),
            })
        } else {
            self.primary()
        }
    }
    
    fn primary(&mut self) -> Result<Expression, String> {
        let token = self.peek();
        let result;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')'")?;
                result = Expression::Grouping {
                    expression: Box::from(expr),
                };
            }
            Int {..} | Float {..} | StringT {..} | BoolT {..} | Char {..} => {
                self.advance();
                result = Expression::Literal {
                    value: token,
                }
            }
            _ => return Err(format!("Expected expression: {:?}", token)),
        }

        Ok(result)
    }


    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type.eq_token(token_type) {
            self.advance();
            Ok(token)
        } else {
            return Err(msg.to_string());
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type.eq_token(token_type) {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.match_token(token_type) {
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EOF
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                And | Or | If | Else |
                Class | Super | This | Fun |
                Return | For | While | DoWhile |
                Loop | Break | Continue | Null |
                Let => return,
                _ => (),
            }
            
            self.advance();
        }
    }
}

