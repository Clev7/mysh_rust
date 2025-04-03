use std::path::PathBuf;
use crate::{
    background, 
    dalek, 
    get_history, 
    movetodir, 
    replay, 
    start, 
    whereami, 
    CliError
};

pub fn tokenize(line: &str) -> (&str, Vec<&str>) {
    let trimmed_line: &str = line.trim();
    let tokens: Vec<&str> = trimmed_line.split(' ').collect();

    return (trimmed_line, tokens);
}

pub fn dispatch(tokens: &[&str], history: &mut Vec<String>, cwd: &mut PathBuf) -> Result<(), CliError> {
    match tokens[0] {
        "movetodir" => movetodir(&tokens, cwd)?,
        "whereami" => whereami(&tokens, cwd)?,
        "replay" => replay(tokens, history, cwd)?,
        "start" => start(&tokens)?,
        "background" => background(&tokens)?,
        "dalek" => dalek(&tokens)?,
        "history" => get_history(&tokens, history)?,
        "byebye" => std::process::exit(0),
        _ => eprintln!("{}: command not found", tokens[0]),
    }

    Ok(())
}