use std::fs;
use std::io;

use crate::error::RloxError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

/// Run lox from source file.
pub fn run_file(path: &str) -> Result<(), RloxError> {
    let content = fs::read_to_string(path)?;
    let mut interpreter = Interpreter::new();
    run(&content, &mut interpreter)
}

/// Run lox using REPL.
pub fn run_prompt() -> Result<(), RloxError> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut interpreter = Interpreter::new();
    loop {
        println!("> ");
        buffer.clear();
        let len = stdin.read_line(&mut buffer)?;
        if len > 0 {
            run(&buffer, &mut interpreter)?
        }
    }
}

fn run(source: &str, interpreter: &mut Interpreter) -> Result<(), RloxError> {
    let mut scanner = Scanner::new(source.to_owned());
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Some(program) if !parser.had_error => {
            interpreter.interpret(program);
        }
        _ => {}
    }

    Ok(())
}
