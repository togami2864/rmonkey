use std::io::{self, Write};

use rmonkey_evaluator::Evaluator;
use rmonkey_lexer::Lexer;
use rmonkey_parser::Parser;

fn main() {
    println!("Welcome to Monkey");

    match std::env::args().nth(1) {
        Some(val) => {
            if val == "lexer" {
                println!("lexer mode");
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
            } else if val == "parser" {
                println!("parser mode");
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
                            p.parse();
                        }
                    }
                }
            } else {
                println!("no such a command: {}", val)
            }
        }

        None => loop {
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
                    let e = Evaluator {};
                    // let result = e.eval(program).unwrap();
                    // println!("{}", result);
                    match e.eval(program) {
                        Ok(result) => println!("{}", result),
                        Err(err) => eprintln!("{}", err),
                    }
                }
            }
        },
    }
}
