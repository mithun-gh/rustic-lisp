use rustyline::{Editor, error::ReadlineError};

pub fn run() {
    println!("Welcome to LISP Calc v0.1\n");

    let mut rl = Editor::<()>::new();

    loop {
        match rl.readline("❯ ") {
            Ok(line) => {
                let line = line.trim();
                rl.add_history_entry(line);
                println!("{}", line);
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => eprintln!("{:?}", err),
        }
    }
}
