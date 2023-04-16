use std::io::{self, Write};

use rmonkey_evaluator::Evaluator;
use rmonkey_lexer::Lexer;
use rmonkey_parser::Parser;

fn main() {
    println!("Welcome to Monkey");

    let mut e = Evaluator::new();
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
                let l = Lexer::new(input);
                let mut p = Parser::new(l);
                let program = p.parse_program().unwrap();
                match e.eval(program) {
                    Ok(result) => println!("{result}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
    }
}
