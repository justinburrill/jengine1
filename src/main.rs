#![allow(dead_code, unused_variables)]
use clap::Parser;
use std::fmt::Display;
use std::io::{self, BufRead};

pub mod evaluation;
pub mod fen;
pub mod macros;
pub mod moves;

pub mod positions;
pub use positions::*;
pub mod square;
pub use square::*;



#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SquareValue {
    #[default]
    Empty,
    Occupied(Piece),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub piece: Piece,
    pub from_square: Square,
    pub to_square: Square,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = self.piece.to_letter().map(|c| c.to_string()).unwrap_or("".to_string());
        write!(f, "{}{}", piece, self.to_square)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, clap::Subcommand)]
enum EngineMode {
    Evaluate { position: String, depth: usize },
    Interactive,
}

#[derive(Parser, Clone, Debug)]
#[command(author = "Justin Burrill", version, about = "Unbeatable chess engine")]
struct CLIArgs {
    #[command(subcommand)]
    mode: EngineMode,
}

fn main() {
    let stdin = io::stdin();
    let args = CLIArgs::parse();
    match args.mode {
        EngineMode::Evaluate { position, depth } => {
            let eval = evaluation::evaluate_position(&fen::parse(&position), depth);
            println!("{}", eval);
        }
        EngineMode::Interactive => todo!(),
    }
    for input_ in stdin.lock().lines() {
        let input_line = input_.unwrap();
    }
}
