use std::{self, fs};

use interpriter::lexer::lexer::Scanner;
use interpriter::lexer::token::{Token, TokenType, LiteralValue};
use interpriter::parser::{expr::Expr, parser::Parser};

fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let src: String = fs::read_to_string(path)?.parse()?;
    Ok(src)
}


#[test]
fn pretty_print_ast() {
    let minus_token = Token::new(TokenType::Minus, "-".to_string(), None, (1, 2, 10));

    let left = Box::from(Expr::Literal { value: LiteralValue::I128Value(0xfff) });
    let right = Box::from(Expr::Literal { value: LiteralValue::U8Value(0xff) });

    let ast = Expr::Binary { left: left, operator: minus_token, right: right };
    assert_eq!("((4095i128) - (255u8))", ast.to_string());
}

#[test]
fn pretty_print2_ast() {
    let file_path = "<stdin>";
    let src = "4095i128 - 255u8";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();
    let mut parser = Parser::new(file_path, scanner.tokens);

    let minus_token = Token::new(TokenType::Minus, "-".to_string(), None, (1, 9, 10));
    let left = Box::from(Expr::Literal { value: LiteralValue::I128Value(0xfff) });
    let right = Box::from(Expr::Literal { value: LiteralValue::U8Value(0xff) });
    let expr = Expr::Binary { left: left, operator: minus_token, right: right };

    let ast = parser.parse().unwrap();
    assert_eq!("((4095i128) - (255u8))", ast.to_string());
    assert_eq!(expr, ast);
}

#[test]
fn simple_parse_expr() {
    let file_path = "<stdin>";
    let src = "(1 + (1 * 123)) / 213";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!("((group ((1) + (group ((1) * (123))))) / (213))", expr.to_string());
}

#[test]
fn parse_binary_expr() {
    let file_path = "<stdin>";
    let src = "(38u8 + 24) - 95i16";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!("((group ((38u8) + (24))) - (95i16))", expr.to_string());
}

#[test]
fn parse_equality_expr() {
    let file_path = "<stdin>";
    let src = "(38u8 != 95i16) == (11u8 == 11u16)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!("((group ((38u8) != (95i16))) == (group ((11u8) == (11u16))))", expr.to_string());
}

#[test]
fn parse_equality_binary_expr() {
    let file_path = "<stdin>";
    let src = "38u128 + 95u16 != 11u8 - 11u16";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!("(((38u128) + (95u16)) != ((11u8) - (11u16)))", expr.to_string());
}

#[test]
fn eval_literal_expr() {
    let file_path = "<stdin>";
    let src = "100";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!(LiteralValue::UndefinedIntValue(100), expr.evaluate());
}

#[test]
fn eval_binary_expr() {
    let file_path = "<stdin>";
    let src = "11 * 20";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!(LiteralValue::UndefinedIntValue(220), expr.evaluate());
}

#[test]
fn eval_binary2_expr() {
    let file_path = "<stdin>";
    let src = "(100 + 200) * 20";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!(LiteralValue::UndefinedIntValue(6000), expr.evaluate());
}

#[test]
fn eval_binary3_expr() {
    let file_path = "<stdin>";
    let src = "(100 + 200) * 20 == 6000";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!(LiteralValue::Bool(true), expr.evaluate());
}

#[test]
fn eval_binary4_expr() {
    let file_path = "<stdin>";
    let src = "((100i + 14i) * 20 == 6000) == false";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    for token in scanner.tokens.iter() {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!(LiteralValue::Bool(true), expr.evaluate());
}

#[test]
fn eval_binary5_expr() {
    let file_path = "<stdin>";
    let src = "((0x214i + 0x214i) * 20 == 6000) == !!false";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let expr = parser.parse().unwrap();

    assert_eq!(LiteralValue::Bool(true), expr.evaluate());
}