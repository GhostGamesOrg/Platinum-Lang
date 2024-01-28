use std::{self, fs};

use interpriter::lexer::{lexer::*, token::{TokenType, LiteralValue}};


fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let src: String = fs::read_to_string(path)?.parse()?;
    Ok(src)
}

#[test]
fn handle_one_char_tokens() {
    let src = "() [] {} , . ; : ~";
    let mut scanner = Scanner::new(src);
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
    
    assert_eq!(scanner.tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(scanner.tokens[1].token_type, TokenType::RightParen);
    assert_eq!(scanner.tokens[2].token_type, TokenType::LeftBrace);
    assert_eq!(scanner.tokens[3].token_type, TokenType::RightBrace);
    assert_eq!(scanner.tokens[4].token_type, TokenType::LeftCurBrace);
    assert_eq!(scanner.tokens[5].token_type, TokenType::RightCurBrace);
    assert_eq!(scanner.tokens[6].token_type, TokenType::Comma);
    assert_eq!(scanner.tokens[7].token_type, TokenType::Dot);
    assert_eq!(scanner.tokens[8].token_type, TokenType::Semicolon);
    assert_eq!(scanner.tokens[9].token_type, TokenType::Colon);
    assert_eq!(scanner.tokens[10].token_type, TokenType::Tilde);
    assert_eq!(scanner.tokens[11].token_type, TokenType::EOF);
}

#[test]
fn handle_multi_char_tokens() {
    let src = "-- ++ -= += /= *= == != >= <= >> >>= >>> << <<= ?? && ||";
    let mut scanner = Scanner::new(src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 19);
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
    assert_eq!(scanner.tokens[12].lexeme, ">>>".to_string());
    assert_eq!(scanner.tokens[13].lexeme, "<<".to_string());
    assert_eq!(scanner.tokens[14].lexeme, "<<=".to_string());
    assert_eq!(scanner.tokens[15].lexeme, "??".to_string());
    assert_eq!(scanner.tokens[16].lexeme, "&&".to_string());
    assert_eq!(scanner.tokens[17].lexeme, "||".to_string());

    assert_eq!(scanner.tokens[0].token_type, TokenType::MinusMinus);
    assert_eq!(scanner.tokens[1].token_type, TokenType::PlusPlus);
    assert_eq!(scanner.tokens[2].token_type, TokenType::MinusEqual);
    assert_eq!(scanner.tokens[3].token_type, TokenType::PlusEqual);
    assert_eq!(scanner.tokens[4].token_type, TokenType::SlashEqual);
    assert_eq!(scanner.tokens[5].token_type, TokenType::StarEqual);
    assert_eq!(scanner.tokens[6].token_type, TokenType::EqualEqual);
    assert_eq!(scanner.tokens[7].token_type, TokenType::BangEqual);
    assert_eq!(scanner.tokens[8].token_type, TokenType::GreaterEqual);
    assert_eq!(scanner.tokens[9].token_type, TokenType::LessEqual);
    assert_eq!(scanner.tokens[10].token_type, TokenType::GreaterGreater);
    assert_eq!(scanner.tokens[11].token_type, TokenType::GreaterGreaterEqual);
    assert_eq!(scanner.tokens[12].token_type, TokenType::GreaterGreaterGreater);
    assert_eq!(scanner.tokens[13].token_type, TokenType::LessLess);
    assert_eq!(scanner.tokens[14].token_type, TokenType::LessLessEqual);
    assert_eq!(scanner.tokens[15].token_type, TokenType::QuestionQuestion);
    assert_eq!(scanner.tokens[16].token_type, TokenType::And);
    assert_eq!(scanner.tokens[17].token_type, TokenType::Or);
    assert_eq!(scanner.tokens[18].token_type, TokenType::EOF);
}

#[test]
fn handle_comments_tokens() {
    let src: String = read_file("tests\\codes\\handle_comments_tokens.ppl").unwrap();

    let mut scanner = Scanner::new(src.as_str());
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 7);
    assert_eq!(scanner.tokens[0].lexeme, "// this is simple comment".to_string());
    assert_eq!(scanner.tokens[1].lexeme, "(".to_string());
    assert_eq!(scanner.tokens[2].lexeme, ")".to_string());
    assert_eq!(scanner.tokens[3].lexeme, "(".to_string());
    assert_eq!(scanner.tokens[4].lexeme, ")".to_string());
    assert_eq!(scanner.tokens[5].lexeme, "/// this is doc comment".to_string());

    assert_eq!(scanner.tokens[0].token_type, TokenType::Coment);
    assert_eq!(scanner.tokens[1].token_type, TokenType::LeftParen);
    assert_eq!(scanner.tokens[2].token_type, TokenType::RightParen);
    assert_eq!(scanner.tokens[3].token_type, TokenType::LeftParen);
    assert_eq!(scanner.tokens[4].token_type, TokenType::RightParen);
    assert_eq!(scanner.tokens[5].token_type, TokenType::DocComent);
    assert_eq!(scanner.tokens[6].token_type, TokenType::EOF);
}

