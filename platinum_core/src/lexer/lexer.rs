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
    c.is_digit(10) || ('A'..='F').contains(&c) || c == '_'
}

fn is_idetifier_char_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_idetifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
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
        "in" => Some(In),
        "range" => Some(Range),
        "while" => Some(While),
        "do" => Some(DoWhile),
        "loop" => Some(Loop),
        "break" => Some(Break),
        "continue" => Some(Continue),
        "let" => Some(Let),
        "mut" => Some(Mut),
        "true" => Some(BoolT { value: true }),
        "false" => Some(BoolT { value:false }),

        "DBG" => Some(DBG),
        _ => None
    }
}

fn hex_to_decimal(hex_string: &str) -> Result<String, std::num::ParseIntError> {
    // Parse the hexadecimal string into an integer
    let hex_int = u128::from_str_radix(hex_string, 16)?;

    // Convert the integer to a decimal string
    let decimal_string = hex_int.to_string();

    Ok(decimal_string)
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
                    } else if self.char_match('>') {
                        MinusGreater
                    } else {
                        Minus
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '^' => {
                let token = {
                    Caret
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
            '%' => {
                let token = {
                    if self.char_match('=') {
                        PersentEqual
                    } else {
                        Persent
                    }
                };
                self.add_token(token, (self.line, pos_start, self.get_pos()));
            }
            '.' => {
                let token = {
                    if self.char_match('.') {
                        DotDot
                    } else {
                        Dot
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
                    Question
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
                    if c == '0' && self.char_match('x') {
                        match self.hex_number() {
                            Ok(_) => (),
                            Err(msg) => return Err(msg)
                        }
                    } else {
                        match self.number() {
                            Ok(_) => (),
                            Err(msg) => return Err(msg)
                        }
                    }
                } else if is_idetifier_char_start(c) {
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
        
        loop {
            buffer.push(self.current);

            if self.is_at_end() {
                break;
            }

            if is_idetifier_char(self.next) {
                self.advance();
            } else {
                break;
            }
        }
        
        if buffer == "null".to_string() {
            self.add_token(Null, (self.line, pos_start, self.get_pos()));
        } else {
            match str_to_keyword(&buffer) {
                Some(token_type) => {
                    self.add_token(token_type, (self.line, pos_start, self.get_pos()));
                }
                _ => {
                    self.add_token(Identifier { value: buffer }, (self.line, pos_start, self.get_pos()));
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
                        let mut u_buffer = String::new();
                        while self.next.is_digit(16) {
                            u_buffer.push(self.next);
                            self.advance();
                        }
                        buffer.push(match char::from_u32(match u32::from_str_radix(&u_buffer, 16) {
                            Ok(value) => value,
                            Err(_) => return Err("Too big hex number".to_string())
                        }) {
                            Some(c) => c,
                            None => return Err("Can't convert unicode to char".to_string())
                        });
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

        self.add_token(StringT { value: buffer }, (self.line, pos_start, self.get_pos()));

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
                        let mut u_buffer = String::new();
                        while self.next.is_digit(16) {
                            u_buffer.push(self.next);
                            self.advance();
                        }
                        result = match char::from_u32(match u32::from_str_radix(&u_buffer, 16) {
                            Ok(value) => value,
                            Err(_) => return Err("Too big hex number".to_string())
                        }) {
                            Some(c) => c,
                            None => return Err("Can't convert unicode to char".to_string())
                        };
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

        self.add_token(Char { value: result }, (self.line, pos_start, self.get_pos()));

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

        let num_type = {
            if with_dot {
                if self.char_match('f') || self.char_match('F') {
                    if self.char_match('3') && self.char_match('2') {
                        NumberType::F32

                    } else if self.char_match('6') && self.char_match('4') {
                        NumberType::F64

                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unrecognized number type at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                    }
                } else {
                    NumberType::UntypedFloat
                }
            } else {
                if self.char_match('i') || self.char_match('I') {                    
                    if self.char_match('8') {
                        NumberType::I8
                        
                    } else if self.char_match('1') {
                        if self.char_match('6') {
                            NumberType::I16
                            
                        } else if self.char_match('2') && self.char_match('8') {
                            NumberType::I128

                        } else {
                            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                            return Err(format!("Unknown number type at  [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        }
                        
                    } else if self.char_match('3') && self.char_match('2') {
                        NumberType::I32
                        
                    } else if self.char_match('6') && self.char_match('4') {
                        NumberType::I64
                        
                    } else {
                        NumberType::ISize

                    }
                } else if self.char_match('u') || self.char_match('U') {

                    if self.char_match('8') {
                        NumberType::U8

                    } else if self.char_match('1') {
                        if self.char_match('6') {
                            NumberType::U16

                        } else if self.char_match('2') && self.char_match('8') {
                            NumberType::U128

                        } else {
                            let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                            return Err(format!("Unknown number type at  [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                        }
                        } else if self.char_match('3') && self.char_match('2') {
                            NumberType::U32

                    } else if self.char_match('6') && self.char_match('4') {
                        NumberType::U64

                    } else {
                        NumberType::USize

                    }
                } else if self.char_match('f') || self.char_match('F') {
                    if self.char_match('3') && self.char_match('2') {
                        NumberType::F32

                    } else if self.char_match('6') && self.char_match('4') {
                        NumberType::F64

                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unrecognized number type at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                    }
                } else {
                    NumberType::UntypedInt
                }
            }
        };
        if num_type == NumberType::F32 || num_type == NumberType::F64 || num_type == NumberType::UntypedFloat {
            self.add_token(Float { value: buffer, num_type: num_type }, (self.line, pos_start, self.get_pos()));
        } else {
            self.add_token(Int { value: buffer, num_type: num_type }, (self.line, pos_start, self.get_pos()));
        }
        Ok(())
    }

    fn hex_number(&mut self) -> Result<(), String> {
        let pos_start = self.get_pos() - 2;

        let mut buffer = String::new();

        while is_hex(self.next) {
            self.advance();
            if self.current != '_' {
                buffer.push(self.current);
            }
        }

        let result = {
            match hex_to_decimal(&buffer) {
                Ok(res) => res,
                Err(msg) => return Err(msg.to_string())
            }
        };
        
        let num_type = {
            if self.char_match('i') || self.char_match('I') {                    
                if self.char_match('8') {
                    NumberType::I8
                    
                } else if self.char_match('1') {
                    if self.char_match('6') {
                        NumberType::I16
                        
                    } else if self.char_match('2') && self.char_match('8') {
                        NumberType::I128

                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unknown number type at  [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    }
                    
                } else if self.char_match('3') && self.char_match('2') {
                    NumberType::I32
                    
                } else if self.char_match('6') && self.char_match('4') {
                    NumberType::I64
                    
                } else {
                    NumberType::ISize

                }
            } else if self.char_match('u') || self.char_match('U') {

                if self.char_match('8') {
                    NumberType::U8

                } else if self.char_match('1') {
                    if self.char_match('6') {
                        NumberType::U16

                    } else if self.char_match('2') && self.char_match('8') {
                        NumberType::U128

                    } else {
                        let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                        return Err(format!("Unknown number type at  [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme))
                    }
                    } else if self.char_match('3') && self.char_match('2') {
                        NumberType::U32

                } else if self.char_match('6') && self.char_match('4') {
                    NumberType::U64

                } else {
                    NumberType::USize

                }
            } else if self.char_match('f') || self.char_match('F') {
                if self.char_match('3') && self.char_match('2') {
                    NumberType::F32

                } else if self.char_match('6') && self.char_match('4') {
                    NumberType::F64

                } else {
                    let lexeme = self.get_lexeme((self.line, pos_start, self.current_pos));
                    return Err(format!("Unrecognized number type at possition [{}:{}:{}]: {}", self.file_path, self.line, self.col, lexeme));
                }
            } else {
                NumberType::UntypedInt
            }
        };
        if num_type == NumberType::F32 || num_type == NumberType::F64 || num_type == NumberType::UntypedFloat {
            self.add_token(Float { value: result, num_type: num_type }, (self.line, pos_start, self.get_pos()));
        } else {
            self.add_token(Int { value: result, num_type: num_type }, (self.line, pos_start, self.get_pos()));
        }
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
        let lexeme = self.get_lexeme(possition);
        self.tokens.push(Token::new(token_type, lexeme, possition));
    }
}
