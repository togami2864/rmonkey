use std::io::{self, Write};

use rmonkey_lexer::Lexer;

fn main() {
    println!("Welcome to Monkey");

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        print!(">>> ");
        io::stdout().flush().expect("Unable to flush stdout");
        stdin
            .read_line(&mut buffer)
            .expect("Unable to read line from user");
        match buffer.trim() {
            ".quit" => {
                println!("Bye!");
                std::process::exit(0);
            }
            input => {
                let mut l = Lexer::new(input);
                l.tokenize();
            }
        }
    }
}
