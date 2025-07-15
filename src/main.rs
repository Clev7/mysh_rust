// Refer to Homework3.docx for instructions on how to write this
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Child;

use mysh_rust::{dispatch, tokenize::tokenize};
use mysh_rust::error_handling::handle_err;

fn main() -> () {
    let mut cwd: PathBuf = env::current_dir().unwrap();
    let mut history: Vec<String> = Vec::new();
    let mut children: Vec<Child> = Vec::new();

    loop {
        print!("# ");
        io::stdout().flush().unwrap();

        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.is_empty() {
            continue;
        }

        let (trimmed_line, tokens) = tokenize(line.as_str());

        if tokens[0] != "replay" {
            history.push(trimmed_line.to_string());
        }

        dispatch(&tokens, &mut history, &mut cwd, &mut children).unwrap_or_else(handle_err);

        io::stdout().flush().unwrap();
    }
}
