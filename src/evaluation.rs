use crate::moves;
use crate::*;

pub fn is_king_in_check(position: &Position, which_king: &PieceColour) -> bool {
    let available_moves = moves::find_avail_moves(position);
    let enemy_king_square = position.find_piece(&Piece {
        kind: PieceKind::King,
        colour: *which_king,
    });
    if enemy_king_square.is_none() {
        return false;
    }
    for Move {
        piece,
        from_square,
        to_square,
    } in available_moves
    {
        if to_square == enemy_king_square.unwrap() {
            return true;
        }
    }
    return false;
}

/// Check to see if a position is currently a checkmate
pub fn check_for_mate(position: &Position) -> Option<FinishedState> {
    // FIXME: Don't run find_available_moves all the time... maybe store in Position struct?
    // or a PositionWithMoves struct?
    if check_if_mated(position, &PieceColour::White) {
        Some(FinishedState::BlackVictory)
    } else if check_if_mated(position, &PieceColour::Black) {
        Some(FinishedState::WhiteVictory)
    } else {
        None
    }
}

pub fn check_if_mated(position: &Position, which_king: &PieceColour) -> bool {
    is_king_in_check(position, which_king) && moves::find_avail_moves_for_player(position, which_king).len() == 0
}

/// Raw difference in piece points
pub fn evaluate_raw_material_difference(position: &Position) -> isize {
    let mut white_score = 0;
    let mut black_score = 0;
    for square in position.squares {
        match square {
            SquareValue::Empty => continue,
            SquareValue::Occupied(Piece { kind, colour }) => {
                if colour == PieceColour::White {
                    white_score += kind.piece_value();
                } else {
                    black_score += kind.piece_value();
                }
            }
        }
    }
    return (white_score - black_score).try_into().unwrap();
}

pub fn evaluate_adjusted_material_difference(position: &Position) -> f32 {
    let mut white_score: f32 = 0.0;
    let mut black_score: f32 = 0.0;
    for (square_idx, squarevalue) in position.squares.iter().enumerate() {
        let square = Square::from_usize(square_idx);
        match squarevalue {
            SquareValue::Empty => continue,
            SquareValue::Occupied(Piece { kind, colour }) => {
                let value: f32 = match kind {
                    PieceKind::Pawn => {
                        let base = kind.piece_value() as f32;
                        let squares_pushed = 6 - square.moves_from_back_rank(colour);
                        let distance_bonus = 0.15 * ( squares_pushed as f32 );
                        let center_bonus = 0.1 * (Square::distance_from_center(&square) as f32);
                        base + distance_bonus + center_bonus
                    },
                    PieceKind::Knight => {
                        let base = kind.piece_value() as f32;
                        let center_bonus = 0.3 * (Square::distance_from_center(&square) as f32);
                        base + center_bonus
                    }
                    _ => kind.piece_value() as f32,
                };

                if *colour == PieceColour::White {
                    white_score += value;
                } else {
                    black_score += value;
                }
            }
        }
    }
    return (white_score - black_score).try_into().unwrap();
}

/// Returns the best move and the resulting position
pub fn find_best_move(position: &Position, depth: usize) -> (Move, PositionEval, Position) {
    let mut positions: Vec<(Move, Position)> = vec![];
    let moves = moves::find_avail_moves(position);
    for themove in moves {
        let mut nextpos = position.clone();
        moves::apply_move(&mut nextpos, &themove);
        positions.push((themove, nextpos));
    }
    positions
        .into_iter()
        .map(|(move_, pos)| (move_, evaluate_position(&pos, depth), pos))
        .max_by_key(|(_, eval, _)| *eval)
        .expect("Passed no moves to find_best_move")
}

pub fn evaluate_position(position: &Position, depth: usize) -> PositionEval {
    match check_for_mate(position) {
        None => {
            if depth == 0 {
                // couldn't figure out a mate
                PositionEval::Undecided(evaluate_raw_material_difference(position) as f32)
            } else {
                let (move_, eval, resulting_pos) = find_best_move(position, depth);
                return evaluate_position(&resulting_pos, depth - 1);
            }
        }
        Some(state) => PositionEval::GameFinished(state),
    }
}

#[cfg(test)]
mod tests {

    mod position_comparison {
        use crate::{FinishedState, PositionEval};

        #[test]
        fn compare_mate_for_white() {
            let pos1 = PositionEval::MateForWhite(1);
            let pos2 = PositionEval::MateForWhite(5);
            let pos3 = PositionEval::MateForWhite(10);
            let pos4 = PositionEval::MateForWhite(15);
            assert!(pos1 > pos2);
            assert!(pos2 > pos3);
            assert!(pos3 > pos4);

            assert!(pos4 < pos3);
            assert!(pos3 < pos2);
            assert!(pos2 < pos1);
        }

