use crate::*;
use std::cmp::min;

pub(crate) const BOARD_SIZE: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from_square, self.to_square)
    }
}

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

fn get_steps_in_direction(start_square: Square, step_x: isize, step_y: isize) -> Vec<isize> {
    let size = BOARD_SIZE as isize;
    let (x, y) = start_square.to_coords();
    let max_x_steps = if step_x < 0 { x } else { BOARD_SIZE as u8 - x };
    let max_y_steps = if step_y < 0 { y } else { BOARD_SIZE as u8 - y };
    let max_steps = min(max_x_steps, max_y_steps);
    let diff: isize = step_y * size + step_x;
    let mut out: Vec<isize> = Vec::with_capacity(max_steps as usize);

    for i in 1..=max_steps {
        out.push(i as isize * diff);
    }
    return out;
}

fn get_king_moves(square: Square) -> Vec<isize> {
    let s = BOARD_SIZE as isize;
    // let out = Vec::with_capacity(8);
    match (
        square.is_left_edge(),
        square.is_right_edge(),
        square.is_top_edge(),
        square.is_bottom_edge(),
    ) {
        (false, false, false, false) => vec![1, -1, s, -s, s - 1, -s - 1, s + 1, -s + 1],
        (true, false, false, false) => vec![1, s, -s, -s - 1, s + 1],
        (false, true, false, false) => vec![-1, s, -s, s - 1, -s + 1],
        (false, false, true, false) => vec![1, -1, -s, -s - 1, -s + 1],
        (false, false, false, true) => vec![1, -1, s, s - 1, s + 1],
        (true, false, false, true) => vec![1, s, s + 1],
        (l,r,t,b) => panic!("Invalid edge combination somehow: {}, {}, {}, {}", l, r, t, b),
    }
}

fn get_knight_moves(start_square: Square) -> Vec<isize> {
    let size = BOARD_SIZE as isize;
    vec![
        2 * size + 1,
        2 * size - 1,
        -2 * size + 1,
        -2 * size - 1,
        size + 2,
        size - 2,
        -size + 2,
        -size - 2,
    ]
}

/// Not pawns, colour agnostic
pub fn get_move_pattern(piece: PieceKind, start_square: Square) -> Vec<isize> {
    use PieceKind::*;
    let size = BOARD_SIZE as isize;
    // TODO: knight and king need fixing, will go off the board
    match piece {
        King => get_king_moves(start_square),
        Knight => get_knight_moves(start_square),
        Rook => {
            let mut r = get_steps_in_direction(start_square, 1, 0);
            r.extend(get_steps_in_direction(start_square, 0, 1));
            r.extend(get_steps_in_direction(start_square, -1, 0));
            r.extend(get_steps_in_direction(start_square, 0, -1));
            r
        }
        Bishop => {
            let mut r = get_steps_in_direction(start_square, 1, 1);
            r.extend(get_steps_in_direction(start_square, -1, 1));
            r.extend(get_steps_in_direction(start_square, 1, -1));
            r.extend(get_steps_in_direction(start_square, -1, -1));
            r
        }
        Queen => {
            let mut r = get_move_pattern(Rook, start_square);
            r.extend(get_move_pattern(Bishop, start_square));
            r
        }
        _ => panic!("Piece {:?} passed to get_move_pattern", piece),
    }
}

/// Finds moves that the piece would be able to make, doesn't look for pieces blocking, king in check, etc.
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
    let mut add_index = |idx: isize| {
        moves.push(Move {
            from_square: location_of_piece,
            to_square: Square::from_usize(idx as usize),
        });
    };
    let add_indices = |idxs: &Vec<isize>| {
        for idx in idxs {
            if !Square::exists(*idx) {
                panic!("Tried to add square index {} as valid move.", idx)
            }
            let target_sq = position.squares[*idx as usize];
            if let SquareValue::Occupied(Piece { kind, colour }) = target_sq {
                if colour == my_colour {
                    continue;
                }
            }
            add_index(*idx);
        }
    };
    let mut add_offsets = |offsets: &Vec<isize>| {
        for offset in offsets {
            let idx = start_idx + offset;
            if !Square::exists(idx) {
                continue;
            }
            let target_sq = position.squares[idx as usize];
            if let SquareValue::Occupied(Piece { kind, colour }) = target_sq {
                if colour == my_colour {
                    continue;
                }
            }
            add_index(idx);
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
            let push_square = start_idx + (BOARD_SIZE as isize * forward_offset);
            add_index(push_square);
            // push 2 squares from starting position
            if location_of_piece.is_starting_square(my_kind, my_colour) {
                let double_push_square = start_idx + 2 * (BOARD_SIZE as isize * forward_offset);
                add_index(double_push_square);
            }
            for o in capture_offsets {
                if position.squares[(start_idx + o) as usize]
                    .is_occupied_by_colour(my_colour.other())
                    || position
                        .en_passant_square
                        .is_some_and(|sq| sq == Square::from_isize(start_idx + 0))
                {
                    add_index(start_idx + o)
                }
            }
        }
        _ => add_offsets(&get_move_pattern(my_kind, location_of_piece)),
    }
    return Some(moves);
}

