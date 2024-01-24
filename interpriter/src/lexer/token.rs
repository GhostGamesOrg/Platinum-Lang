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
    String,
    Float,
    Integer,
    Bool,

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

    True,
    False,

    Fun,
    Return,
    
    For,
    While,
    DoWhile,
    Loop,
    Break,
    Continue,

    Nil,
    Let,

    /// End Of File
    EOF
}


#[derive(Debug, Clone)]
pub enum LiteralValue {
    // I8Value(i8),
    // I16Value(i16),
    // I32Value(i32),
    // I64Value(i64),
    // I128Value(i128),
    // ISizeValue(isize),

    // U8Value(u8),
    // U16Value(u16),
    // U32Value(u32),
    // U64Value(u64),
    // U128Value(u128),
    // USizeValue(usize),

    // F32Value(f32),
    // F64Value(f64),

    // UndefinedIntValue(i128),
    // UndefinedFloatValue(f64),

    IntValue(i128),
    FloatValue(f64),

    StringValue(String),
    IdentifierValue(String),
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone)]
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