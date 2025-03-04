// Refer to Homework3.docx for instructions on how to write this
use std::{env, fmt, error::Error};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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

#[derive(Debug)]
struct CLIError {
    msg: String,
    source: Option<Box<dyn Error>>
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for CLIError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

fn movetodir(tokens: Vec<&str>) -> Result<PathBuf, CLIError>{
    if tokens.len() > 2 {
        Err()
    } else {
        let new_path: PathBuf = PathBuf::from(tokens[1].to_string());

        if new_path.exists() {
            return Ok(new_path)
        }

        return Err("Invalid Path!");
    }
}

fn whereami(tokens: Vec<&str>, cwd: &PathBuf) -> Result<(), &'static str>{
    if tokens.len() > 1 {
        Err("Usage: whereami")
    } else {
        println!("{}", cwd.to_str().unwrap());

        Ok(())
    }
}

fn history(tokens: Vec<&str>) {
    todo!()
}

fn replay(tokens: Vec<&str>) {
    todo!()
}

fn start(tokens: Vec<&str>) {
    todo!()
}

fn background(tokens: Vec<&str>) {
    todo!()
}

fn dalek(tokens: Vec<&str>) {
    todo!()
}

fn main() -> Result<(), io::Error>{
    let mut cwd: PathBuf = env::current_dir()?;

    print!("# ");
    io::stdout().flush().unwrap();

    loop {
        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line: &str = line.trim_end();

        println!("line: {line}");

        let tokens: Vec<&str> = line.split(' ').collect();

        println!("{tokens:?}");

        match tokens[0] {
            "movetodir" => {
                cwd = movetodir(tokens)?;
            },
            "whereami" => whereami(tokens, &cwd),
            "replay" => replay(tokens),
            "start" => start(tokens),
            "background" => background(tokens),
            "dalek" => dalek(tokens),
            "history" => history(tokens),
            "byebye" => std::process::exit(0),
            _ => println!("{}: command not found", tokens[0])
        }

        print!("# ");
        io::stdout().flush().unwrap();
    }
}