use crate::lexer::token::*;

macro_rules! add_single_tokens {
    ($self:expr, $_char:expr, $( $c:expr => $token:ident ),*) => {
        match $_char {
            $(
                $c => $self.add_token(TokenType::$token, ($self.line, $self.get_pos() - 1, $self.get_pos())),

            )*
            _ => {
                // Handle other cases or generate an error for unknown tokens
            }
        }
    };
}

pub struct Scanner {
    src: String,
    pub tokens: Vec<Token>,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: &str) -> Scanner {
        Scanner {
            src: src.to_string(),
            tokens: vec![],
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors: Vec<String> = vec![];

        while !self.is_at_end() {
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        // When programm stops scanning tokens, it adds EOF token.
        self.add_token(TokenType::EOF, (self.line, self.get_pos(), self.get_pos()));

        if errors.len() > 0 {
            let mut joined = "".to_string();
            for msg in errors.iter() {
                joined.push_str(&msg);
                joined.push_str("\n");
            }
            return Err(joined);
        }
        
        Ok(self.tokens.clone())
    }

    pub fn scan_token(&mut self) -> Result<(), String> {
        let pos_start = self.get_pos();
        let c = self.advance();
        
        add_single_tokens!(
            self, c,
            '(' => LeftParen,
            ')' => RightParen,
            '[' => LeftBrace,
            ']' => RightBrace,
            '{' => LeftCurBrace,
            '}' => RightCurBrace,
            ',' => Comma,
            '.' => Dot,
            ';' => Semicolon,
            ':' => Colon,
            '~' => Tilde
        );

        match c {
            '-' => {
                let token = {
                    if self.char_match('=') {
                        TokenType::MinusEqual
                    } else if self.char_match('-') {
                        TokenType::MinusMinus
                    } else {
                        TokenType::Minus
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '+' => {
                let token = {
                    if self.char_match('=') {
                        TokenType::PlusEqual
                    } else if self.char_match('+') {
                        TokenType::PlusPlus
                    } else {
                        TokenType::Plus
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '/' => {
                let token = {
                    if self.char_match('/') {
                        let comment = {
                            if self.char_match('/') {
                                TokenType::DocComent
                            } else {
                                TokenType::Coment
                            }
                        };
                        loop {
                            if self.peek() == '\n' || self.is_at_end() {
                                break;
                            }
                            self.advance();
                        }
                        comment
                    } else if self.char_match('=') {
                        TokenType::SlashEqual
                    } else {
                        TokenType::Slash
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '*' => {
                let token = {
                    if self.char_match('=') {
                        TokenType::StarEqual
                    } else {
                        TokenType::Star
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '!' => {
                let token = {
                    if self.char_match('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '>' => {
                let token = {
                    if self.char_match('=') {
                        TokenType::GreaterEqual
                    } else if self.char_match('>') {
                        if self.char_match('=') {
                            TokenType::GreaterGreaterEqual
                        } else if self.char_match('>') {
                            TokenType::GreaterGreaterGreater
                        } else {
                            TokenType::GreaterGreater
                        }
                    } else{
                        TokenType::Greater
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '<' => {
                let token = {
                    if self.char_match('=') {
                        TokenType::LessEqual
                    } else if self.char_match('<') {
                        if self.char_match('=') {
                            TokenType::LessLessEqual
                        } else {
                            TokenType::LessLess
                        }
                    } else {
                        TokenType::Less
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '&' => {
                let token = {
                    if self.char_match('&') {
                        TokenType::And
                    } else {
                        TokenType::Ampersant
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '|' => {
                let token = {
                    if self.char_match('|') {
                        TokenType::Or
                    } else {
                        TokenType::Bar
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '?' => {
                let token = {
                    if self.char_match('?') {
                        TokenType::QuestionQuestion
                    } else {
                        TokenType::Question
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '=' => {
                let token = {
                    if self.char_match('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => {
                match self.string() {
                    Ok(_) => (),
                    Err(msg) => return Err(msg)
                }
            }
            _ => return Err(format!("Unrecognized char at possition [{} | {}:{}]: {}", self.line, pos_start, self.current, c)),
        }
        Ok(())
    }

    fn string(&mut self) -> Result<(), String> {
        let pos_start = self.get_pos() - 1;

        let mut buffer = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            let c = self.advance();

            match c {
                '\\' => {
                    if self.char_match('\\') {
                        buffer.push('\\')
                    } else if self.char_match('n') {
                        buffer.push('\n')
                    } else if self.char_match('r') {
                        buffer.push('\r')
                    } else if self.char_match('t') {
                        buffer.push('\t')
                    } else if self.char_match('0') {
                        buffer.push('\0')
                    } else if self.char_match('"') {
                        buffer.push('\"')
                    } else if self.char_match('u') {
                        // let mut u_buffer = String::new();
                        // let u_c = self.advance();
                        // while u_c.is_digit(16) {
                        //     
                        // }
                        // buffer.push('\u');
                        todo!("Unicode");
                    } else {
                        return Err("Unexpected charrecter".to_string());
                    }
                },
                _ => buffer.push(c)
            }
        }
        
        if self.is_at_end() {
            let lexeme = self.get_lexeme((self.line, pos_start, self.current));
            return Err(format!("Unterminated string at possition [{} | {}:{}]: {}", self.line, pos_start, self.current, lexeme));
        }
        
        self.advance();

        self.add_token_lit(TokenType::String, Some(LiteralValue::StringValue(buffer)), (self.line, pos_start, self.get_pos()));

        Ok(())
    }

    fn advance(&mut self) -> char {
        let current_char = self.src.as_bytes()[self.current];
        self.current += 1;

        current_char as char
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.src.as_bytes()[self.current] as char
    }

    // fn peek_pos(&self, pos: usize) -> Option<char> {
    //     self.src.chars().nth(self.current + pos)
    // }
    
    fn get_pos(&self) -> usize {
        self.current
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn get_lexeme(&self, possition: Possition) -> String {
        let mut lexeme = String::new();
        let bytes = self.src.as_bytes();

        for i in possition.1..possition.2 {
            lexeme.push(bytes[i] as char);
        }

        lexeme
    }

    fn add_token(&mut self, token_type: TokenType, possition: Possition) {
        self.add_token_lit(token_type, None, possition);
    }

    fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>, possition: Possition) {
        let lexeme = self.get_lexeme(possition);
        self.tokens.push(Token::new(token_type, lexeme, literal, possition));
    }
}
