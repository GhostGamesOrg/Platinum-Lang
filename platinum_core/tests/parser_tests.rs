use std::{self, fs};

use platinum_core::lexer::lexer::Scanner;
use platinum_core::parser::parser::Parser;

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

    assert_eq!("(assigment -33)", statements[0].to_string());
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

    
    assert_eq!("(block \n(assigment 308)\n(assigment 113)\n(assigment 123)\n)", statements[0].to_string());
}

#[test]
fn parse_let_stmt() {
    let file_path = "<stdin>";
    let src = "let mut SASA: u8 = 10;";

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

    assert_eq!("(let mut Identifier { value: \"SASA\" } SASA: Identifier { value: \"u8\" } u8 = (assigment 10))", statements[0].to_string());
}

#[test]
fn parse_let_stmt2() {
    let file_path = "<stdin>";
    let src = "let mut SASA: u8;";

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

    assert_eq!("(let mut Identifier { value: \"SASA\" } SASA: Identifier { value: \"u8\" } u8 = (assigment null))", statements[0].to_string());
}

#[test]
fn parse_if_stmt() {
    let file_path = "tests\\parser_codes\\parse_if_stmt.ppl";
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

    assert_eq!("(if ((assigment true)) (block \n(assigment true)\n))", statements[0].to_string());
}

#[test]
fn parse_if_else_stmt() {
    let file_path = "tests\\parser_codes\\parse_if_else_stmt.ppl";
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

    assert_eq!("(if ((assigment true)) (block \n(assigment true)\n) else (block \n(assigment \"hello\")\n))", statements[0].to_string());
}

#[test]
fn parse_if_else_stmt2() {
    let file_path = "tests\\parser_codes\\parse_if_else_stmt2.ppl";
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

    assert_eq!("(if ((assigment true)) (block \n(assigment true)\n) else (if ((assigment false)) (block \n(assigment 123)\n) else (block \n(assigment \"hello\")\n)))", statements[0].to_string());
}

#[test]
fn parse_func_define_stmt() {
    let file_path = "tests\\parser_codes\\parse_func_define_stmt.ppl";
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

    assert_eq!("(fun Identifier { value: \"hello\" } hello([NotOptional { name: Token { token_type: Identifier { value: \"a\" }, lexeme: \"a\", possition: (1, 10, 11) }, _type: Token { token_type: Identifier { value: \"u8\" }, lexeme: \"u8\", possition: (1, 13, 15) } }, Optional { name: Token { token_type: Identifier { value: \"b\" }, lexeme: \"b\", possition: (1, 17, 18) }, _type: Token { token_type: Identifier { value: \"i128\" }, lexeme: \"i128\", possition: (1, 20, 24) }, value: Literal { value: Token { token_type: Int { value: \"2583\", num_type: UntypedInt }, lexeme: \"2583\", possition: (1, 31, 32) } } }]) -> Null void (block \n(assigment 2)\n))", statements[0].to_string());
}

#[test]
fn parse_func_use_stmt() {
    let file_path = "tests\\parser_codes\\parse_func_use_stmt.ppl";
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

    assert_eq!("(functionUse Identifier { value: \"print\" } print([Expr { value: Literal { value: Token { token_type: StringT { value: \"Hello\" }, lexeme: \"\\\"Hello\\\"\", possition: (1, 6, 13) } } }, Optional { name: Token { token_type: Identifier { value: \"a\" }, lexeme: \"a\", possition: (1, 15, 16) }, value: Literal { value: Token { token_type: BoolT { value: true }, lexeme: \"true\", possition: (1, 19, 23) } } }])", statements[0].to_string());
}

#[test]
fn parse_loop_stmt() {
    let file_path = "tests\\parser_codes\\parse_loop_stmt.ppl";
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
    
    assert_eq!("(loop (block \n(let Identifier { value: \"a\" } a: Identifier { value: \"u8\" } u8 = (assigment 123))\n))", statements[0].to_string());
}

