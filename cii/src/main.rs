use std::{
    env,
    process::exit,
    fs,
    io::{self, BufRead, Write}
};

use interpriter::lexer::lexer::*;

/// Runs file
/// Use `metal run <filename>` to run
fn run_file(path: &str) -> Result<(), String> {

    // Reads file first
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(src) => run(&src),
    }
}

/// Runs script from console
/// Use `metal` to run and then write your script
fn run_prompt() -> Result<(), String> {
    loop {
        print!(":>> ");

        let mut buffer = String::new();

        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Couldnt flush stdout".to_string()),
        }

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
        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

/// Runs source code
fn run(src: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(src);

    let tokens = scanner.scan_tokens()?;
    
    for token in tokens {
        println!("{:?}", token);
    }
    return Ok(());

}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Usage: jlox [script]");
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
