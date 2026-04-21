#![allow(dead_code, unused_variables)]
use clap::Parser;
use std::fmt::Display;
use std::io::{self, BufRead};

pub mod evaluation;
pub mod fen;
pub mod macros;
pub mod moves;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Square {
    A1 = 0,
    A2 = 1,
    A3 = 2,
    A4 = 3,
    A5 = 4,
    A6 = 5,
    A7 = 6,
    A8 = 7,
    B1 = 8,
    B2 = 9,
    B3 = 10,
    B4 = 11,
    B5 = 12,
    B6 = 13,
    B7 = 14,
    B8 = 15,
    C1 = 16,
    C2 = 17,
    C3 = 18,
    C4 = 19,
    C5 = 20,
    C6 = 21,
    C7 = 22,
    C8 = 23,
    D1 = 24,
    D2 = 25,
    D3 = 26,
    D4 = 27,
    D5 = 28,
    D6 = 29,
    D7 = 30,
    D8 = 31,
    E1 = 32,
    E2 = 33,
    E3 = 34,
    E4 = 35,
    E5 = 36,
    E6 = 37,
    E7 = 38,
    E8 = 39,
    F1 = 40,
    F2 = 41,
    F3 = 42,
    F4 = 43,
    F5 = 44,
    F6 = 45,
    F7 = 46,
    F8 = 47,
    G1 = 48,
    G2 = 49,
    G3 = 50,
    G4 = 51,
    G5 = 52,
    G6 = 53,
    G7 = 54,
    G8 = 55,
    H1 = 56,
    H2 = 57,
    H3 = 58,
    H4 = 59,
    H5 = 60,
    H6 = 61,
    H7 = 62,
    H8 = 63,
}

