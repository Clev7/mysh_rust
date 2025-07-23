use std::{path::PathBuf, process::Child, process::Command};
use std::fs::{canonicalize, read_to_string};

pub mod error_handling;
pub mod tokenize;

mod tests;

use error_handling::{CliError, OutOfBoundsParams};
use tokenize::tokenize;

fn whereami(tokens: &[&str], cwd: &PathBuf) -> Result<(), CliError> {
    if tokens.len() > 1 {
        Err(CliError::BadLen(tokens.len()))
    } else {
        println!("{}", cwd.to_str().unwrap());

        Ok(())
    }
}

fn diff(tokens: &[&str]) -> Result<(), CliError> {
    if tokens.len() != 3 {
        Err(CliError::BadLen(tokens.len()))
    } else {
        // We should be doing File I/O here.

        println!("f1: {:?}", canonicalize(tokens[1]));
        println!("f2: {:?}", canonicalize(tokens[2]));

        let f1_contents = read_to_string(tokens[0]).map_err(|err| CliError::IoError(err))?;
        let f2_contents = read_to_string(tokens[1]).map_err(|err| CliError::IoError(err))?;

        println!("{}", f1_contents);
        println!("{}", f2_contents);

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
fn replay(
    curr_tokens: &[&str],
    history: &mut Vec<String>,
    cwd: &mut PathBuf,
    children: &mut Vec<Child>,
) -> Result<(), CliError> {
    if curr_tokens.len() != 2 {
        return Err(CliError::BadLen(curr_tokens.len()));
    }

    let idx: usize = curr_tokens[1]
        .parse::<usize>()
        .map_err(|err| CliError::ParseError(err))?;

    if idx >= history.len() {
        return Err(CliError::OutOfBounds(OutOfBoundsParams {
            idx,
            len: history.len(),
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

    child
        .wait()
        .map_err(|err: std::io::Error| CliError::IoError(err))?;

    Ok(())
}

fn background(tokens: &[&str], children: &mut Vec<Child>) -> Result<(), CliError> {
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

    let target_pid: u32 = tokens[1]
        .parse::<u32>()
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
        return Ok(());
    }

    print!("Exterminating {} processes: ", children.len());

    for child in children.iter_mut() {
        print!("{} ", child.id());
        child
            .kill()
            .map_err(|err: std::io::Error| CliError::IoError(err))?;
    }
    println!();

    children.clear();
    Ok(())
}

// TODO: Implement this
fn ls(tokens: &[&str]) -> Result<(), CliError> {
    let mut command = Command::new("ls");

    for &tok in tokens {
        command.arg(tok);
    }

    command
        .spawn()
        .map_err(|err| CliError::IoError(err))?
        .wait()
        .map_err(|err| CliError::IoError(err))?;

    Ok(())
}

pub fn movetodir(tokens: &[&str], cwd: &mut PathBuf) -> Result<(), CliError> {
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

pub fn dispatch(
    tokens: &[&str],
    history: &mut Vec<String>,
    cwd: &mut PathBuf,
    children: &mut Vec<Child>,
) -> Result<(), CliError> {
    match tokens[0] {
        "movetodir" => movetodir(tokens, cwd)?,
        "ls" => ls(tokens)?,
        "whereami" => whereami(tokens, cwd)?,
        "replay" => replay(tokens, history, cwd, children)?,
        "start" => start(tokens)?,
        "background" => background(tokens, children)?,
        "dalek" => dalek(tokens, children)?,
        "dalekall" => dalekall(tokens, children)?,
        "history" => get_history(tokens, history)?,
        "diff" => diff(tokens)?,
        // TODO: Make a more robust help where you can pass specific commands as args
        "help" => {
            println!("These shell commands are defined internally. Type 'help' to see this list.");
            println!("Type 'help name' to find out more about the function 'name'.");
            println!();
            println!(
                "{:<30}\t\t\tMoves the current directory to the file path if valid.",
                "movetodir [filepath]"
            );
            println!(
                "{:<30}\t\t\tPrints the current working directory.",
                "whereami"
            );
            println!("{:<30}\t\t\tPrints a numbered list of all previously executed commands. Use -c to clear.", "history [-c]");
            println!("{:<30}\t\t\tExits the terminal.", "byebye");
            println!(
                "{:<30}\t\t\tExecutes the nth command from history. ",
                "replay [n]"
            );
            println!("{:<30}\t\t\tRuns the specified program, applying any passed arguments to it, then waits for it to finish.", "start program [args]");
            println!(
                "{:<30}\t\t\tSimilar to start but runs the specified program in the background.",
                "background program [args]"
            );
            println!("{:<30}\t\t\tKills the process with PID as its PID. The PID can be from a child of this terminal process or any other PID listed on this system.", "dalek [PID]");
            println!(
                "{:<30}\t\t\tExecutes command, applying any passed arguments to it, n times.",
                "repeat [n] command [args]"
            );
            println!(
                "{:<30}\t\t\tKills all child processes spawned by this terminal.",
                "dalekall"
            );
        },
        "byebye" => std::process::exit(0),
        "" => (),
        _ => eprintln!("{}: command not found", tokens[0]),
    }

    Ok(())
}