#[test]
fn parse_for_stmt() {
    let file_path = "tests\\parser_codes\\parse_for_stmt.ppl";
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
    
    assert_eq!("(for (Identifier { value: \"i\" } i in (range (assigment 0)..(assigment 11))) (block \n(assigment 2)\n))", statements[0].to_string());
}

#[test]
fn parse_while_stmt() {
    let file_path = "tests\\parser_codes\\parse_while_stmt.ppl";
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
    
    assert_eq!("(while (assigment false) (block \n(let Identifier { value: \"a\" } a: Identifier { value: \"u8\" } u8 = (assigment null))\n))", statements[0].to_string());
}

#[test]
fn parse_do_while_stmt() {
    let file_path = "tests\\parser_codes\\parse_do_while_stmt.ppl";
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
    
    assert_eq!("(do (block \n(let Identifier { value: \"a\" } a: Identifier { value: \"u8\" } u8 = (assigment null))\n) while (assigment false))", statements[0].to_string());
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

#[test]
fn test_binary_modulus() {
    let file_path = "<stdin>";
    let src = "10 % 3";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 1)", statements[0].to_string());
}

#[test]
fn test_binary_bitwise_and() {
    let file_path = "<stdin>";
    let src = "5 & 3";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 1)", statements[0].to_string());
}

#[test]
fn test_binary_addition() {
    let file_path = "<stdin>";
    let src = "5 + 3";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 8)", statements[0].to_string());
}

#[test]
fn test_binary_subtraction() {
    let file_path = "<stdin>";
    let src = "10 - (3 + 2)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 5)", statements[0].to_string());
}

#[test]
fn test_binary_multiplication() {
    let file_path = "<stdin>";
    let src = "(3 * 2) * 4";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 24)", statements[0].to_string());
}

#[test]
fn test_binary_integer_division() {
    let file_path = "<stdin>";
    let src = "10 / (2 * 2)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 2)", statements[0].to_string());
}

#[test]
fn test_binary_float_division() {
    let file_path = "<stdin>";
    let src = "10.0 / (2 * 2)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 2.5)", statements[0].to_string());
}

#[test]
fn test_binary_bitwise_or() {
    let file_path = "<stdin>";
    let src = "5 | 3";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 7)", statements[0].to_string());
}

#[test]
fn test_complex_expression_with_parentheses() {
    let file_path = "<stdin>";
    let src = "(5 * (4 + 3)) - (10 % 3)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 34)", statements[0].to_string());
}

#[test]
fn test_binary_bitwise_xor() {
    let file_path = "<stdin>";
    let src = "5 ^ 3";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 6)", statements[0].to_string());
}

#[test]
fn test_complex_expression_with_multiple_parentheses() {
    let file_path = "<stdin>";
    let src = "((5 * (4 + 3)) - (10 % 3)) / 2";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 17)", statements[0].to_string());
}

#[test]
fn test_string_equality_comparison() {
    let file_path = "<stdin>";
    let src = "\"hello\" == \"world\"";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment false)", statements[0].to_string());
}

#[test]
fn test_integer_comparison_operators() {
    let file_path = "<stdin>";
    let src = "5 < 10 && 10 >= 5";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment true)", statements[0].to_string());
}

#[test]
fn test_float_equality_comparison() {
    let file_path = "<stdin>";
    let src = "3.14 == 3.14";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment true)", statements[0].to_string());
}

#[test]
fn test_string_concatenation() {
    let file_path = "<stdin>";
    let src = "\"hello\" + \"world\"";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment helloworld)", statements[0].to_string());
}

#[test]
fn test_integer_addition() {
    let file_path = "<stdin>";
    let src = "5 + 10";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 15)", statements[0].to_string());
}

#[test]
fn test_ternary_expression() {
    let file_path = "<stdin>";
    let src = "true ? 10 : 20";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 10)", statements[0].to_string());
}

#[test]
fn test_unary_expression() {
    let file_path = "<stdin>";
    let src = "-5";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment -5)", statements[0].to_string());
}

#[test]
fn test_grouping_expression() {
    let file_path = "<stdin>";
    let src = "(5 + 10) * 3";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 45)", statements[0].to_string());
}

