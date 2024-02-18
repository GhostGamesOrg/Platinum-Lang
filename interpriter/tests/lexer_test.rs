use std::{self, fs};

use interpriter::lexer::{lexer::*, token::{NumberType, TokenType::{self, *}}};


fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let src: String = fs::read_to_string(path)?.parse()?;
    Ok(src)
}

#[test]
fn handle_one_char_tokens() {
    let file_path = "<stdin>";
    let src = "() [] {} , . ; : ~";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 12);
    assert_eq!(scanner.tokens[0].lexeme, "(".to_string());
    assert_eq!(scanner.tokens[1].lexeme, ")".to_string());
    assert_eq!(scanner.tokens[2].lexeme, "[".to_string());
    assert_eq!(scanner.tokens[3].lexeme, "]".to_string());
    assert_eq!(scanner.tokens[4].lexeme, "{".to_string());
    assert_eq!(scanner.tokens[5].lexeme, "}".to_string());
    assert_eq!(scanner.tokens[6].lexeme, ",".to_string());
    assert_eq!(scanner.tokens[7].lexeme, ".".to_string());
    assert_eq!(scanner.tokens[8].lexeme, ";".to_string());
    assert_eq!(scanner.tokens[9].lexeme, ":".to_string());
    assert_eq!(scanner.tokens[10].lexeme, "~".to_string());
    
    assert_eq!(scanner.tokens[0].token_type, LeftParen);
    assert_eq!(scanner.tokens[1].token_type, RightParen);
    assert_eq!(scanner.tokens[2].token_type, LeftBrace);
    assert_eq!(scanner.tokens[3].token_type, RightBrace);
    assert_eq!(scanner.tokens[4].token_type, LeftCurBrace);
    assert_eq!(scanner.tokens[5].token_type, RightCurBrace);
    assert_eq!(scanner.tokens[6].token_type, Comma);
    assert_eq!(scanner.tokens[7].token_type, Dot);
    assert_eq!(scanner.tokens[8].token_type, Semicolon);
    assert_eq!(scanner.tokens[9].token_type, Colon);
    assert_eq!(scanner.tokens[10].token_type, Tilde);
    assert_eq!(scanner.tokens[11].token_type, EOF);
}

#[test]
fn handle_multi_char_tokens() {
    let file_path = "<stdin>";
    let src = "-- ++ -= += /= *= == != >= <= >> >>= << <<= ?? && ||";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 18);
    assert_eq!(scanner.tokens[0].lexeme, "--".to_string());
    assert_eq!(scanner.tokens[1].lexeme, "++".to_string());
    assert_eq!(scanner.tokens[2].lexeme, "-=".to_string());
    assert_eq!(scanner.tokens[3].lexeme, "+=".to_string());
    assert_eq!(scanner.tokens[4].lexeme, "/=".to_string());
    assert_eq!(scanner.tokens[5].lexeme, "*=".to_string());
    assert_eq!(scanner.tokens[6].lexeme, "==".to_string());
    assert_eq!(scanner.tokens[7].lexeme, "!=".to_string());
    assert_eq!(scanner.tokens[8].lexeme, ">=".to_string());
    assert_eq!(scanner.tokens[9].lexeme, "<=".to_string());
    assert_eq!(scanner.tokens[10].lexeme, ">>".to_string());
    assert_eq!(scanner.tokens[11].lexeme, ">>=".to_string());
    assert_eq!(scanner.tokens[12].lexeme, "<<".to_string());
    assert_eq!(scanner.tokens[13].lexeme, "<<=".to_string());
    assert_eq!(scanner.tokens[14].lexeme, "??".to_string());
    assert_eq!(scanner.tokens[15].lexeme, "&&".to_string());
    assert_eq!(scanner.tokens[16].lexeme, "||".to_string());

    assert_eq!(scanner.tokens[0].token_type, MinusMinus);
    assert_eq!(scanner.tokens[1].token_type, PlusPlus);
    assert_eq!(scanner.tokens[2].token_type, MinusEqual);
    assert_eq!(scanner.tokens[3].token_type, PlusEqual);
    assert_eq!(scanner.tokens[4].token_type, SlashEqual);
    assert_eq!(scanner.tokens[5].token_type, StarEqual);
    assert_eq!(scanner.tokens[6].token_type, EqualEqual);
    assert_eq!(scanner.tokens[7].token_type, BangEqual);
    assert_eq!(scanner.tokens[8].token_type, GreaterEqual);
    assert_eq!(scanner.tokens[9].token_type, LessEqual);
    assert_eq!(scanner.tokens[10].token_type, GreaterGreater);
    assert_eq!(scanner.tokens[11].token_type, GreaterGreaterEqual);
    assert_eq!(scanner.tokens[12].token_type, LessLess);
    assert_eq!(scanner.tokens[13].token_type, LessLessEqual);
    assert_eq!(scanner.tokens[14].token_type, QuestionQuestion);
    assert_eq!(scanner.tokens[15].token_type, And);
    assert_eq!(scanner.tokens[16].token_type, Or);
    assert_eq!(scanner.tokens[17].token_type, EOF);
}

