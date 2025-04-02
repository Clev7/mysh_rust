mod utils;
mod cli_error;

// Refer to Homework3.docx for instructions on how to write this
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use utils::{tokenize, dispatch};
use cli_error::CliError;

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

fn movetodir(tokens: &[&str], cwd: &mut PathBuf) -> Result<(), CliError> {
    if tokens.len() != 2 {
        Err(CliError::BadLen(tokens.len()))
    } else {
        let new_path = PathBuf::from(tokens[1].to_string());

        if new_path.exists() {
            // This value is moved.
            *cwd = new_path;
            
            return Ok(());
        }

        Err(CliError::FileNotFound(new_path))
    }
}

fn whereami(tokens: &[&str], cwd: &PathBuf) -> Result<(), CliError> {
    if tokens.len() > 1 {
        Err(CliError::BadLen(tokens.len()))
    } else {
        println!("{}", cwd.to_str().unwrap());

        Ok(())
    }
}

fn get_history(tokens: &[&str], history: &mut Vec<String>) -> Result<(), CliError> {
    if tokens.len() > 2 {
        return Err(CliError::BadLen(tokens.len()));
    }

    if tokens.len() == 2 {
        if tokens[1] != "-c" {
            Err(CliError::InvalidUsage)
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

// TODO: Implement with split_at_mut.
// TODO: understand split_at_mut better.
fn replay(curr_tokens: &[&str], history: &mut Vec<String>, cwd: &mut PathBuf) -> Result<(), CliError> {
    if curr_tokens.len() != 2  {
        return Err(CliError::BadLen(curr_tokens.len()));
    }

    let idx: usize = curr_tokens[1]
        .parse::<usize>()
        .map_err(|err| CliError::ParseError(err))?;

    let (trimmed_line, command_tokens) = tokenize(&history[idx]);
    let trimmed_line = trimmed_line.to_owned();
    let mut command_vec: Vec<String> = Vec::new();

    for token in command_tokens.iter() {
        command_vec.push(token.to_string());
    }

    let command_slice: Vec<&str> = command_vec.iter().map(|s| s.as_str()).collect();

    if idx != 0 {
        history.push(trimmed_line);
        // println!("{history:?} {command_vec:?}");
        dispatch(&command_slice, &mut history.clone(), cwd)?;
    }

    Ok(())
}

fn start(tokens: &[&str]) -> Result<(), CliError> {
    if tokens.len() < 2 {
        return Err(CliError::BadLen(tokens.len()));
    }

    let mut child = Command::new(tokens[1])
        .args(&tokens[2..])
        .spawn()
        .map_err(|err: std::io::Error| CliError::IoError(err))?;

    child.wait().map_err(|err: std::io::Error| CliError::IoError(err))?;

    Ok(())
}

fn background(tokens: &[&str]) -> Result<(), CliError>{
    if tokens.len() < 2 {
        return Err(CliError::BadLen(tokens.len()));
    }

    Command::new(tokens[1])
        .args(&tokens[2..])
        .spawn()
        .map_err(|err: std::io::Error| CliError::IoError(err))?;
    
    Ok(())
}

fn dalek(tokens: &[&str]) -> Result<(), CliError>{
    todo!()
}

fn handle_err(err: CliError) {
    println!("{:?}", err);
}

fn main() -> () {
    let mut cwd: PathBuf = env::current_dir().unwrap();
    let mut history: Vec<String> = Vec::new();

    loop {
        print!("# ");
        io::stdout().flush().unwrap();

        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let (trimmed_line, tokens) = utils::tokenize(line.as_str());

        if tokens[0] != "replay" {
            history.push(trimmed_line.to_string());
        }

        // Takes in tokens: Vec<&str>, cwd: 
        // dispatch(&tokens, &mut history, &mut cwd).unwrap();

        dispatch(&tokens, &mut history, &mut cwd).unwrap_or_else(handle_err);

        io::stdout().flush().unwrap();
    }
}
