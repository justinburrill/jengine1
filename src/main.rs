#![allow(dead_code, unused_variables)]
use clap::Parser;
use std::fmt::Display;
use std::io::{self, BufRead};

mod math;
pub mod evaluation;
pub mod fen;
pub mod macros;
pub mod moves;
pub use moves::*;
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

impl SquareValue {
    pub fn is_occupied(&self) -> bool {
        match self {
            SquareValue::Occupied(_) => true,
            SquareValue::Empty => false,
        }
    }

    pub fn is_occupied_by_colour(&self, colour: PieceColour) -> bool {
        match self {
            SquareValue::Occupied(p) => p.colour == colour,
            SquareValue::Empty => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from_square: Square,
    pub to_square: Square,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.from_square.partial_cmp(&other.from_square) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.to_square.partial_cmp(&other.to_square)
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("failed to order some moves")
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from_square, self.to_square)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, clap::Subcommand)]
enum EngineMode {
    Evaluate { position: String, depth: usize },
    UCI,
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
        EngineMode::Interactive => todo!("Interactive mode is not ready yet."),
        EngineMode::UCI => todo!("UCI yet is not ready yet."),
    }
    for input_ in stdin.lock().lines() {
        let input_line = input_.unwrap();
    }
}
