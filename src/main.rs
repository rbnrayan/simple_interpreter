use interpreter::{interpreter::Interpreter, lexer::Lexer, parser::Parser};
use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    let mut input = String::new();

    print!(" > ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let tokens = match Lexer::new(&input).lex() {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("[lexer-error]:\n{}", e);
            exit(1);
        }
    };
    let ast = match Parser::new(tokens).parse() {
        Ok(ast) => ast,
        Err(e) => {
            println!("[parser-error]:\n{}", e);
            exit(1);
        }
    };
    let interpreter = Interpreter::new(ast);
    println!("[interpreter]:\nresult: {}", interpreter.eval());
}
