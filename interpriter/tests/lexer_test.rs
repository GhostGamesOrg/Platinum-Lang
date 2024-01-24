use interpriter::lexer::{lexer::*, token::TokenType};

#[test]
fn handle_one_char_tokens() {
    let src = "() [] {} , . ; : ~";
    let mut scanner = Scanner::new(src);
    let _ = scanner.scan_tokens();

    assert_eq!(scanner.tokens.len(), 12);
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
    let src = "// this is simple comment\n () \n() /// this is doc comment";
    let mut scanner = Scanner::new(src);
    let _ = scanner.scan_tokens();

    for token in scanner.tokens.iter() {
        println!("{:?}", token);
    }

    assert_eq!(scanner.tokens.len(), 7);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Coment);
    assert_eq!(scanner.tokens[1].token_type, TokenType::LeftParen);
    assert_eq!(scanner.tokens[2].token_type, TokenType::RightParen);
    assert_eq!(scanner.tokens[3].token_type, TokenType::LeftParen);
    assert_eq!(scanner.tokens[4].token_type, TokenType::RightParen);
    assert_eq!(scanner.tokens[5].token_type, TokenType::DocComent);
    assert_eq!(scanner.tokens[6].token_type, TokenType::EOF);
}

// for token in scanner.tokens.iter() {
//     println!("{:?}", token);
// }