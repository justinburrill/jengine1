use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
};

use crate::*;

struct InteractiveCommandInfo {
    argc: usize,
    help_text: &'static str,
}

type Commands = HashMap<&'static str, InteractiveCommandInfo>;

fn get_commands() -> Commands {
    vec![
        (
            "new",
            InteractiveCommandInfo {
                argc: 0,
                help_text: "Reset the board",
            },
        ),
        (
            "move",
            InteractiveCommandInfo {
                argc: 1,
                help_text: "Enter a move like 'e2e4'",
            },
        ),
        (
            "help",
            InteractiveCommandInfo {
                argc: 0,
                help_text: "Print help",
            },
        ),
        (
            "print",
            InteractiveCommandInfo {
                argc: 0,
                help_text: "Print the board",
            },
        ),
        (
            "flip",
            InteractiveCommandInfo {
                argc: 0,
                help_text: "Flip the perspective",
            },
        ),
        (
            "exit",
            InteractiveCommandInfo {
                argc: 0,
                help_text: "Close program",
            },
        ),
    ]
    .into_iter()
    .collect()
}

fn verify_cmd(input_words: &Vec<&str>, cmds: &Commands) -> Result<(), String> {
    let command = input_words[0];
    if !cmds.contains_key(command) {
        return Err(format!("Command '{}' not recognized.", command));
    }
    let cmd_info = cmds.get(command).unwrap();
    if input_words.len() - 1 != cmd_info.argc {
        return Err(format!(
            "Expected {} arg(s) to command '{}'",
            cmd_info.argc, command
        ));
    }
    return Ok(());
}

pub fn handle_interactive() {
    let stdin = io::stdin();
    // (command, number of args)
    let mut position: Position = starting_position!();
    let mut perspective = PieceColour::White;
    let commands = get_commands();
    println!("Welcome to interactive mode.");
    println!("{}", position.display(perspective));
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let input_line = stdin.lock().lines().next().unwrap().unwrap();
        let words: Vec<&str> = input_line.split_whitespace().collect();
        if words.len() == 0 {
            continue; // empty line, just prompt again
        }
        if let Err(msg) = verify_cmd(&words, &commands) {
            println!("{}", msg);
            continue;
        }
        let command = words[0];
        match command {
            "new" => {
                position = starting_position!();
                println!("{}", position.display(perspective));
            }
            "move" => {
                if let Some(inputted_move) = Move::from_str(words[1]) {
                    let result = position.apply_move(&inputted_move);
                    if let Err(text) = result {
                        println!("Invalid move: {}", text);
                    }
                } else {
                    println!("Improper move: {}", words[1]);
                }
                println!("{}", position.display(perspective));
            }
            "help" => {
                let commands_text = commands.iter().fold(String::new(), |acc, (s, info)| {
                    format!("{}\n{}: ({} args) {}", acc, s, info.argc, info.help_text)
                });
                println!("Commands:\n{}", commands_text);
            }
            "print" => {
                println!("{}", position.display(perspective));
            }
            "flip" => {
                perspective = perspective.other();
                println!("{}", position.display(perspective));
            }
            "exit" => {
                return;
            }
            _ => todo!(),
        }
    }
}
