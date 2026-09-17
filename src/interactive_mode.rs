use std::{
    collections::HashMap,
    io::{self, BufRead},
};

pub fn handle_interactive() {
    let stdin = io::stdin();
    // (command, number of args)
    let commands: HashMap<&str, u8> = vec![("new", 0), ("move", 1), ("help", 0)]
        .into_iter()
        .collect();
    loop {
        print!("> ");
        let input_line = stdin.lock().lines().next().unwrap().unwrap();
        let words: Vec<&str> = input_line.split_whitespace().collect();
        if words.len() == 0 {
            continue;
        }
        let command = words[0];
        if !commands.contains_key(command) {
            println!("Command '{}' not recognized.", command);
        }
        match command {
            "new" => (),
            _ => todo!()
        }
    }
}