#[test]
fn handle_comments_tokens() {
    let file_path = "tests\\lexer_codes\\handle_comments_tokens.ppl";
    let src: String = read_file(file_path).unwrap();

    let mut scanner = Scanner::new(file_path, src.as_str());
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 7);
    assert_eq!(scanner.tokens[0].lexeme, "// this is simple comment".to_string());
    assert_eq!(scanner.tokens[1].lexeme, "(".to_string());
    assert_eq!(scanner.tokens[2].lexeme, ")".to_string());
    assert_eq!(scanner.tokens[3].lexeme, "(".to_string());
    assert_eq!(scanner.tokens[4].lexeme, ")".to_string());
    assert_eq!(scanner.tokens[5].lexeme, "/// this is doc comment".to_string());

    assert_eq!(scanner.tokens[0].token_type, Coment);
    assert_eq!(scanner.tokens[1].token_type, LeftParen);
    assert_eq!(scanner.tokens[2].token_type, RightParen);
    assert_eq!(scanner.tokens[3].token_type, LeftParen);
    assert_eq!(scanner.tokens[4].token_type, RightParen);
    assert_eq!(scanner.tokens[5].token_type, DocComent);
    assert_eq!(scanner.tokens[6].token_type, EOF);
}

#[test]
fn handle_string_tokens() {
    let file_path = "<stdin>";
    let src = "\"Hello\"";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].lexeme, "\"Hello\"".to_string());
    assert_eq!(scanner.tokens[0].token_type, StringT { value: "Hello".to_string() });
    assert_eq!(scanner.tokens[1].token_type, EOF);
}

#[test]
fn string_unterminated_error() {
    let file_path = "<stdin>";
    let src = "\"Hello";
    let mut scanner = Scanner::new(file_path, src);
    match scanner.scan_tokens() {
        Err(msg) => {
            assert_eq!(msg, "Unterminated string at possition [<stdin>:1:7]: \"Hello\n".to_string());
        },
        Ok(_) => ()
    }
}

#[test]
fn string_unexpected_char_error() {
    let file_path = "<stdin>";
    let src = "\"\\w\"";
    let mut scanner = Scanner::new(file_path, src);
    match scanner.scan_tokens() {
        Err(msg) => {
            assert_eq!(msg, "Unexpected charrecter at possition [<stdin>:1:4]: \"\\w\n".to_string());
        },
        Ok(_) => ()
    }
}

#[test]
fn handle_char_tokens() {
    let file_path = "<stdin>";
    let src = "'A'";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].lexeme, "'A'".to_string());
    assert_eq!(scanner.tokens[0].token_type, Char { value: 'A' });
    assert_eq!(scanner.tokens[1].token_type, EOF);
}

#[test]
fn handle_special_chars_tokens() {
    let file_path = "tests\\lexer_codes\\handle_special_chars_tokens.ppl";
    let src: String = read_file(file_path).unwrap();

    let mut scanner = Scanner::new(file_path, src.as_str());
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 9);

    assert_eq!(scanner.tokens[0].lexeme, "'A'".to_string());
    assert_eq!(scanner.tokens[1].lexeme, "// Тут будет просто 'A'".to_string());
    assert_eq!(scanner.tokens[2].lexeme, "'\\t'".to_string());
    assert_eq!(scanner.tokens[3].lexeme, "// Тут уже будет '\\t'".to_string());
    assert_eq!(scanner.tokens[4].lexeme, "'\\n'".to_string());
    assert_eq!(scanner.tokens[5].lexeme, "// Тут уже будет '\\n'".to_string());
    assert_eq!(scanner.tokens[6].lexeme, "'Ф'".to_string());
    assert_eq!(scanner.tokens[7].lexeme, "// Тут будет просто 'Ф'".to_string());

    assert_eq!(scanner.tokens[0].token_type, Char { value: 'A' });
    assert_eq!(scanner.tokens[1].token_type, Coment);
    assert_eq!(scanner.tokens[2].token_type, Char { value: '\t' });
    assert_eq!(scanner.tokens[3].token_type, Coment);
    assert_eq!(scanner.tokens[4].token_type, Char { value: '\n' });
    assert_eq!(scanner.tokens[5].token_type, Coment);
    assert_eq!(scanner.tokens[6].token_type, Char { value: 'Ф' });
    assert_eq!(scanner.tokens[7].token_type, Coment);
    assert_eq!(scanner.tokens[8].token_type, EOF);
}