#[test]
fn handle_string_tokens() {
    let src = "\"Hello\"";
    let mut scanner = Scanner::new(src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::StringValue("Hello".to_string())));
    assert_eq!(scanner.tokens[0].lexeme, "\"Hello\"".to_string());
    assert_eq!(scanner.tokens[0].token_type, TokenType::String);
    assert_eq!(scanner.tokens[1].token_type, TokenType::EOF);
}


#[test]
fn string_unterminated_error() {
    let src = "\"Hello";
    let mut scanner = Scanner::new(src);
    match scanner.scan_tokens() {
        Err(msg) => {
            assert_eq!(msg, "Unterminated string at possition [1 | 0:6]: \"Hello\n".to_string());
        },
        Ok(_) => ()
    }
}

#[test]
fn string_unexpected_char_error() {
    let src = "\"\\w\"";
    let mut scanner = Scanner::new(src);
    match scanner.scan_tokens() {
        Err(msg) => {
            assert_eq!(msg, "Unexpected charrecter at possition [1 | 0:3]: \"\\w\n".to_string());
        },
        Ok(_) => ()
    }
}

#[test]
fn handle_char_tokens() {
    let src = "'A'";
    let mut scanner = Scanner::new(src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::CharValue('A')));
    assert_eq!(scanner.tokens[0].lexeme, "'A'".to_string());
    assert_eq!(scanner.tokens[0].token_type, TokenType::Char);
    assert_eq!(scanner.tokens[1].token_type, TokenType::EOF);
}

#[test]
fn handle_special_chars_tokens() {
    let src: String = read_file("tests\\codes\\handle_special_chars_tokens.ppl").unwrap();

    let mut scanner = Scanner::new(src.as_str());
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 7);
    assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::CharValue('A')));
    assert_eq!(scanner.tokens[2].literal, Some(LiteralValue::CharValue('\t')));
    assert_eq!(scanner.tokens[4].literal, Some(LiteralValue::CharValue('\n')));

    assert_eq!(scanner.tokens[0].lexeme, "'A'".to_string());
    assert_eq!(scanner.tokens[1].lexeme, "// Тут будет просто 'A'".to_string());
    assert_eq!(scanner.tokens[2].lexeme, "'\\t'".to_string());
    assert_eq!(scanner.tokens[3].lexeme, "// Тут уже будет '\\t'".to_string());
    assert_eq!(scanner.tokens[4].lexeme, "'\\n'".to_string());
    assert_eq!(scanner.tokens[5].lexeme, "// Тут уже будет '\\n'".to_string());

    assert_eq!(scanner.tokens[0].token_type, TokenType::Char);
    assert_eq!(scanner.tokens[1].token_type, TokenType::Coment);
    assert_eq!(scanner.tokens[2].token_type, TokenType::Char);
    assert_eq!(scanner.tokens[3].token_type, TokenType::Coment);
    assert_eq!(scanner.tokens[4].token_type, TokenType::Char);
    assert_eq!(scanner.tokens[5].token_type, TokenType::Coment);
    assert_eq!(scanner.tokens[6].token_type, TokenType::EOF);
}