        #[test]
        fn compare_mate_for_black() {
            let pos1 = PositionEval::MateForBlack(1);
            let pos2 = PositionEval::MateForBlack(5);
            let pos3 = PositionEval::MateForBlack(10);
            let pos4 = PositionEval::MateForBlack(15);
            assert!(pos1 < pos2);
            assert!(pos2 < pos3);
            assert!(pos3 < pos4);

            assert!(pos4 > pos3);
            assert!(pos3 > pos2);
            assert!(pos2 > pos1);
        }

        #[test]
        fn compare_undecided_position() {
            let pos1 = PositionEval::Undecided(1.0);
            let pos2 = PositionEval::Undecided(5.5);
            let pos3 = PositionEval::Undecided(-5.2);
            let pos4 = PositionEval::Undecided(-10.0);
            assert!(pos1 < pos2);
            assert!(pos2 > pos3);
            assert!(pos3 > pos4);
            assert!(pos4 < pos3);
            assert!(pos3 < pos2);
            assert!(pos2 > pos1);
        }

        #[test]
        fn compare_mixed_positions() {
            let pos1 = PositionEval::Undecided(3.8);
            let pos2 = PositionEval::Undecided(-5.2);
            let pos3 = PositionEval::Undecided(-1.0);
            let pos4 = PositionEval::MateForBlack(1);
            let pos5 = PositionEval::MateForBlack(5);
            let pos6 = PositionEval::MateForWhite(1);
            let pos7 = PositionEval::MateForWhite(5);
            assert!(pos1 > pos2);
            assert!(pos2 < pos3);
            assert!(pos3 > pos4);
            assert!(pos4 < pos5);
            assert!(pos5 < pos6);
            assert!(pos6 > pos7);
        }

        #[test]
        fn compare_finished_games() {
            let stale = FinishedState::Stalemate;
            let black = FinishedState::BlackVictory;
            let white = FinishedState::WhiteVictory;
            assert!((-1).partial_cmp(&1) == Some(std::cmp::Ordering::Less));
            assert!(black < white);
            assert!(white > black);
            assert!(white == white);
            assert!(black == black);
            assert!(stale == stale);
            assert!(black < stale);
            assert!(white > stale);
            assert!(stale < white);
            assert!(stale > black);
        }

        #[test]
        fn compare_mixed_positions_and_finished_games() {
            let white_winning = PositionEval::Undecided(3.8);
            let white_really_winning = PositionEval::Undecided(50.7);
            let black_really_winning = PositionEval::Undecided(-5.2);
            let black_kinda_winning = PositionEval::Undecided(-1.0);
            let mate4black = PositionEval::MateForBlack(1);
            let mate4white = PositionEval::MateForWhite(1);
            let stalemate = PositionEval::GameFinished(FinishedState::Stalemate);
            let black_won = PositionEval::GameFinished(FinishedState::BlackVictory);
            let white_won = PositionEval::GameFinished(FinishedState::WhiteVictory);
            assert!(white_winning < white_won);
            assert!(white_won > white_winning);
            assert!(black_kinda_winning < white_won);
            assert!(white_won > black_kinda_winning);
            assert!(white_won > black_really_winning);
            assert!(white_won > mate4white);
            assert!(white_won > stalemate);
            assert!(mate4white < white_won);
            assert!(stalemate < white_won);
            assert!(white_winning > black_won);
            assert!(white_winning > stalemate);
            assert!(black_won < white_winning);
            assert!(stalemate < white_winning);
            assert!(black_kinda_winning < stalemate);
            assert!(stalemate > black_kinda_winning);
            assert!(mate4black < stalemate);
            assert!(mate4white > mate4black);
            assert!(mate4black < mate4white);
        }
    }

    mod position_extras {

    }

    mod shallow_evaluation {
        use crate::{PieceColour, evaluation, fen};
        #[test]
        fn test_king_in_check() {
            let pos = fen::parse("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3");
            assert!(evaluation::is_king_in_check(&pos, &PieceColour::White));
            assert!(!evaluation::is_king_in_check(&pos, &PieceColour::Black));
        }
    }

    mod find_best_move {
        use crate::{PositionEval, evaluation, fen};
        #[test]
        fn detect_fools_mate() {
            let pos = fen::parse("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2");
            let evaluation = evaluation::evaluate_position(&pos, 2);
            assert_eq!(evaluation, PositionEval::MateForBlack(1));
        }
    }
}
