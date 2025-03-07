// Refer to Homework3.docx for instructions on how to write this
use std::{env, fmt};
use std::io::{self, Write};

use std::path::PathBuf;

#[derive(Debug)]
enum error {
    IoError(std::io::Error),
    FileNotFound(PathBuf),
    BadLen(usize),
    InvalidUsage
}

/*
    Commands to implement:
    1. movetodir
    2. whereami
    3. history [-c]
    4. byebye
    5. replay number
    6. start program [parameters]
    7. background program [parameters]
    8. dalek PID

    Bonuses:
    9. repeat n command _
    10. dalekall
*/

fn movetodir(tokens: &[&str]) -> Result<PathBuf, error> {
    if tokens.len() > 2 {
        Err(error::BadLen(tokens.len()))
    } else { 
        let new_path: PathBuf = PathBuf::from(tokens[1].to_string());

        if new_path.exists() {
            return Ok(new_path)
        }

        Err(error::FileNotFound(new_path))
    }
}

fn whereami(tokens: &[&str], cwd: &PathBuf) -> Result<(), error>{
    if tokens.len() > 1 {
        Err(error::BadLen(tokens.len()))
    } else {
        println!("{}", cwd.to_str().unwrap());

        Ok(())
    }
}

fn get_history(tokens: &[&str], history: &[&str]) -> Result<(), error> {
    if tokens.len() > 2 {
        Err(error::BadLen(tokens.len()))
    } else if tokens[1] != "-c" {
        Err(error::InvalidUsage)
    } else {

        for &line in history {
            println!("{line}");
        }

        Ok(())
    }
}

fn replay(tokens: &[&str]) {
    todo!()
}

fn start(tokens: &[&str]) {
    todo!()
}

fn background(tokens: &[&str]) {
    todo!()
}

fn dalek(tokens: &[&str]) {
    todo!()
}

fn main() -> Result<(), error>{
    let mut cwd: PathBuf = env::current_dir()?;
    let mut history: Vec<&str> = Vec::new();

    print!("# ");
    io::stdout().flush().unwrap();

    loop {
        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line: &str = line.trim_end();

        println!("line: {line}");

        history.push(line);

        let tokens: Vec<&str> = line.split(' ').collect();

        println!("{tokens:?}");

        match tokens[0] {
            "movetodir" => {
                cwd = movetodir(&tokens)?;
            },
            "whereami" => whereami(&tokens, &cwd)?,
            "replay" => replay(&tokens),
            "start" => start(&tokens),
            "background" => background(&tokens),
            "dalek" => dalek(&tokens),
            "history" => get_history(&tokens, &history)?,
            "byebye" => std::process::exit(0),
            _ => println!("{}: command not found", tokens[0])
        }

        print!("# ");
        io::stdout().flush().unwrap();
    }
}