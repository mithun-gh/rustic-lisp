use rustyline::{Editor, error::ReadlineError};
use crate::interpreter::lexer::{Lexer, Token};

pub fn run() {
    println!("Welcome to LISP Calc v0.1\n");

    let mut rl = Editor::<()>::new();

    loop {
        match rl.readline("❯ ") {
            Ok(line) => {
                let line = line.trim();
                rl.add_history_entry(line);
                print_output(line);
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => eprintln!("{:?}", err),
        }
    }
}

fn print_output(input: &str) {
    let input = input.chars().collect::<Vec<char>>();
    let lexer = Lexer::new(input.iter());
    println!("{:?}", lexer.collect::<Vec<Token>>());
}
