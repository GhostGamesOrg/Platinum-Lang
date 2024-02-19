use crate::{lexer::token::{Token, TokenType::{self, *}}, parser::stmt::{Argument, UseArgument}};

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
        if self.match_token(Let) {
            return self.let_statement();
        }
        if self.match_token(Fun) {
            return self.func_statement();
        }
        if self.match_token(For) {
            return self.for_statement();
        }
        if self.match_token(Range) {
            return self.range_statement();
        }
        if self.match_token(Break) {
            return self.break_statement();
        }
        if self.match_token(Continue) {
            return self.continue_statement();
        }
        if self.match_token(Return) {
            return self.return_statement();
        }
        if self.match_token(If) {
            return self.if_else_statement();
        }
        if self.match_token(While) {
            return self.while_statement();
        }
        if self.match_token(DoWhile) {
            return self.do_while_statement();
        }
        if self.match_token(Loop) {
            return self.loop_statement();
        }
        println!("{:?}", self.peek());
        if self.look_token(0, Identifier { value: String::new() }) && self.look_token(1, LeftParen) {
            return self.func_use_statement(); // todo
        }
        self.assigment_statement()
    }

    fn assigment_statement(&mut self) -> Result<Statement, String> {
        let expression = self.expression()?;
        Ok(Statement::Assigment { expression: expression })
    }

    fn block_statement(&mut self) -> Result<Statement, String> {
        let mut statements = Vec::new();
        while !self.match_tokens(vec![RightCurBrace, EOF]) {
            statements.push(self.statement()?);
        }
        if self.previous().token_type == EOF {
            return Err("Block statements wasn't closed".to_string());
        }
        Ok(Statement::Block { statements: statements })
    }
    
    fn let_statement(&mut self) -> Result<Statement, String> {

        let mutable = self.match_token(Mut);

        let name = self.consume(Identifier { value: String::new() }, "Identifier expected, for variable declaration.")?;
        let _ = self.consume(Colon, "`:` expected")?;
        let _type = self.consume(Identifier { value: String::new() }, "Type expected, for variable declaration.")?;

        let mut defined = false;
        let mut assigment_stmt = Statement::Assigment {
            expression: Expression::Literal {
                value: Token::new(
                    Null,
                    "null".to_string(),
                    _type.possition
                )
            }
        };
        if self.match_token(Equal) {
            defined = true;
            assigment_stmt = self.statement()?;
        }
        
        let _ = self.consume(Semicolon, "`;` expected after variable define statement")?;
        
        Ok(
            Statement::Let {
                mutable: mutable,
                defined: defined,
                _type: _type,
                name: name,
                value: Box::from(assigment_stmt)
            }
        )
    }
    
    fn func_statement(&mut self) -> Result<Statement, String> {
        let name = self.consume(Identifier { value: String::new() }, "Identifier expected, for function declaration.")?;

        let _ = self.consume(LeftParen, "`(` expected")?;
        let mut arguments: Vec<Argument> = vec![];
        let mut starts_optional_args = false;
        while !self.match_token(RightParen) {
            let arg_name = self.consume(Identifier { value: String::new() }, "Argument name expected")?;

            let _ = self.consume(Colon, "`:` expected")?;

            let arg_type = self.consume(Identifier { value: String::new() }, "Argument type expected")?;

            if self.match_token(Equal) {
                starts_optional_args = true;
                let value = self.expression()?;
                arguments.push(Argument::Optional { name: arg_name, _type: arg_type, value: value });
            } else if !starts_optional_args {
                arguments.push(Argument::NotOptional { name: arg_name, _type: arg_type });
            } else {
                return Err("Required argument cannot be after optional".to_string());
            }
            self.match_token(Comma);
        }

        let mut _type = Token::new(Null, "void".to_string(), self.previous().possition);
        
        if self.match_token(MinusGreater) {
            _type = self.consume(Identifier { value: String::new() }, "Returned type expected after `->`")?;
        }

        let _ = self.consume(LeftCurBrace, "`{` expected")?;

        let block = self.block_statement()?;

        Ok(
            Statement::Function { name, _type, arguments, block: Box::from(block) }
        )
    }
    
    fn range_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume(LeftBrace, "`[` expected")?;
        let one = self.statement()?;
        let _ = self.consume(Semicolon, "`;` expected")?;
        let two = self.statement()?;
        let _ = self.consume(RightBrace, "`]` expected")?;
        Ok(Statement::RangeIter { start_num: Box::from(one), end_num: Box::from(two) })
    }

    fn break_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume(Semicolon, "`;` expected")?;
        Ok(Statement::Break)
    }
    
    fn continue_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume(Semicolon, "`;` expected")?;
        Ok(Statement::Continue)
    }
    
    fn return_statement(&mut self) -> Result<Statement, String> {
        let returned = self.statement()?;
        let _ = self.consume(Semicolon, "`;` expected")?;
        Ok(Statement::Return { returned: Box::from(returned) })
    }
    
    fn if_else_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume(LeftParen, "`(` expected")?;
        let condition = self.statement()?;
        let _ = self.consume(RightParen, "`)` expected")?;
        let if_block = self.statement()?;
        let mut else_block = None;
        if self.match_token(Else) {
            else_block = Some(Box::from(self.statement()?));
        }

        Ok(Statement::IfElse { condition: Box::from(condition), if_block: Box::from(if_block), else_block: else_block })
    }

    fn for_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume(LeftParen, "`(` expected")?;

        let var = self.consume(Identifier { value: String::new() }, "Variable name expected")?;
        
        let _ = self.consume(In, "Variable name expected")?;

        let container = self.statement()?;
        
        let _ = self.consume(RightParen, "`)` expected")?;
        let a = self.consume(LeftCurBrace, "`{` expected")?;
        let block = self.block_statement()?;
        Ok(
            Statement::For {
                var,
                container: Box::from(container),
                block: Box::from(block)
            }
        )
    }

    fn while_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume(LeftParen, "`(` expected")?;
        let condition = self.statement()?;
        let _ = self.consume(RightParen, "`)` expected")?;
        let _ = self.consume(LeftCurBrace, "`{` expected")?;
        let block = self.block_statement()?;
        Ok(Statement::While { condition: Box::from(condition), block: Box::from(block) })
    }

    fn do_while_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume(LeftCurBrace, "`{` expected")?;
        let block = self.block_statement()?;
        let _ = self.consume(While, "`while` expected after block statement")?;
        let _ = self.consume(LeftParen, "`(` expected")?;
        let condition = self.statement()?;
        let _ = self.consume(RightParen, "`)` expected")?;
        let _ = self.consume(Semicolon, "`;` expected after variable define statement")?;
        Ok(Statement::DoWhile { block: Box::from(block), condition: Box::from(condition) })
    }

    fn loop_statement(&mut self) -> Result<Statement, String> {
        let _ = self.consume(LeftCurBrace, "`{` expected")?;
        Ok(Statement::Loop { block: Box::from(self.block_statement()?) })
    }

    fn func_use_statement(&mut self) -> Result<Statement, String> {
        let name = self.consume(Identifier { value: String::new() }, "Function name expected")?;
        let _ = self.consume(LeftParen, "`(` expected")?;
        let mut arguments: Vec<UseArgument> = vec![];
        let mut starts_optional_args = false;
        while !self.match_token(RightParen) {
            if self.look_token(0, Identifier { value: String::new() }) && self.look_token(1, Equal) {
                starts_optional_args = true;
                let arg_name = self.consume(Identifier { value: String::new() }, "Argument name expected")?;
                let _ = self.consume(Equal, "`=` expected")?;
                let value: Expression = self.expression()?;
                arguments.push(UseArgument::Optional { name: arg_name, value: value });
            } else if !starts_optional_args {
                arguments.push(UseArgument::Expr { value: self.expression()? });
            } else {
                return Err("Required argument cannot be after optional".to_string());
            }
            self.match_token(Comma);
        }
        Ok(Statement::FunctionUse { name, arguments })
    }


    fn expression(&mut self) -> Result<Expression, String> {
        self.assigment()?.optimize_expression()
    }
    
    fn assigment(&mut self) -> Result<Expression, String> {

        self.ternary()
    }

    fn ternary(&mut self) -> Result<Expression, String> {
        let mut result: Expression = self.logical_or()?;
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
            Int {..} | Float {..} | StringT {..} | BoolT {..} | Char {..} | Null => {
                self.advance();
                result = Expression::Literal {
                    value: token,
                }
            }
            Identifier { .. } => {
                self.advance();
                result = Expression::Variable {
                    name: token,
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

    fn look_token(&mut self, pos: usize, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.tokens[self.current + pos].token_type.eq_token(token_type) {
                true
            } else {
                false
            }
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