#[test]
fn test_literal_expression() {
    let file_path = "<stdin>";
    let src = "true";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment true)", statements[0].to_string());
}

#[test]
fn test_complex_ternary_expression() {
    let file_path = "<stdin>";
    let src = "true ? (5 * 2) : (10 / 2)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 10)", statements[0].to_string());
}

#[test]
fn test_nested_grouping_expression() {
    let file_path = "<stdin>";
    let src = "(((5)))";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 5)", statements[0].to_string());
}

#[test]
fn test_literal_string_expression() {
    let file_path = "<stdin>";
    let src = "\"hello\"";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment \"hello\")", statements[0].to_string());
}

#[test]
fn test_nested_ternary_expression() {
    let file_path = "<stdin>";
    let src = "true ? (false ? 1 : 2) : (false ? 3 : 4)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 2)", statements[0].to_string());
}

#[test]
fn test_unary_not_expression() {
    let file_path = "<stdin>";
    let src = "!true";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment false)", statements[0].to_string());
}

#[test]
fn test_literal_integer_expression() {
    let file_path = "<stdin>";
    let src = "42";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 42)", statements[0].to_string());
}

#[test]
fn test_complex_expression_1() {
    let file_path = "<stdin>";
    let src = "!(((5 * (4 + 3)) - (10 % 3)) / 2 == 6) && true ? (5 * 2) : (10 / 2)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 10)", statements[0].to_string());
}

#[test]
fn test_complex_expression_2() {
    let file_path = "<stdin>";
    let src = "-(-(-(-(5)))) == 5";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment true)", statements[0].to_string());
}

#[test]
fn test_complex_expression_3() {
    let file_path = "<stdin>";
    let src = "(true && false) || (true && true) ? 1 : 2";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 1)", statements[0].to_string());
}

#[test]
fn test_complex_expression_4() {
    let file_path = "<stdin>";
    let src = "(5 == 5) ? (6 == 6) ? 10 : 20 : (false && true) ? 30 : 40";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 10)", statements[0].to_string());
}

#[test]
fn test_complex_expression_5() {
    let file_path = "<stdin>";
    let src = "((5 + 3) * (4 / 2)) % 3 < 2 && (true && false)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment false)", statements[0].to_string());
}

#[test]
fn test_complex_expression_6() {
    let file_path = "<stdin>";
    let src = "true ? (10 < 5 ? false : 10 > 5) : (true && false)";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment true)", statements[0].to_string());
}

#[test]
fn test_complex_expression_7() {
    let file_path = "<stdin>";
    let src = "!true && (5 == 5) ? 10 : 20";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 20)", statements[0].to_string());
}

#[test]
fn test_complex_expression_8() {
    let file_path = "<stdin>";
    let src = "(10 < 5) ? 5 : (true && false) ? 10 : 20";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 20)", statements[0].to_string());
}

#[test]
fn test_complex_expression_9() {
    let file_path = "<stdin>";
    let src = "((5 * 2) / 4) > 1 ? 10 : 20";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 10)", statements[0].to_string());
}

#[test]
fn test_complex_expression_10() {
    let file_path = "<stdin>";
    let src = "(true || false) ? (4 * 3) : (10 + 5) * 2 / 30";

    let mut scanner = Scanner::new(file_path, src);
    let _ = scanner.scan_tokens();

    let mut parser = Parser::new(file_path, scanner.tokens);
    let statements = parser.parse().unwrap();

    assert_eq!("(assigment 12)", statements[0].to_string());
}


// #[test]
// fn test_expression_with_variables() {
    // let file_path = "<stdin>";
    // let src = "let a = 5; let b = 3; a * b";
// 
    // let mut scanner = Scanner::new(file_path, src);
    // let _ = scanner.scan_tokens();
// 
    // let mut parser = Parser::new(file_path, scanner.tokens);
    // let statements = parser.parse().unwrap();
// 
    // assert_eq!("(assigment 15)", statements[2].to_string());
// }