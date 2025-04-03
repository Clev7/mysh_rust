mod utils;
mod cli_error;

// Refer to Homework3.docx for instructions on how to write this
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Child, Command};
use utils::{tokenize, dispatch};
use cli_error::{CliError, OutOfBoundsParams};

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
            Err(CliError::InvalidUsage("Usage: history [-c]"))
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
fn replay(curr_tokens: &[&str], history: &mut Vec<String>, cwd: &mut PathBuf, children: &mut Vec<Child>) -> Result<(), CliError> {
    if curr_tokens.len() != 2  {
        return Err(CliError::BadLen(curr_tokens.len()));
    }

    let idx: usize = curr_tokens[1]
        .parse::<usize>()
        .map_err(|err| CliError::ParseError(err))?;

    if idx >= history.len() {
        return Err(CliError::OutOfBounds(OutOfBoundsParams {
            idx: idx,
            len: history.len()
        }));
    }

    let (trimmed_line, command_tokens) = tokenize(&history[idx]);
    let trimmed_line = trimmed_line.to_owned();
    let mut command_vec: Vec<String> = Vec::new();

    for token in command_tokens.iter() {
        command_vec.push(token.to_string());
    }

    let command_slice: Vec<&str> = command_vec.iter().map(|s| s.as_str()).collect();

    history.push(trimmed_line);
    // println!("{history:?} {command_vec:?}");
    dispatch(&command_slice, &mut history.clone(), cwd, children)?;

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

fn background(tokens: &[&str], children: &mut Vec<Child>) -> Result<(), CliError>{
    if tokens.len() < 2 {
        return Err(CliError::BadLen(tokens.len()));
    }

    let child = Command::new(tokens[1])
        .args(&tokens[2..])
        .spawn()
        .map_err(|err: std::io::Error| CliError::IoError(err))?;

    println!("PID: {}", child.id());

    children.push(child);
    
    Ok(())
}

fn dalek(tokens: &[&str], children: &mut Vec<Child>) -> Result<(), CliError> {
    if tokens.len() != 2 {
        return Err(CliError::BadLen(tokens.len()));
    }

    let target_pid: u32 = tokens[1].parse::<u32>()
                            .map_err(|err| CliError::ParseError(err))?;

    Command::new("kill")
        .arg("-9")
        .arg(tokens[1])
        .spawn()
        .map_err(|err| CliError::IoError(err))?
        .wait()
        .map_err(|err| CliError::IoError(err))?;

    children.retain(|child| child.id() != target_pid);

    Ok(())
}

fn dalekall(tokens: &[&str], children: &mut Vec<Child>) -> Result<(), CliError> {
    if tokens.len() != 1 {
        return Err(CliError::BadLen(tokens.len()));
    }

    if children.len() == 0 {
        eprintln!("This shell has not produced any child processes to kill");
        return Ok(())
    }

    print!("Exterminating {} processes: ", children.len());

    for child in children.iter_mut() {
        print!("{}", child.id());
        child.kill().map_err(|err: std::io::Error| CliError::IoError(err))?;
    }

    children.clear();
    Ok(())
}

fn handle_err(err: CliError) {
    use CliError::*;
    match err {
        IoError(e) => eprintln!("IOError({:?})", e),
        FileNotFound(file_path) => eprintln!("File not found: {:?}", file_path),
        BadLen(arg) => eprintln!("Incorrect length of arguments: {:?}", arg),
        ParseError(arg) => eprintln!("An error occurred while parsing argument \"{arg}\""),
        OutOfBounds(OutOfBoundsParams { idx, len }) => { 
            eprintln!("Index {idx} out of bounds for length {len}")
        },
        InvalidUsage(usage) => eprintln!("{:?}", usage)
    }
}

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

        let (trimmed_line, tokens) = utils::tokenize(line.as_str());

        if tokens[0] != "replay" {
            history.push(trimmed_line.to_string());
        }

        dispatch(&tokens, &mut history, &mut cwd, &mut children).unwrap_or_else(handle_err);

        io::stdout().flush().unwrap();
    }
}