use std::fs;
use std::io::{self, Error, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::time::Instant;

use rmonkey_evaluator::Evaluator;
use rmonkey_lexer::Lexer;
use rmonkey_parser::Parser;

#[test]
fn run_monkey_file() {
    let dir_str = ::std::env::var("CARGO_MANIFEST_DIR").expect("failed to read CARGO_MANIFEST_DIR");
    let dir = Path::new(&dir_str).join("tests/fixtures");
    let files = get_monkey_files(dir).unwrap();

    for (path, file_name) in files {
        let code = read_file_content(path).unwrap();
        let mut e = Evaluator::new();

        let time = Instant::now();
        let l = Lexer::new(&code);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        match e.eval(program) {
            Ok(_) => {
                println!("{file_name}:{:?}", time.elapsed());
            }
            Err(err) => eprintln!("{err}"),
        }
    }
}

fn get_monkey_files<P: AsRef<Path>>(dir: P) -> io::Result<Vec<(PathBuf, String)>> {
    let entries = fs::read_dir(dir)?;
    let mut monkey_files = Vec::new();

    for entry in entries {
        let entry = entry?;
        let file_name = entry
            .file_name()
            .into_string()
            .expect("OsString.into_string()");
        let path = entry.path();
        let extension = path.extension().unwrap_or_default();

        if !path.is_file() {
            continue;
        }

        if extension == "monkey" {
            monkey_files.push((path, file_name));
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Found a non-monkey file in the fixtures directory",
            ));
        }
    }

    Ok(monkey_files)
}

fn read_file_content<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
