pub type Possition = (usize, usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberType {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,

    U8,
    U16,
    U32,
    U64,
    U128,
    USize,

    F32,
    F64,

    UntypedInt,
    UntypedFloat,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // [
    RightBrace,     // ]
    LeftCurBrace,   // {
    RightCurBrace,  // }
    Comma,          // ,
    Dot,            // .
    DotDot,         // ..
    Semicolon,      // ;
    Colon,          // :
    
    Minus,          // -
    Plus,           // +
    Slash,          // /
    Star,           // *
    Persent,        // *
    Equal,          // =
    
    MinusMinus,     // --
    PlusPlus,       // ++

    MinusEqual,     // -=
    PlusEqual,      // +=
    SlashEqual,     // /=
    StarEqual,      // *=
    PersentEqual,   // *=

    EqualEqual,     // ==
    Bang,           // !
    BangEqual,      // !=
    Greater,        // >
    GreaterEqual,   // >=
    Less,           // <
    LessEqual,      // <=

    GreaterGreater,         // >>
    GreaterGreaterEqual,    // >>=
    LessLess,               // <<
    LessLessEqual,          // <<=
    Tilde,                  // ~

    Question,           // ?
    QuestionQuestion,   // ??
    Ampersant,      // &
    Bar,            // |
    Caret,          // ^

    
    MinusGreater,   // ->
    

    // Literals.
    Identifier { value: String },
    StringT { value: String },
    Char { value: char },
    Int { value: String, num_type: NumberType },
    Float { value: String, num_type: NumberType },
    BoolT { value: bool },

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
    In,
    Range,
    While,
    DoWhile,
    Loop,
    Break,
    Continue,

    Null,
    Let,
    Mut,
    
    DBG, // Turn of on release
    
    /// End Of File
    EOF
}

impl TokenType {
    pub fn eq_token(&self, token: TokenType) -> bool {
        match (self, token.clone()) {
            (TokenType::Identifier {..}, TokenType::Identifier {..}) |
            (TokenType::StringT {..}, TokenType::StringT {..}) |
            (TokenType::Char {..}, TokenType::Char {..}) |
            (TokenType::Int {..}, TokenType::Int {..}) |
            (TokenType::Float {..}, TokenType::Float {..}) |
            (TokenType::BoolT {..}, TokenType::BoolT {..}) => return true,
            _ => {
                return self.eq(&token);
            }
        }
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
    pub possition: Possition
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        possition: Possition
    ) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            possition: possition
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.token_type, self.lexeme)
    }
}

/*

let test = 0.1;
let test2 = test + 0.1;

*/