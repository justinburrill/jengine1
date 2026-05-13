use crate::*;

pub fn get_squares_with_pieces(position: &Position, for_player: &PieceColour) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    for (i, sq) in position.squares.iter().enumerate() {
        match sq {
            SquareValue::Empty => {}
            SquareValue::Occupied(Piece { kind: _, colour }) => {
                if colour == for_player {
                    out.push(i);
                }
            }
        }
    }
    return out;
}

pub fn find_available_moves(position: &Position) -> Vec<Move> {
    find_available_moves_for(position, &position.whos_move())
}

pub fn find_available_moves_for(position: &Position, to_move: &PieceColour) -> Vec<Move> {
    for sq_i in get_squares_with_pieces(position, &position.whos_move()) {}
    todo!()
}

pub fn move_is_valid(position: &mut Position, themove: &Move) -> bool {
    todo!()
}

pub fn apply_move(position: &Position, themove: &Move) -> Position {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::moves::*;

    fn test_moves_from_back_rank() {
        assert_eq!(
            Square::moves_from_back_rank(&Square::A1, &PieceColour::White),
            0
        );
        assert_eq!(
            Square::moves_from_back_rank(&Square::A1, &PieceColour::Black),
            7
        );
        assert_eq!(
            Square::moves_from_back_rank(&Square::E4, &PieceColour::White),
            3
        );
        assert_eq!(
            Square::moves_from_back_rank(&Square::G6, &PieceColour::Black),
            2
        );
    }
}