#[test]
fn char_unterminated_error() {
    let src = "'A";
    let mut scanner = Scanner::new(src);
    match scanner.scan_tokens() {
        Err(msg) => {
            assert_eq!(msg, "Unterminated char at possition [1 | 0:2]: 'A\n".to_string());
        },
        Ok(_) => ()
    }
}

#[test]
fn char_unexpected_char_error() {
    let src = "'\\w'";
    let mut scanner = Scanner::new(src);
    match scanner.scan_tokens() {
        Err(msg) => {
            assert_eq!(msg, "Unexpected charrecter at possition [1 | 0:3]: '\\w\n".to_string());
        },
        Ok(_) => ()
    }
}

#[test]
fn handle_number_token() {
    let src = "100";
    let mut scanner = Scanner::new(src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::UndefinedIntValue(100)));
    assert_eq!(scanner.tokens[1].token_type, TokenType::EOF);
}

#[test]
fn handle_number_with_underscore_token() {
    let src = "1_000_000";
    let mut scanner = Scanner::new(src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::UndefinedIntValue(1_000_000)));
    assert_eq!(scanner.tokens[1].token_type, TokenType::EOF);
}

#[test]
fn handle_numbers_tokens() {
    let src = read_file("tests\\codes\\handle_number_tokens.ppl").unwrap();
    let mut scanner = Scanner::new(src.as_str());
    let _ = scanner.scan_tokens();

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

    assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::UndefinedIntValue(100)));
    assert_eq!(scanner.tokens[1].literal, Some(LiteralValue::UndefinedFloatValue(100.0)));
    assert_eq!(scanner.tokens[2].literal, Some(LiteralValue::F32Value(100.0)));
    assert_eq!(scanner.tokens[3].literal, Some(LiteralValue::F64Value(100.0)));
    assert_eq!(scanner.tokens[4].literal, Some(LiteralValue::F32Value(100.0)));
    assert_eq!(scanner.tokens[5].literal, Some(LiteralValue::F64Value(100.0)));
    assert_eq!(scanner.tokens[6].literal, Some(LiteralValue::I8Value(100)));
    assert_eq!(scanner.tokens[7].literal, Some(LiteralValue::I16Value(100)));
    assert_eq!(scanner.tokens[8].literal, Some(LiteralValue::I32Value(100)));
    assert_eq!(scanner.tokens[9].literal, Some(LiteralValue::I64Value(100)));
    assert_eq!(scanner.tokens[10].literal, Some(LiteralValue::I128Value(100)));
    assert_eq!(scanner.tokens[11].literal, Some(LiteralValue::ISizeValue(100)));
    assert_eq!(scanner.tokens[12].literal, Some(LiteralValue::U8Value(100)));
    assert_eq!(scanner.tokens[13].literal, Some(LiteralValue::U16Value(100)));
    assert_eq!(scanner.tokens[14].literal, Some(LiteralValue::U32Value(100)));
    assert_eq!(scanner.tokens[15].literal, Some(LiteralValue::U64Value(100)));
    assert_eq!(scanner.tokens[16].literal, Some(LiteralValue::U128Value(100)));
    assert_eq!(scanner.tokens[17].literal, Some(LiteralValue::USizeValue(100)));
    
    assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[1].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[2].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[3].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[4].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[5].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[6].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[7].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[8].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[9].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[10].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[11].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[12].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[13].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[14].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[15].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[16].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[17].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[18].token_type, TokenType::EOF);
}

#[test]
fn handle_hex_number_token() {
    let src = "0xFF";
    let mut scanner = Scanner::new(src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::UndefinedIntValue(0xFF)));
    assert_eq!(scanner.tokens[1].token_type, TokenType::EOF);
}

// for token in scanner.tokens.iter() {
//     println!("{:?}", token);
// }