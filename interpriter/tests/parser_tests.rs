use std::{self, fs};

use interpriter::lexer::lexer::Scanner;
use interpriter::lexer::token::{Token, TokenType, NumberType};
use interpriter::parser::{expr::Expression, parser::Parser};

fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let src: String = fs::read_to_string(path)?.parse()?;
    Ok(src)
}

#[test]
fn parse_expr() {
    let file_path = "<stdin>";
    let src = "(38u8 + 24) - 95i16";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements; 
    match parser.parse() {
        Ok(stmts) => {
            statements = stmts;
        },
        Err(msg) => panic!("{}", msg)
    }

    assert_eq!("(assigment (- (group (+ 38u8 24)) 95i16))", statements[0].to_string());
}

#[test]
fn parse_block_stmt() {
    let file_path = "tests\\parser_codes\\parse_block_stmt.ppl";
    let src = read_file(file_path).unwrap();
    let mut scanner = Scanner::new(file_path, src.as_str());
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements; 
    match parser.parse() {
        Ok(stmts) => {
            statements = stmts;
        },
        Err(msg) => panic!("{}", msg)
    }
    println!("{:?}", statements[0]);
    assert_eq!("(block \n(assigment (+ 95 213))\n(assigment (+ 11 (* 51 2)))\n(assigment (ternary (>= (group (* 25 13)) 1254) ? 51 : 15))\n)", statements[0].to_string());
}

#[test]
fn test_parse_optimize() {
    let file_path = "<stdin>";
    let src = "\"d\" + \"d\" + (\"d\")";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements; 
    match parser.parse() {
        Ok(stmts) => {
            statements = stmts;
        },
        Err(msg) => panic!("{}", msg)
    }
    
    assert_eq!("(assigment ddd)", statements[0].to_string());
}