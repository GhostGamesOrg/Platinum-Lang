use std::{
    env,
    process::exit,
    fs,
    io::{self, BufRead, Write}
};

use interpriter::lexer::lexer::*;
use interpriter::parser::parser::Parser;

/// Runs file
/// Use `metal run <filename>` to run
fn run_file(path: &str) -> Result<(), String> {

    // Reads file first
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(src) => run(path, &src),
    }
}

/// Runs script from console
/// Use `metal` to run and then write your script
fn run_prompt() -> Result<(), String> {
    loop {
        print!(":>> ");

        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Couldnt flush stdout".to_string()),
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        match handle.read_line(&mut buffer) {
            Ok(line) => {
                if line <= 2 {
                    return Ok(());
                }
            },
            Err(_) => return Err("Couldnt read line".to_string()),
        }
        match run("<stdin>", &buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

/// Runs source code
fn run(file_path: &str, src: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(file_path, src);
    match scanner.scan_tokens() {
        Err(msg) => println!("{}", msg),
        _ => ()
    }

    let mut parser = Parser::new(file_path, scanner.tokens.clone());
    let expr = parser.parse()?;

    println!("{}", expr.evaluate().to_string());
    return Ok(());

}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Usage: metal [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    }
}
