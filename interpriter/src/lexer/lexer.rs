use std::str::Chars;

use crate::lexer::token::*;

macro_rules! add_single_tokens {
    ($self:expr, $( $c:expr => $token:ident ),*) => {
        match $self.current {
            $(
                $c => {
                    $self.add_token(TokenType::$token, ($self.line, $self.get_pos() - 1, $self.get_pos()));
                    return Ok(());
                }

            )*
            _ => {
                // Handle other cases or generate an error for unknown tokens
            }
        }
    };
}

fn is_hex(c: char) -> bool {
    c.is_digit(16)
}

fn is_idetifier_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn str_to_keyword(string: &str) -> Option<TokenType> {
    match string {
        "and" => Some(TokenType::And),
        "or" => Some(TokenType::Or),
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "class" => Some(TokenType::Class),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "false" => Some(TokenType::False),
        "fun" => Some(TokenType::Fun),
        "return" => Some(TokenType::Return),
        "for" => Some(TokenType::For),
        "while" => Some(TokenType::While),
        "do" => Some(TokenType::DoWhile),
        "loop" => Some(TokenType::Loop),
        "break" => Some(TokenType::Break),
        "continue" => Some(TokenType::Continue),
        "nil" => Some(TokenType::Nil),
        "let" => Some(TokenType::Let),
        _ => None
    }
}