pub fn move_is_valid(position: &Position, themove: &Move) -> bool {
    let piece_is_there = position.squares[themove.from_square as usize].is_occupied();
    // FIXME: inefficient probably
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
    use crate::{
        moves::{BOARD_SIZE, get_steps_in_direction},
        square::Square::*,
        *,
    };

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
    fn find_king_moves() {
        let king_in_middle = fen::parse("8/8/8/3K4/8/8/8/8 w - - 0 1");
        let king_in_corner = fen::parse("8/8/8/8/8/8/8/K7 w - - 0 1");
        let piece = Piece {
            kind: PieceKind::King,
            colour: PieceColour::White,
        };

        let mut expected_moves_corner: Vec<Move> = vec![A2, B2, B1]
            .iter()
            .map(|sq| Move {
                from_square: A1,
                to_square: *sq,
            })
            .collect::<Vec<Move>>();
        let mut expected_moves_middle: Vec<Move> = vec![D4, D6, E4, E5, E6, C4, C5, C6]
            .iter()
            .map(|sq| Move {
                from_square: D5,
                to_square: *sq,
            })
            .collect::<Vec<Move>>();
        expected_moves_middle.sort();
        expected_moves_corner.sort();
        let mut result_middle = moves::find_avail_moves_for_piece(&king_in_middle, D5).unwrap();
        result_middle.sort();
        let mut result_corner = moves::find_avail_moves_for_piece(&king_in_corner, A1).unwrap();
        result_corner.sort();
        assert_eq!(result_middle, expected_moves_middle);
        assert_eq!(result_corner, expected_moves_corner);
    }

    #[test]
    fn test_is_edge_square() {
        assert!(Square::from_u8(0).is_bottom_edge());
        assert!(Square::from_u8(1).is_edge_square());
        assert!(Square::from_u8(2).is_edge_square());
        assert!(Square::from_u8(3).is_bottom_edge());
        assert!(Square::from_u8(4).is_edge_square());
        assert!(Square::from_u8(5).is_edge_square());
        assert!(Square::from_u8(6).is_edge_square());
        assert!(Square::from_u8(7).is_edge_square());
        assert!(Square::from_u8(8).is_edge_square());
        assert!(Square::from_u8(15).is_right_edge());
        assert!(Square::from_u8(16).is_edge_square());
        assert!(Square::from_u8(23).is_edge_square());
        assert!(Square::from_u8(24).is_edge_square());
        assert!(Square::from_u8(31).is_edge_square());
        assert!(Square::from_u8(32).is_edge_square());
        assert!(Square::from_u8(39).is_edge_square());
        assert!(Square::from_u8(40).is_edge_square());
        assert!(Square::from_u8(47).is_edge_square());
        assert!(Square::from_u8(48).is_edge_square());
        assert!(Square::from_u8(55).is_edge_square());
        assert!(Square::from_u8(56).is_edge_square());
        assert!(Square::from_u8(57).is_edge_square());
        assert!(Square::from_u8(58).is_top_edge());
        assert!(Square::from_u8(59).is_top_edge());
        assert!(Square::from_u8(60).is_top_edge());
        assert!(Square::from_u8(61).is_top_edge());
        assert!(Square::from_u8(62).is_top_edge());
        assert!(Square::from_u8(63).is_top_edge());
        //
        assert!(!Square::from_u8(09).is_edge_square());
        assert!(!Square::from_u8(10).is_edge_square());
        assert!(!Square::from_u8(11).is_edge_square());
        assert!(!Square::from_u8(12).is_edge_square());
        assert!(!Square::from_u8(42).is_edge_square());
        assert!(!Square::from_u8(54).is_edge_square());
        assert!(!Square::from_u8(42).is_edge_square());
        assert!(!Square::from_u8(49).is_edge_square());
        assert!(!Square::from_u8(33).is_edge_square());
        assert!(!Square::from_u8(38).is_edge_square());
        assert!(!Square::from_u8(14).is_edge_square());
    }

    #[test]
    fn test_get_steps_in_direction() {
        let size = BOARD_SIZE as isize;

        let square = Square::A1;
        let (step_x, step_y) = (1, 1);
        let diff = size * step_y + step_x;
        let steps: Vec<isize> = (1..=7).map(|i| i * diff).collect();
        assert_eq!(get_steps_in_direction(square, step_x, step_y), steps);
    }
}
