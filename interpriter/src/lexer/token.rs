pub type Possition = (usize, usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // [
    RightBrace,     // ]
    LeftCurBrace,   // {
    RightCurBrace,  // }
    Comma,          // ,
    Dot,            // .
    Semicolon,      // ;
    Colon,          // :
    
    Minus,          // -
    Plus,           // +
    Slash,          // /
    Star,           // *
    Equal,          // =
    
    MinusMinus,     // --
    PlusPlus,       // ++

    MinusEqual,     // -=
    PlusEqual,      // +=
    SlashEqual,     // /=
    StarEqual,      // *=

    EqualEqual,     // ==
    Bang,           // !
    BangEqual,      // !=
    Greater,        // >
    GreaterEqual,   // >=
    Less,           // <
    LessEqual,      // <=

    GreaterGreater,         // >>
    GreaterGreaterEqual,    // >>=
    GreaterGreaterGreater,  // >>>
    LessLess,               // <<
    LessLessEqual,          // <<=
    Tilde,                  // ~

    Question,           // ?
    QuestionQuestion,   // ??
    Ampersant,      // &
    Bar,            // |
    

    // Literals.
    Identifier,
    StringT,
    Char,
    Number,
    BoolT,

    DocComent,
    Coment,

    // Keywords.
    And,    // and  &&
    Or,     // or   ||

    If,
    Else,

    Class,
    Super,
    This,

    Fun,
    Return,
    
    For,
    While,
    DoWhile,
    Loop,
    Break,
    Continue,

    Null,
    Let,

    /// End Of File
    EOF
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LiteralValue {
    Bool(bool),
    I8Value(i8),              // 100i8
    I16Value(i16),            // 100i16
    I32Value(i32),            // 100i32
    I64Value(i64),            // 100i64
    I128Value(i128),          // 100i128
    ISizeValue(isize),        // 100i

    U8Value(u8),              // 100u8
    U16Value(u16),            // 100u16
    U32Value(u32),            // 100u32
    U64Value(u64),            // 100u64
    U128Value(u128),          // 100u128
    USizeValue(usize),        // 100u

    F32Value(f32),            // 10f32
    F64Value(f64),            // 10f64

    UndefinedIntValue(i128),  // 100
    UndefinedFloatValue(f64), // 10.0

    StringValue(String),
    CharValue(char),
    IdentifierValue(String),
    NullLiteral(bool),
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Bool(value) => value.to_string(),

            LiteralValue::I8Value(value) => format!("{}i8", value.to_string()),
            LiteralValue::I16Value(value) => format!("{}i16", value.to_string()),
            LiteralValue::I32Value(value) => format!("{}i32", value.to_string()),
            LiteralValue::I64Value(value) => format!("{}i64", value.to_string()),
            LiteralValue::I128Value(value) => format!("{}i128", value.to_string()),
            LiteralValue::ISizeValue(value) => format!("{}i", value.to_string()),

            LiteralValue::U8Value(value) => format!("{}u8", value.to_string()),
            LiteralValue::U16Value(value) => format!("{}u16", value.to_string()),
            LiteralValue::U32Value(value) => format!("{}u32", value.to_string()),
            LiteralValue::U64Value(value) => format!("{}u64", value.to_string()),
            LiteralValue::U128Value(value) => format!("{}u128", value.to_string()),
            LiteralValue::USizeValue(value) => format!("{}u", value.to_string()),

            LiteralValue::F32Value(value) => format!("{}f32", value.to_string()),
            LiteralValue::F64Value(value) => format!("{}f64", value.to_string()),

            LiteralValue::UndefinedIntValue(value) => value.to_string(),
            LiteralValue::UndefinedFloatValue(value) => value.to_string(),
            
            LiteralValue::StringValue(value) => format!("\"{}\"", value),
            LiteralValue::CharValue(value) => format!("'{}'", value),
            LiteralValue::IdentifierValue(value) => value.clone(),
            LiteralValue::NullLiteral(_) => "null".to_string(),
        }
    }

    pub fn get_type(&self) -> String {
        let result = match self {
            LiteralValue::Bool(_) => "boolean",

            LiteralValue::I8Value(_) => "i8",
            LiteralValue::I16Value(_) => "i16",
            LiteralValue::I32Value(_) => "i32",
            LiteralValue::I64Value(_) => "i64",
            LiteralValue::I128Value(_) => "i128",
            LiteralValue::ISizeValue(_) => "i",

            LiteralValue::U8Value(_) => "u8",
            LiteralValue::U16Value(_) => "u16",
            LiteralValue::U32Value(_) => "u32",
            LiteralValue::U64Value(_) => "u64",
            LiteralValue::U128Value(_) => "u128",
            LiteralValue::USizeValue(_) => "u",

            LiteralValue::F32Value(_) => "f32",
            LiteralValue::F64Value(_) => "f64",

            LiteralValue::UndefinedIntValue(_) => "int",
            LiteralValue::UndefinedFloatValue(_) => "float",
            
            LiteralValue::StringValue(_) => "String",
            LiteralValue::CharValue(_) => "char",
            LiteralValue::IdentifierValue(_) => "Identifier",
            LiteralValue::NullLiteral(_) => "null",
        };
        result.to_string()
    }
    
    pub fn is_same_type(&self, other: &LiteralValue) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    pub fn match_literal_types(&self, allowed_literals: Vec<LiteralValue>) -> bool {
        allowed_literals.iter().any(|allowed| self.is_same_type(allowed))
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub possition: Possition
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        possition: Possition
    ) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            possition: possition
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

/*

let test = 0.1;
let test2 = test + 0.1;

*/