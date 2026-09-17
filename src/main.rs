#![allow(dead_code, unused_variables)]
use clap::Parser;
use std::fmt::{Debug, Display};

pub mod evaluation;
pub mod fen;
pub mod macros;
mod math;
pub mod moves;
pub use moves::*;
pub mod positions;
pub use positions::*;
pub mod square;
pub use square::*;
pub mod interactive_mode;

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
    let args = CLIArgs::parse();
    match args.mode {
        EngineMode::Evaluate { position, depth } => {
            let eval = evaluation::evaluate_position(&fen::parse(&position), depth);
            println!("{}", eval);
        }
        EngineMode::Interactive => interactive_mode::handle_interactive(),
        EngineMode::UCI => todo!("UCI yet is not ready yet."),
    }
}