#[test]
fn char_unterminated_error() {
    let file_path = "<stdin>";
    let src = "'A";
    let mut scanner = Scanner::new(file_path, src);
    match scanner.scan_tokens() {
        Err(msg) => {
            assert_eq!(msg, "Unterminated char at possition [<stdin>:1:3]: 'A\n".to_string());
        },
        Ok(_) => ()
    }
}

#[test]
fn char_unexpected_char_error() {
    let file_path = "<stdin>";
    let src = "'\\w'";
    let mut scanner = Scanner::new(file_path, src);
    match scanner.scan_tokens() {
        Err(msg) => {
            assert_eq!(msg, "Unexpected charrecter at possition [<stdin>:1:4]: '\\w\n".to_string());
        },
        Ok(_) => ()
    }
}

#[test]
fn handle_number_token() {
    let file_path = "<stdin>";
    let src = "100";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::UntypedInt });
    assert_eq!(scanner.tokens[1].token_type, EOF);
}

#[test]
fn handle_number_with_underscore_token() {
    let file_path = "<stdin>";
    let src = "1_000_000";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Int { value: "1000000".to_string(), num_type: NumberType::UntypedInt });
    assert_eq!(scanner.tokens[1].token_type, EOF);
}

#[test]
fn handle_numbers_tokens() {
    let file_path = "tests\\lexer_codes\\handle_number_tokens.ppl";
    let src = read_file(file_path).unwrap();
    let mut scanner = Scanner::new(file_path, src.as_str());
    let _ = scanner.scan_tokens();

    for token in scanner.tokens.iter() {
        println!("{:?}", token);
    }
    
    assert_eq!(scanner.tokens.len(), 19);
    assert_eq!(scanner.tokens[0].lexeme, "100".to_string());
    assert_eq!(scanner.tokens[1].lexeme, "100.0".to_string());
    assert_eq!(scanner.tokens[2].lexeme, "100f32".to_string());
    assert_eq!(scanner.tokens[3].lexeme, "100f64".to_string());
    assert_eq!(scanner.tokens[4].lexeme, "100.0f32".to_string());
    assert_eq!(scanner.tokens[5].lexeme, "100.0f64".to_string());
    assert_eq!(scanner.tokens[6].lexeme, "100i8".to_string());
    assert_eq!(scanner.tokens[7].lexeme, "100i16".to_string());
    assert_eq!(scanner.tokens[8].lexeme, "100i32".to_string());
    assert_eq!(scanner.tokens[9].lexeme, "100i64".to_string());
    assert_eq!(scanner.tokens[10].lexeme, "100i128".to_string());
    assert_eq!(scanner.tokens[11].lexeme, "100i".to_string());
    assert_eq!(scanner.tokens[12].lexeme, "100u8".to_string());
    assert_eq!(scanner.tokens[13].lexeme, "100u16".to_string());
    assert_eq!(scanner.tokens[14].lexeme, "100u32".to_string());
    assert_eq!(scanner.tokens[15].lexeme, "100u64".to_string());
    assert_eq!(scanner.tokens[16].lexeme, "100u128".to_string());
    assert_eq!(scanner.tokens[17].lexeme, "100u".to_string());

    assert_eq!(scanner.tokens[0].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::UntypedInt });
    assert_eq!(scanner.tokens[1].token_type, TokenType::Float { value: "100.0".to_string(), num_type: NumberType::UntypedFloat });
    assert_eq!(scanner.tokens[2].token_type, TokenType::Float { value: "100".to_string(), num_type: NumberType::F32 });
    assert_eq!(scanner.tokens[3].token_type, TokenType::Float { value: "100".to_string(), num_type: NumberType::F64 });
    assert_eq!(scanner.tokens[4].token_type, TokenType::Float { value: "100.0".to_string(), num_type: NumberType::F32 });
    assert_eq!(scanner.tokens[5].token_type, TokenType::Float { value: "100.0".to_string(), num_type: NumberType::F64 });
    assert_eq!(scanner.tokens[6].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::I8 });
    assert_eq!(scanner.tokens[7].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::I16 });
    assert_eq!(scanner.tokens[8].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::I32 });
    assert_eq!(scanner.tokens[9].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::I64 });
    assert_eq!(scanner.tokens[10].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::I128 });
    assert_eq!(scanner.tokens[11].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::ISize });
    assert_eq!(scanner.tokens[12].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::U8 });
    assert_eq!(scanner.tokens[13].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::U16 });
    assert_eq!(scanner.tokens[14].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::U32 });
    assert_eq!(scanner.tokens[15].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::U64 });
    assert_eq!(scanner.tokens[16].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::U128 });
    assert_eq!(scanner.tokens[17].token_type, TokenType::Int { value: "100".to_string(), num_type: NumberType::USize });

    assert_eq!(scanner.tokens[18].token_type, EOF);
}

