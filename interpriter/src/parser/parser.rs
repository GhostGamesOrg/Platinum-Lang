use crate::lexer::token::{Token, TokenType, TokenType::*};

use super::expr::Expr;

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

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_tokens(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right)
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        
        while self.match_tokens(
            vec![
                Greater,
                GreaterEqual,
                Less,
                LessEqual
                ]
            ) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right)
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_tokens(vec![Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right)
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_tokens(vec![Slash, Star]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right)
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(vec![Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary { operator: operator, right: Box::from(right) })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();

        let result: Expr;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')'")?;
                result = Expr::Grouping { expression: Box::from(expr) }
            }
            Number | Char | StringT | BoolT | Null => {
                self.advance();
                result = Expr::Literal { value: token.literal.unwrap() }
            },
            _ => return Err("".to_string()),
        }

        Ok(result)

        // if self.match_token(LeftParen) {
        //     let expr = self.expression()?;
        //     self.consume(RightParen, "Expected ')'")?;
        //     Ok(Expr::Grouping { expression: Box::from(expr) })
        // } else {
        //     let token = self.peek();
        //     self.advance();
        //     Ok(Expr::Literal { value: token.literal.unwrap() })
        // }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        } else {
            return Err(msg.to_string());
        }
        Ok(())
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == token_type {
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