impl Square {
    pub fn from_usize(int: usize) -> Square {
        match int {
            0 => Square::A1,
            1 => Square::A2,
            2 => Square::A3,
            3 => Square::A4,
            4 => Square::A5,
            5 => Square::A6,
            6 => Square::A7,
            7 => Square::A8,
            8 => Square::B1,
            9 => Square::B2,
            10 => Square::B3,
            11 => Square::B4,
            12 => Square::B5,
            13 => Square::B6,
            14 => Square::B7,
            15 => Square::B8,
            16 => Square::C1,
            17 => Square::C2,
            18 => Square::C3,
            19 => Square::C4,
            20 => Square::C5,
            21 => Square::C6,
            22 => Square::C7,
            23 => Square::C8,
            24 => Square::D1,
            25 => Square::D2,
            26 => Square::D3,
            27 => Square::D4,
            28 => Square::D5,
            29 => Square::D6,
            30 => Square::D7,
            31 => Square::D8,
            32 => Square::E1,
            33 => Square::E2,
            34 => Square::E3,
            35 => Square::E4,
            36 => Square::E5,
            37 => Square::E6,
            38 => Square::E7,
            39 => Square::E8,
            40 => Square::F1,
            41 => Square::F2,
            42 => Square::F3,
            43 => Square::F4,
            44 => Square::F5,
            45 => Square::F6,
            46 => Square::F7,
            47 => Square::F8,
            48 => Square::G1,
            49 => Square::G2,
            50 => Square::G3,
            51 => Square::G4,
            52 => Square::G5,
            53 => Square::G6,
            54 => Square::G7,
            55 => Square::G8,
            56 => Square::H1,
            57 => Square::H2,
            58 => Square::H3,
            59 => Square::H4,
            60 => Square::H5,
            61 => Square::H6,
            62 => Square::H7,
            63 => Square::H8,
            _ => panic!("Invalid square index passed: {}", int),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Square::A1 => "A1",
                Square::A2 => "A2",
                Square::A3 => "A3",
                Square::A4 => "A4",
                Square::A5 => "A5",
                Square::A6 => "A6",
                Square::A7 => "A7",
                Square::A8 => "A8",
                Square::B1 => "B1",
                Square::B2 => "B2",
                Square::B3 => "B3",
                Square::B4 => "B4",
                Square::B5 => "B5",
                Square::B6 => "B6",
                Square::B7 => "B7",
                Square::B8 => "B8",
                Square::C1 => "C1",
                Square::C2 => "C2",
                Square::C3 => "C3",
                Square::C4 => "C4",
                Square::C5 => "C5",
                Square::C6 => "C6",
                Square::C7 => "C7",
                Square::C8 => "C8",
                Square::D1 => "D1",
                Square::D2 => "D2",
                Square::D3 => "D3",
                Square::D4 => "D4",
                Square::D5 => "D5",
                Square::D6 => "D6",
                Square::D7 => "D7",
                Square::D8 => "D8",
                Square::E1 => "E1",
                Square::E2 => "E2",
                Square::E3 => "E3",
                Square::E4 => "E4",
                Square::E5 => "E5",
                Square::E6 => "E6",
                Square::E7 => "E7",
                Square::E8 => "E8",
                Square::F1 => "F1",
                Square::F2 => "F2",
                Square::F3 => "F3",
                Square::F4 => "F4",
                Square::F5 => "F5",
                Square::F6 => "F6",
                Square::F7 => "F7",
                Square::F8 => "F8",
                Square::G1 => "G1",
                Square::G2 => "G2",
                Square::G3 => "G3",
                Square::G4 => "G4",
                Square::G5 => "G5",
                Square::G6 => "G6",
                Square::G7 => "G7",
                Square::G8 => "G8",
                Square::H1 => "H1",
                Square::H2 => "H2",
                Square::H3 => "H3",
                Square::H4 => "H4",
                Square::H5 => "H5",
                Square::H6 => "H6",
                Square::H7 => "H7",
                Square::H8 => "H8",
            },
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastleAvailability {
    BlackKingside,
    BlackQueenside,
    WhiteKingside,
    WhiteQueenside,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

impl PieceKind {
    pub fn piece_value(self: &PieceKind) -> usize {
        match self {
            PieceKind::King => 0,
            PieceKind::Queen => 9,
            PieceKind::Rook => 5,
            PieceKind::Bishop => 3,
            PieceKind::Knight => 3,
            PieceKind::Pawn => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceColour {
    White,
    Black,
}

impl PieceColour {
    pub fn other(self: &PieceColour) -> PieceColour {
        match self {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositionEval {
    MateForWhite(u32),
    MateForBlack(u32),
    Undecided(f32),
    GameFinished(FinishedState),
}

impl Display for PositionEval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionEval::MateForWhite(n) => format!("Mate for white in {} moves.", n),
                PositionEval::MateForBlack(n) => format!("Mate for black in {} moves.", n),
                PositionEval::Undecided(score) => format!("Position has an evaluation of {}.", score),
                PositionEval::GameFinished(state) => match state {
                    FinishedState::Stalemate => format!("The position is a stalemate."),
                    FinishedState::WhiteVictory => format!("The position checkmate for white."),
                    FinishedState::BlackVictory => format!("The position checkmate for black."),
                },
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FinishedState {
    BlackVictory = -999999999,
    Stalemate = 0,
    WhiteVictory = 999999999,
}

impl PartialOrd for FinishedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as isize).partial_cmp(&(*other as isize))
    }
}

impl Ord for FinishedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for PositionEval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use PositionEval::*;
        use std::cmp::Ordering::{Greater, Less};
        match (self, other) {
            (GameFinished(s1), GameFinished(s2)) => s1.partial_cmp(s2),

            (MateForWhite(_), GameFinished(FinishedState::WhiteVictory)) => Some(Less),
            (MateForWhite(n), MateForWhite(m)) => m.partial_cmp(n),
            (MateForWhite(_), _) => Some(Greater),

            (MateForBlack(_), GameFinished(FinishedState::BlackVictory)) => Some(Greater),
            (MateForBlack(n), MateForBlack(m)) => n.partial_cmp(m),
            (MateForBlack(_), _) => Some(Less),

            (Undecided(s1), Undecided(s2)) => s1.partial_cmp(s2),
            (Undecided(s1), GameFinished(s2)) => s1.partial_cmp(&(*s2 as i32 as f32)),

            (b, a) => a.partial_cmp(b).map(|ord| ord.reverse()),
        }
    }
}

impl Eq for PositionEval {}

impl Ord for PositionEval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub colour: PieceColour,
}

impl Piece {
    pub fn to_letter(&self) -> Option<char> {
        match self.kind {
            PieceKind::Rook => Some('R'),
            PieceKind::Bishop => Some('B'),
            PieceKind::Queen => Some('Q'),
            PieceKind::King => Some('K'),
            PieceKind::Knight => Some('N'),
            PieceKind::Pawn => None,
        }
    }
}

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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub white_to_move: bool,
    pub full_move_count: i32, // what move are we on
    pub half_moves_since_capture_or_pawn_advance: i32,
    pub castle_availability: Vec<CastleAvailability>,
    pub squares: [SquareValue; 64],
}

impl Position {
    pub fn whos_move(&self) -> PieceColour {
        if self.white_to_move {
            PieceColour::White
        } else {
            PieceColour::Black
        }
    }

    pub fn find_piece(&self, piece: &Piece) -> Option<Square> {
        for (i, square) in self.squares.iter().enumerate() {
            match square {
                SquareValue::Empty => continue,
                SquareValue::Occupied(p) => {
                    if p == piece {
                        return Some(Square::from_usize(i));
                    }
                }
            }
        }
        return None;
    }

    pub fn apply_move(&mut self, themove: &Move) {
        todo!()
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
