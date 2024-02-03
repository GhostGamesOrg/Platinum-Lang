use std::str::Chars;

use crate::lexer::token::{*, TokenType::*};

macro_rules! add_single_tokens {
    ($self:expr, $( $c:expr => $token:ident ),*) => {
        match $self.current {
            $(
                $c => {
                    $self.add_token($token, ($self.line, $self.get_pos() - 1, $self.get_pos()));
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
        "and" => Some(And),
        "or" => Some(Or),
        "if" => Some(If),
        "else" => Some(Else),
        "class" => Some(Class),
        "super" => Some(Super),
        "this" => Some(This),
        "fun" => Some(Fun),
        "return" => Some(Return),
        "for" => Some(For),
        "while" => Some(While),
        "do" => Some(DoWhile),
        "loop" => Some(Loop),
        "break" => Some(Break),
        "continue" => Some(Continue),
        "let" => Some(Let),
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
        self.add_token(EOF, (self.line, self.get_pos(), self.get_pos()));

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
                        MinusEqual
                    } else if self.char_match('-') {
                        MinusMinus
                    } else {
                        Minus
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '+' => {
                let token = {
                    if self.char_match('=') {
                        PlusEqual
                    } else if self.char_match('+') {
                        PlusPlus
                    } else {
                        Plus
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '/' => {
                let token = {
                    if self.char_match('/') {
                        let comment = {
                            if self.char_match('/') {
                                DocComent
                            } else {
                                Coment
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
                        SlashEqual
                    } else {
                        Slash
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '*' => {
                let token = {
                    if self.char_match('=') {
                        StarEqual
                    } else {
                        Star
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '!' => {
                let token = {
                    if self.char_match('=') {
                        BangEqual
                    } else {
                        Bang
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '>' => {
                let token = {
                    if self.char_match('=') {
                        GreaterEqual
                    } else if self.char_match('>') {
                        if self.char_match('=') {
                            GreaterGreaterEqual
                        } else if self.char_match('>') {
                            GreaterGreaterGreater
                        } else {
                            GreaterGreater
                        }
                    } else{
                        Greater
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '<' => {
                let token = {
                    if self.char_match('=') {
                        LessEqual
                    } else if self.char_match('<') {
                        if self.char_match('=') {
                            LessLessEqual
                        } else {
                            LessLess
                        }
                    } else {
                        Less
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '&' => {
                let token = {
                    if self.char_match('&') {
                        And
                    } else {
                        Ampersant
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '|' => {
                let token = {
                    if self.char_match('|') {
                        Or
                    } else {
                        Bar
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '?' => {
                let token = {
                    if self.char_match('?') {
                        QuestionQuestion
                    } else {
                        Question
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '=' => {
                let token = {
                    if self.char_match('=') {
                        EqualEqual
                    } else {
                        Equal
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            ' ' | '\r' | '\t' | '\0' => {}
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
                    println!("{}", c as u8);
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
        
        if buffer == "true".to_string() {
            self.add_token_lit(BoolT, Some(LiteralValue::Bool(true)), (self.line, pos_start, self.get_pos()));
        } else if buffer == "false".to_string() {
            self.add_token_lit(BoolT, Some(LiteralValue::Bool(false)), (self.line, pos_start, self.get_pos()));
        } else if buffer == "null".to_string() {
            self.add_token_lit(Null, Some(LiteralValue::NullLiteral(false)), (self.line, pos_start, self.get_pos()));
        } else {
            match str_to_keyword(&buffer) {
                Some(token_type) => {
                    self.add_token(token_type, (self.line, pos_start, self.get_pos()));
                }
                _ => {
                    self.add_token_lit(Identifier, Some(LiteralValue::IdentifierValue(buffer)), (self.line, pos_start, self.get_pos()));
                }
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

        self.add_token_lit(StringT, Some(LiteralValue::StringValue(buffer)), (self.line, pos_start, self.get_pos()));

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

        self.add_token_lit(Char, Some(LiteralValue::CharValue(result)), (self.line, pos_start, self.get_pos()));

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
            if self.next.is_digit(10) || self.next == '.' || self.next == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let literal = {
            if with_dot {
                if self.char_match('f') || self.char_match('F') {
                    if self.char_match('3') && self.char_match('2') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<f32>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `f32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };

                        LiteralValue::F32Value(number)
                    } else if self.char_match('6') && self.char_match('4') {                        
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
                if self.char_match('i') || self.char_match('I') {                    
                    if self.char_match('8') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<i8>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i8` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I8Value(number)
                        
                    } else if self.char_match('1') {
                        if self.char_match('6') {
                            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                            let number = match buffer.parse::<i16>() {
                                Ok(num) => num,
                                Err(_) => return Err(format!("To big value for type `i16` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                            };
                            LiteralValue::I16Value(number)
                            
                        } else if self.char_match('2') && self.char_match('8') {
                            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                            let number = match buffer.parse::<i128>() {
                                Ok(num) => num,
                                Err(_) => return Err(format!("To big value for type `i128` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                            };
                            LiteralValue::I128Value(number)
                        } else {
                            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                            return Err(format!("Unknown number type at  [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        }
                        
                    } else if self.char_match('3') && self.char_match('2') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<i32>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I32Value(number)
                        
                    } else if self.char_match('6') && self.char_match('4') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<i64>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I64Value(number)
                        
                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<isize>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `isize` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::ISizeValue(number)

                    }
                } else if self.char_match('u') || self.char_match('U') {
                   
                    if self.char_match('8') {
                        self.advance();

                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<u8>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u8` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U8Value(number)
                        
                    
                    } else if self.char_match('1') {
                        if self.char_match('6') {
                            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                            let number = match buffer.parse::<u16>() {
                                Ok(num) => num,
                                Err(_) => return Err(format!("To big value for type `u16` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                            };
                            LiteralValue::U16Value(number)
                            
                        } else if self.char_match('2') && self.char_match('8') {
                            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                            let number = match buffer.parse::<u128>() {
                                Ok(num) => num,
                                Err(_) => return Err(format!("To big value for type `u128` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                            };
                            LiteralValue::U128Value(number)
                        } else {
                            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                            return Err(format!("Unknown number type at  [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        }
                        } else if self.char_match('3') && self.char_match('2') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U32Value(number)
                        
                    } else if self.char_match('6') && self.char_match('4') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<u64>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U64Value(number)
                        
                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<usize>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `usize` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::USizeValue(number)
                    }
                } else if self.char_match('f') || self.char_match('F') {
                    if self.char_match('3') && self.char_match('2') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match buffer.parse::<f32>() {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `f32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };

                        LiteralValue::F32Value(number)
                    } else if self.char_match('6') && self.char_match('4') {
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
        
        self.add_token_lit(Number, Some(literal), (self.line, pos_start, self.get_pos()));
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
            if self.char_match('f') || self.char_match('F') {
                if self.char_match('3') && self.char_match('2') {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match i128::from_str_radix(&buffer, 16) {
                        Ok(num) => num as f32,
                        Err(_) => return Err(format!("To big value for type `f32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };

                    LiteralValue::F32Value(number)
                } else if self.char_match('6') && self.char_match('4') {                        
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match i128::from_str_radix(&buffer, 16) {
                        Ok(num) => num as f64,
                        Err(_) => return Err(format!("To big value for type `f64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };

                    LiteralValue::F64Value(number)
                } else {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    return Err(format!("Unrecognized number type at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                }
            } else if self.char_match('i') || self.char_match('I') {                    
                if self.char_match('8') {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match i8::from_str_radix(&buffer, 16) {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for type `i8` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::I8Value(number)
                    
                } else if self.char_match('1') {
                    if self.char_match('6') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match i16::from_str_radix(&buffer, 16) {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i16` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I16Value(number)
                        
                    } else if self.char_match('2') && self.char_match('8') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match i128::from_str_radix(&buffer, 16) {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `i128` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::I128Value(number)
                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unknown number type at  [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    }
                    
                } else if self.char_match('3') && self.char_match('2') {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match i32::from_str_radix(&buffer, 16) {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for type `i32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::I32Value(number)
                    
                } else if self.char_match('6') && self.char_match('4') {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match i64::from_str_radix(&buffer, 16) {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for type `i64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::I64Value(number)
                    
                } else {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match isize::from_str_radix(&buffer, 16) {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for type `isize` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::ISizeValue(number)

                }
            } else if self.char_match('u') || self.char_match('U') {
               
                if self.char_match('8') {
                    self.advance();

                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match u8::from_str_radix(&buffer, 16) {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for type `u8` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::U8Value(number)
                    
                
                } else if self.char_match('1') {
                    if self.char_match('6') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match u16::from_str_radix(&buffer, 16) {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u16` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U16Value(number)
                        
                    } else if self.char_match('2') && self.char_match('8') {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        let number = match u128::from_str_radix(&buffer, 16) {
                            Ok(num) => num,
                            Err(_) => return Err(format!("To big value for type `u128` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        };
                        LiteralValue::U128Value(number)
                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unknown number type at  [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    }
                    } else if self.char_match('3') && self.char_match('2') {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match u32::from_str_radix(&buffer, 16) {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for type `u32` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::U32Value(number)
                    
                } else if self.char_match('6') && self.char_match('4') {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match u64::from_str_radix(&buffer, 16) {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for type `u64` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::U64Value(number)
                    
                } else {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    let number = match usize::from_str_radix(&buffer, 16) {
                        Ok(num) => num,
                        Err(_) => return Err(format!("To big value for type `usize` [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    };
                    LiteralValue::USizeValue(number)
                }
            } else {
                let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                let number = match i128::from_str_radix(&buffer, 16) {
                    Ok(num) => num,
                    Err(_) => return Err(format!("To big value for the integer number [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                };
                LiteralValue::UndefinedIntValue(number)
            }
        };

        self.add_token_lit(Number, Some(literal), (self.line, pos_start, self.get_pos()));
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