pub struct Scanner<'s> {
    file_path: String,
    src: String,
    chars: Chars<'s>,
    pub tokens: Vec<Token>,
    current_pos: usize,
    current: char,
    next: char,
    line: usize,
    col: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(file_path: &str, src: &'s str) -> Scanner<'s> {
        Scanner {
            file_path: file_path.to_string(),
            src: src.to_string(),
            chars: src.chars(),
            tokens: vec![],
            current_pos: 0,
            current: ' ',
            next: ' ',
            line: 1,
            col: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors: Vec<String> = vec![];

        self.advance();
        self.current_pos = 0;

        while !self.is_at_end() {
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => {
                    println!("ERROR!");
                    errors.push(msg);
                    break;
                },
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
        self.advance();
        
        add_single_tokens!(
            self,
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

        match self.current {
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
                            if self.current == '\n' || self.is_at_end() {
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
            '\n' => {
                self.line += 1;
                self.col = 1;
            }
            '"' => {
                match self.string() {
                    Ok(_) => (),
                    Err(msg) => return Err(msg)
                }
            }
            '\'' => {
                match self.char() {
                    Ok(_) => (),
                    Err(msg) => return Err(msg)
                }
            }
            c => {
                if c.is_digit(10) {
                    if c == '0' {
                        if self.char_match('X') || self.char_match('x') {
                            match self.hex_number() {
                                Ok(_) => (),
                                Err(msg) => return Err(msg)
                            }
                        }
                    } else {
                        match self.number() {
                            Ok(_) => (),
                            Err(msg) => return Err(msg)
                        }
                    }
                } else if is_idetifier_char(c) {
                    match self.identifier() {
                        Ok(_) => (),
                        Err(msg) => return Err(msg)
                    };
                } else {
                    return Err(format!("Unrecognized char at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, c));
                }
            }
        }
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), String> {
        let pos_start = self.get_pos() - 1;

        let mut buffer = String::new();
        
        while is_idetifier_char(self.current) {
            buffer.push(self.current);

            if self.is_at_end() {
                break;
            }

            self.advance();
        }

        match str_to_keyword(&buffer) {
            Some(token_type) => {
                self.add_token(token_type, (self.line, pos_start, self.get_pos()));
            }
            _ => {
                self.add_token_lit(TokenType::Identifier, Some(LiteralValue::IdentifierValue(buffer)), (self.line, pos_start, self.get_pos()));
            }
        }

        Ok(())
    }

    fn string(&mut self) -> Result<(), String> {
        let pos_start = self.get_pos() - 1;

        let mut buffer = String::new();

        self.advance();
        while self.current != '"' && !self.is_at_end() {
            if self.current == '\n' {
                self.line += 1;
            }

            match self.current {
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
                        self.advance();
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unexpected charrecter at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                    }
                },
                _ => buffer.push(self.current)
            }
            
            self.advance();
        }
        
        if self.is_at_end() && self.current != '"' {
            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
            return Err(format!("Unterminated string at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
        }

        self.add_token_lit(TokenType::String, Some(LiteralValue::StringValue(buffer)), (self.line, pos_start, self.get_pos()));

        Ok(())
    }

    fn char(&mut self) -> Result<(), String> {
        let pos_start = self.get_pos() - 1;
        let mut result: char = ' ';
        self.advance();
        while self.current != '\'' && !self.is_at_end() {
            if self.current == '\n' {
                self.line += 1;
                let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                return Err(format!("Unexpected charrecter at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
            }

            match self.current {
                '\\' => {
                    if self.char_match('\\') {
                        result = '\\';
                    } else if self.char_match('n') {
                        result = '\n';
                    } else if self.char_match('r') {
                        result = '\r';
                    } else if self.char_match('t') {
                        result = '\t';
                    } else if self.char_match('0') {
                        result = '\0';
                    } else if self.char_match('"') {
                        result = '\"';
                    } else if self.char_match('u') {
                        // let mut u_buffer = String::new();
                        // let u_c = self.advance();
                        // while u_c.is_digit(16) {
                        //     
                        // }
                        // buffer.push('\u');
                        todo!("Unicode");
                    } else {
                        self.advance();
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unexpected charrecter at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                    }
                },
                _ => result = self.current
            }

            self.advance();
        }
        
        if self.is_at_end() && self.current != '\'' {
            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
            return Err(format!("Unterminated char at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
        }

        self.add_token_lit(TokenType::Char, Some(LiteralValue::CharValue(result)), (self.line, pos_start, self.get_pos()));

        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
        let pos_start = self.get_pos() - 1;

        let mut buffer = String::new();

        let mut with_dot = false;

        loop {
            if self.current == '_' {
            } else if self.current == '.' {
                if with_dot {
                    self.advance();
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    return Err(format!("Unexpected dot in number at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                } else {
                    with_dot = true;
                    buffer.push(self.current);
                }
            } else if self.current.is_digit(10) {
                buffer.push(self.current);
            } else {
                break;
            }

            if self.is_at_end() {
                break;
            }
            self.advance();
        }

        let literal = {
            if with_dot {
                if self.current == 'f' || self.current == 'F' {
                    self.advance();

                    if self.current == '3' && self.char_match('2') {
                        self.advance();
                        
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<f32>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `f32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };

                        LiteralValue::F32Value(number)
                    } else if self.current == '6' && self.char_match('4') {
                        self.advance();
                        
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<f64>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `f64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };

                        LiteralValue::F64Value(number)
                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unrecognized number type at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                    }
                } else {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match buffer.parse::<f64>() {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for the float [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    
                    LiteralValue::UndefinedFloatValue(number)
                }
            } else {
                if self.current == 'i' || self.current == 'I' {
                    self.advance();

                    if self.current == '8' {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<i8>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i8` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I8Value(number)
                        
                    } else if self.current == '1' && self.char_match('6') {
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<i16>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i16` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I16Value(number)
                        
                    } else if self.current == '3' && self.char_match('2') {
                        self.advance();
                        
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<i32>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I32Value(number)
                        
                    } else if self.current == '6' && self.char_match('4') {
                        self.advance();
                        
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<i64>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I64Value(number)
                        
                    } else if self.current =='1' && self.char_match('2') && self.char_match('8') {
                        self.advance();
                        self.advance();
                        
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<i128>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i128` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I128Value(number)
                        
                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<isize>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `isize` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::ISizeValue(number)

                    }
                } else if self.current == 'u' || self.current == 'U' {
                    self.advance();

                    if self.current == '8' {
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<u8>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u8` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U8Value(number)
                        
                    } else if self.current == '1' && self.char_match('6') {
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<u16>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u16` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U16Value(number)
                        
                    } else if self.current == '3' && self.char_match('2') {
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U32Value(number)
                        
                    } else if self.current == '6' && self.char_match('4') {
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<u64>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U64Value(number)
                        
                    } else if self.current == '1' && self.char_match('2') && self.char_match('8') {
                        self.advance();
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<u128>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u128` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U128Value(number)
                        
                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<usize>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `usize` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::USizeValue(number)
                    }
                } else if self.current == 'f' || self.current == 'F' {
                    self.advance();
                    if self.current == '3' && self.char_match('2') {
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<f32>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `f32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };

                        LiteralValue::F32Value(number)
                    } else if self.current == '6' && self.char_match('4') {
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<f64>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `f64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };

                        LiteralValue::F64Value(number)
                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unrecognized number type at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                    }
                } else {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match buffer.parse::<i128>() {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for the integer number [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::UndefinedIntValue(number)
                }
            }
        };
        
        self.add_token_lit(TokenType::Number, Some(literal), (self.line, pos_start, self.get_pos()));
        Ok(())
    }

    fn hex_number(&mut self) -> Result<(), String> {
        let pos_start = self.get_pos() - 2;

        let mut buffer = String::new();

        self.advance();

        while is_hex(self.current) {
            buffer.push(self.current);
            self.advance();
        }

        let literal = {
            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
            let number = match i128::from_str_radix(&buffer, 16) {
                Ok(num) => num,
                Err(_) => return Err(format!("To big value for the integer number [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
            };
            LiteralValue::UndefinedIntValue(number)
        };

        self.add_token_lit(TokenType::Number, Some(literal), (self.line, pos_start, self.get_pos()));
        Ok(())
    }
    
    fn advance(&mut self) {
        let next_char = match self.chars.next() {
            Some(c) => c,
            _ => '\0'
        };
        self.current_pos += 1;
        self.col += 1;
    
        self.current = self.next;
        self.next = next_char;
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.next != expected {
            return false;
        }

        self.advance();
        true
    }

    fn get_pos(&self) -> usize {
        self.current_pos
    }

    fn is_at_end(&self) -> bool {
        self.current_pos >= self.src.len()
    }
    
    fn relative_pos_is_at_end(&self, relative: usize) -> bool {
        self.current_pos + relative >= self.src.len()
    }

    fn get_lexeme(&self, possition: Possition) -> String {
        let lexeme = &self.src.chars().skip(possition.1).take(possition.2 - possition.1).collect::<String>();
    
        lexeme.trim().to_string()
    }

    fn add_token(&mut self, token_type: TokenType, possition: Possition) {
        self.add_token_lit(token_type, None, possition);
    }

    fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>, possition: Possition) {
        let lexeme = self.get_lexeme(possition);
        self.tokens.push(Token::new(token_type, lexeme, literal, possition));
    }
}