#[test]
fn handle_hex_number_token() {
    let file_path = "<stdin>";
    let src = "0xFF";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].token_type, Int { value: "255".to_string(), num_type: NumberType::UntypedInt });
    assert_eq!(scanner.tokens[1].token_type, EOF);
}

#[test]
fn handle_idetifier_token() {
    let file_path = "<stdin>";
    let src = "hello";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].token_type, Identifier { value: "hello".to_string() });
    assert_eq!(scanner.tokens[1].token_type, EOF);
}

#[test]
fn handle_underscore_idetifier_token() {
    let file_path = "<stdin>";
    let src = "_";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].token_type, Identifier { value: "_".to_string() });
    assert_eq!(scanner.tokens[1].token_type, EOF);
}

#[test]
fn handle_not_ascii_idetifier_token() {
    let file_path = "<stdin>";
    let src = "привет";
    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    for token in scanner.tokens.iter() {
        println!("{:?}", token);
    }

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].token_type, Identifier { value: "привет".to_string() });
    assert_eq!(scanner.tokens[1].token_type, EOF);
}

#[test]
fn handle_standart_idetifiers_token() {
    let file_path = "tests\\lexer_codes\\handle_standart_idetifiers_token.ppl";
    let src = read_file(file_path).unwrap();
    let mut scanner = Scanner::new(file_path, src.as_str());
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 21);

    assert_eq!(scanner.tokens[0].lexeme, "and".to_string());
    assert_eq!(scanner.tokens[1].lexeme, "or".to_string());
    assert_eq!(scanner.tokens[2].lexeme, "if".to_string());
    assert_eq!(scanner.tokens[3].lexeme, "else".to_string());
    assert_eq!(scanner.tokens[4].lexeme, "class".to_string());
    assert_eq!(scanner.tokens[5].lexeme, "super".to_string());
    assert_eq!(scanner.tokens[6].lexeme, "this".to_string());
    assert_eq!(scanner.tokens[7].lexeme, "fun".to_string());
    assert_eq!(scanner.tokens[8].lexeme, "return".to_string());
    assert_eq!(scanner.tokens[9].lexeme, "for".to_string());
    assert_eq!(scanner.tokens[10].lexeme, "while".to_string());
    assert_eq!(scanner.tokens[11].lexeme, "do".to_string());
    assert_eq!(scanner.tokens[12].lexeme, "loop".to_string());
    assert_eq!(scanner.tokens[13].lexeme, "break".to_string());
    assert_eq!(scanner.tokens[14].lexeme, "continue".to_string());
    assert_eq!(scanner.tokens[15].lexeme, "let".to_string());
    assert_eq!(scanner.tokens[16].lexeme, "mut".to_string());
    assert_eq!(scanner.tokens[17].lexeme, "null".to_string());
    assert_eq!(scanner.tokens[18].lexeme, "true".to_string());
    assert_eq!(scanner.tokens[19].lexeme, "false".to_string());
    
    assert_eq!(scanner.tokens[0].token_type, And);
    assert_eq!(scanner.tokens[1].token_type, Or);
    assert_eq!(scanner.tokens[2].token_type, If);
    assert_eq!(scanner.tokens[3].token_type, Else);
    assert_eq!(scanner.tokens[4].token_type, Class);
    assert_eq!(scanner.tokens[5].token_type, Super);
    assert_eq!(scanner.tokens[6].token_type, This);
    assert_eq!(scanner.tokens[7].token_type, Fun);
    assert_eq!(scanner.tokens[8].token_type, Return);
    assert_eq!(scanner.tokens[9].token_type, For);
    assert_eq!(scanner.tokens[10].token_type, While);
    assert_eq!(scanner.tokens[11].token_type, DoWhile);
    assert_eq!(scanner.tokens[12].token_type, Loop);
    assert_eq!(scanner.tokens[13].token_type, Break);
    assert_eq!(scanner.tokens[14].token_type, Continue);
    assert_eq!(scanner.tokens[15].token_type, Let);
    assert_eq!(scanner.tokens[16].token_type, Mut);
    assert_eq!(scanner.tokens[17].token_type, Null);
    assert_eq!(scanner.tokens[18].token_type, BoolT { value: true });
    assert_eq!(scanner.tokens[19].token_type, BoolT { value: false });

    assert_eq!(scanner.tokens[20].token_type, EOF);
}

// for token in scanner.tokens.iter() {
//     println!("{:?}", token);
// }