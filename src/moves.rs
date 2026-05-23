use crate::*;

pub fn get_squares_with_pieces(position: &Position, for_player: &PieceColour) -> Vec<Square> {
    let mut out: Vec<Square> = vec![];
    for (i, value) in position.squares.iter().enumerate() {
        match value {
            SquareValue::Empty => continue,
            SquareValue::Occupied(Piece { kind: _, colour }) => {
                if colour == for_player {
                    out.push(Square::from_usize(i));
                }
            }
        }
    }
    return out;
}

pub fn find_avail_moves(position: &Position) -> Vec<Move> {
    find_avail_moves_for_player(position, &position.whos_move())
}

pub fn find_avail_moves_for_player(position: &Position, to_move: &PieceColour) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for sq in get_squares_with_pieces(position, &position.whos_move()) {
        moves.extend(
            find_avail_moves_for_piece(position, sq)
                .expect("tried to find available moves for a piece that doesn't exist"),
        );
    }
    moves
}

/// Finds moves that the piece would be able to make, doesn't look for king in check, etc.
pub fn find_avail_moves_for_piece(
    position: &Position,
    location_of_piece: Square,
) -> Option<Vec<Move>> {
    use PieceColour::*;
    use PieceKind::*;
    let sqvalue = position.squares[location_of_piece as usize];
    let mut moves: Vec<Move> = vec![];
    let my_kind;
    let my_colour;
    if let SquareValue::Occupied(Piece { kind, colour }) = sqvalue {
        my_kind = kind;
        my_colour = colour;
    } else {
        return None;
    };

    let start_idx: isize = location_of_piece as isize;
    let mut move_to_index = |idx: isize| {
        moves.push(Move {
            from_square: location_of_piece,
            to_square: Square::from_usize(idx as usize),
        });
    };
    let mut move_to_indices = |idxs: Vec<isize>| {
        for idx in idxs {
            if !Square::exists(idx) {
                continue;
            }
            let target_sq = position.squares[idx as usize];
            if let SquareValue::Occupied(Piece { kind, colour }) = target_sq {
                if colour == my_colour {
                    continue;
                }
            }
            move_to_index(idx);
        }
    };
    match my_kind {
        Pawn => {
            let forward_offset = match my_colour {
                White => 1,
                Black => -1,
            };
            let capture_offsets = match my_colour {
                White => [7, 9],
                Black => [-7, -9],
            };
            let push_square = start_idx + (8 * forward_offset);
            move_to_index(push_square);
            for o in capture_offsets {
                //  TODO: en passant
                if position.squares[(start_idx + o) as usize]
                    .is_occupied_by_colour(my_colour.other())
                {
                    move_to_index(start_idx + o)
                }
            }
        }
        King => move_to_indices(vec![
            start_idx + 1,
            start_idx - 1,
            start_idx + 8,
            start_idx - 8,
            start_idx + 7,
            start_idx - 7,
            start_idx + 9,
            start_idx - 9,
        ]),
        Knight => move_to_indices(vec![
            start_idx + 15,
            start_idx + 17,
            start_idx - 15,
            start_idx - 17,
            start_idx + 10,
            start_idx - 10,
            start_idx + 6,
            start_idx - 6,
        ]),
        _ => todo!(),
    }
    return Some(moves);
}


pub fn move_is_valid(position: &Position, themove: &Move) -> bool {
    let piece_is_there = position.squares[themove.from_square as usize].is_occupied();
    let piece_can_move_like_that = find_avail_moves_for_piece(position, themove.from_square)
        .expect("tried to find available moves for a piece that doesn't exist")
        .contains(themove);
    piece_is_there && piece_can_move_like_that
}

/// Assumes that the move is valid
pub fn apply_move(position: &Position, themove: &Move) -> Position {
    let mut new_pos = (*position).clone();
    let piece = match new_pos.squares[themove.from_square as usize] {
        SquareValue::Occupied(p) => p,
        _ => panic!("Tried to apply invalid move, piece not present at from_square"),
    };
    new_pos.squares[themove.from_square as usize] = no!();
    new_pos.squares[themove.to_square as usize] = SquareValue::Occupied(piece);
    new_pos
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn valid_moves() {
        assert!(move_is_valid(
            &starting_position!(),
            &Move {
                from_square: Square::E2,
                to_square: Square::E4
            }
        ));
    }

    #[test]
    fn test_find_king_moves() {
        let pos = fen::parse("8/8/8/3K4/8/8/8/8 w - - 0 1");
        let piece = Piece {
            kind: PieceKind::King,
            colour: PieceColour::White,
        };
        let from_square = Square::D5;

        let mut expected_moves: Vec<Move> = vec![
            Square::D4,
            Square::D6,
            Square::E4,
            Square::E5,
            Square::E6,
            Square::C4,
            Square::C5,
            Square::C6,
        ]
        .iter()
        .map(|sq| Move {
            from_square,
            to_square: *sq,
        })
        .collect::<Vec<Move>>();
        expected_moves.sort();
        let mut result = moves::find_avail_moves_for_piece(&pos, from_square).unwrap();
        result.sort();
        assert_eq!(result, expected_moves);
    }
}
