use crate::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub squares: [SquareValue; moves::BOARD_SIZE.pow(2)],
    pub white_to_move: bool,
    pub castle_availability: Vec<CastleAvailability>,
    pub en_passant_square: Option<Square>,
    pub half_moves_since_capture_or_pawn_advance: i32,
    pub full_move_count: i32, // what move are we on
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

    /// Doesn't check if the piece can move like that, nor does it test for check.
    pub fn apply_move(&mut self, themove: &Move) -> Result<(), &str> {
        // *self = apply_move(&self, themove);
        if let Some(moving_piece) = self.piece_at(themove.from_square) {
            if moving_piece.colour != self.whos_move() {
                Err("Can't move enemy piece")
            } else {
                if let Some(capture_piece) = self.piece_at(themove.to_square) {
                    if capture_piece.colour == moving_piece.colour {
                        return Err("Can't capture friendly piece");
                    }
                }
                self.squares[themove.from_square as usize] = no!();
                self.squares[themove.to_square as usize] = SquareValue::Occupied(moving_piece);
                self.white_to_move = !self.white_to_move;
                return Ok(());
            }
        } else {
            Err("No piece at that square")
        }
    }

    pub fn count_pieces(&self) -> u8 {
        self.squares.iter().fold(0, |acc, sq| match sq {
            SquareValue::Empty => acc,
            SquareValue::Occupied(_) => acc + 1,
        })
    }

    pub fn square_is_occupied(&self, square: Square) -> bool {
        self.squares[square as usize].is_occupied()
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        match self.squares[square as usize] {
            SquareValue::Occupied(p) => Some(p),
            SquareValue::Empty => None,
        }
    }

    pub fn display(&self, from_perspective: PieceColour) -> String {
        /*
        looks like this:
        |---|---|---|---|---|---|---|---|\n
        | r | n | b | q | k | b | n | r | 8\n
        |---|---|---|---|---|---|---|---|\n
        | p | p | p | p | p | p | p | p | 7\n
        ...
        34 width * 17 height
         */
        let black_perspective = from_perspective == PieceColour::Black;
        let size = moves::BOARD_SIZE;
        // newline + ending bar + 4 per square
        let out_width = 1 + 1 + 4 * size;
        // ending bar + 2 per square
        let out_height = 1 + 2 * size;
        let status_str = &format!(
            "Move {}: {} to move\n",
            self.full_move_count,
            self.whos_move().to_string()
        );
        let white_str = "WHITE\n";
        let black_str = "black\n";
        let extra_squares_for_coords = 16;
        let str_size: usize = out_width * out_height
            + status_str.len()
            + white_str.len()
            + black_str.len()
            + extra_squares_for_coords
            + out_width;
        let mut out = String::with_capacity(str_size);
        out.push_str(if black_perspective {
            white_str
        } else {
            black_str
        });
        // IMPROVE: iterators
        let row_iter: Vec<usize> = if black_perspective {
            (1..=size).collect()
        } else {
            (1..=size).rev().collect()
        };
        let col_iter: Vec<usize> = if black_perspective {
            (1..=size).rev().collect()
        } else {
            (1..=size).collect()
        };
        for y in row_iter {
            // divider
            for _ in 1..=size {
                out.push_str("|---");
            }
            out.push_str("|\n");
            out.push_str("");
            for x in &col_iter {
                out.push_str("| ");
                let piece = self.piece_at(Square::from_coords(*x, y));
                out.push(match piece {
                    None => ' ',
                    Some(p) => p.to_letter(),
                });
                out.push(' ');
            }
            let coord_letter = size - y;
            out.push_str(&format!("| {}\n", y));
        }
        // final divider
        for _ in 1..=size {
            out.push_str("|---");
        }
        out.push_str("|\n");
        out.push('\n');
        let letter_offset = 'a' as u8 - 1;
        for i in &col_iter {
            let letter: char = char::from(*i as u8 + letter_offset);
            out.push_str(&format!("  {} ", letter));
        }
        out.push('\n');
        out.push_str(if black_perspective {
            black_str
        } else {
            white_str
        });
        out.push_str(status_str);
        return out;
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
            (Undecided(s1), GameFinished(s2)) => s1.partial_cmp(&(*s2 as i32 as f32)), // dbl cast is needed

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
                PositionEval::Undecided(score) =>
                    format!("Position has an evaluation of {}.", score),
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
