use std::{
    env,
    process::exit,
    fs,
    io::{self, BufRead, Write}
};

use platinum_core::lexer::lexer::*;
use platinum_core::parser::parser::Parser;
// use interpriter::interpreter::Interpreter;

/// Runs file
/// Use `metal run <filename>` to run
fn run_file(path: &str) -> Result<(), String> {
    // let mut interpreter = Interpreter::new();
    
    // Reads file first
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(src) => run(path, &src),
    }
}

/// Runs script from console
/// Use `metal` to run and then write your script
fn run_prompt() -> Result<(), String> {
    // let mut interpreter = Interpreter::new();
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

    for token in scanner.tokens.iter() {
        println!("{:?}", token);
    }
    
    let mut parser = Parser::new(file_path, scanner.tokens.clone());
    let statements = parser.parse()?;
    
    println!("{}", statements[0].to_string());

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
