use std::fs;
use std::io;

use crate::error::RloxError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

/// Run lox from source file.
pub fn run_file(path: &str) -> Result<(), RloxError> {
    let content = fs::read_to_string(path)?;
    run(&content)
}

/// Run lox using REPL.
pub fn run_prompt() -> Result<(), RloxError> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        println!("> ");
        buffer.clear();
        let len = stdin.read_line(&mut buffer)?;
        if len > 0 {
            run(&buffer)?
        }
    }
}

fn run(source: &str) -> Result<(), RloxError> {
    let mut scanner = Scanner::new(source.to_owned());
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Some(program) if !parser.had_error => {
            let mut interpreter = Interpreter::new();
            if let Some(value) = interpreter.inperpret(program) {
                println!("{}", value);
            }
        }
        _ => {}
    }

    Ok(())
}
