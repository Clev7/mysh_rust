// Refer to Homework3.docx for instructions on how to write this
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    FileNotFound(PathBuf),
    BadLen(usize),
    InvalidUsage,
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

fn movetodir(tokens: &[&str]) -> Result<PathBuf, Error> {
    if tokens.len() > 2 {
        Err(Error::BadLen(tokens.len()))
    } else { 
        let new_path: PathBuf = PathBuf::from(tokens[1].to_string());

        if new_path.exists() {
            return Ok(new_path)
        }

        Err(Error::FileNotFound(new_path))
    }
}

fn whereami(tokens: &[&str], cwd: &PathBuf) -> Result<(), Error>{
    if tokens.len() > 1 {
        Err(Error::BadLen(tokens.len()))
    } else {
        println!("{}", cwd.to_str().unwrap());

        Ok(())
    }
}

fn get_history(tokens: &[&str], history: &mut Vec<String>) -> Result<(), Error> {
    if tokens.len() > 2 {
        return Err(Error::BadLen(tokens.len()));
    }

    if tokens.len() == 2 {
        if tokens[1] != "-c" {
            Err(Error::InvalidUsage)
        } else {
            history.clear();

            Ok(())
        }
    } else {
        for (i, line) in history.iter().enumerate() {
            println!("{i}: {line}");
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

fn main() -> Result<(), Error>{
    let mut cwd = env::current_dir().map_err(|err| Error::IoError(err))?;
    let mut history: Vec<String> = Vec::new();

    loop {
        print!("# ");
        io::stdout().flush().unwrap();

        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line: &str = line.trim_end_matches('\n').trim_end();

        // println!("line: {line}");

        history.push(trimmed_line.to_string());

        let tokens: Vec<&str> = trimmed_line.split(' ').collect();
        // println!("{tokens:?}");

        match tokens[0] {
            "movetodir" => {
                cwd = movetodir(&tokens)?;
            },
            "whereami" => whereami(&tokens, &cwd)?,
            "replay" => replay(&tokens),
            "start" => start(&tokens),
            "background" => background(&tokens),
            "dalek" => dalek(&tokens),
            "history" => get_history(&tokens, &mut history)?,
            "byebye" => std::process::exit(0),
            _ => println!("{}: command not found", tokens[0])
        }

        io::stdout().flush().unwrap();
    }
}