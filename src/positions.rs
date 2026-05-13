use crate::*;

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

    pub fn count_pieces(&self) -> usize {
        self.squares.iter().fold(0, |acc, sq| match sq {
            SquareValue::Empty => acc,
            SquareValue::Occupied(_) => acc + 1,
        })
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastleAvailability {
    BlackKingside,
    BlackQueenside,
    WhiteKingside,
    WhiteQueenside,
}
