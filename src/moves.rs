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
