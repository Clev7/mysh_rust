use std::{path::PathBuf, process::Child};
use crate::{
    background, 
    dalek, 
    get_history, 
    movetodir, 
    replay, 
    start, 
    whereami, 
    dalekall,
    CliError
};

pub fn tokenize(line: &str) -> (&str, Vec<&str>) {
    let trimmed_line: &str = line.trim();
    let tokens: Vec<&str> = trimmed_line.split(' ').collect();

    return (trimmed_line, tokens);
}

pub fn dispatch(tokens: &[&str], history: &mut Vec<String>, cwd: &mut PathBuf, children: &mut Vec<Child>) -> Result<(), CliError> {
    match tokens[0] {
        "movetodir" => movetodir(&tokens, cwd)?,
        "whereami" => whereami(&tokens, cwd)?,
        "replay" => replay(tokens, history, cwd, children)?,
        "start" => start(&tokens)?,
        "background" => background(&tokens, children)?,
        "dalek" => dalek(&tokens, children)?,
        "dalekall" => dalekall(&tokens, children)?,
        "history" => get_history(&tokens, history)?,
        // TODO: Make a more robust help where you can pass specific commands as args
        "help" => {
            println!("These shell commands are defined internally. Type 'help' to see this list.");
            println!("Type 'help name' to find out more about the function 'name'.");
            println!();
            println!("{:<30}\t\t\tmoves the current directory to the file path if valid.", "movetodir [filepath]");
            println!("{:<30}\t\t\tprints the current working directory.", "whereami");
            println!("{:<30}\t\t\tPrints a numbered list of all previously executed commands. Use -c to clear.", "history [-c]");
            println!("{:<30}\t\t\tExits the terminal.", "byebye");
            println!("{:<30}\t\t\tExecutes the nth command from history. ", "replay [n]");
            println!("{:<30}\t\t\tRuns the specified program, applying any passed arguments to it, then waits for it to finish.", "start program [args]");
            println!("{:<30}\t\t\tSimilar to start but runs the specified program in the background.", "background [program] [args]");
            println!("{:<30}\t\t\tKills the process with PID as its PID. The PID can be from a child of this terminal process or any other PID listed on this system.", "dalek [PID]");
            println!("{:<30}\t\t\texecutes command, applying any passed arguments to it, n times.", "repeat [n] command [args]");
            println!("{:<30}\t\t\tKills all child processes spawned by this terminal.", "dalekall");
        }
        "byebye" => std::process::exit(0),
        _ => eprintln!("{}: command not found", tokens[0]),
    }

    Ok(())
}