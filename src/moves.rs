use crate::*;
use std::cmp::min;

pub(crate) const BOARD_SIZE: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from_square: Square,
    pub to_square: Square,
}

impl Move {
    /// Input a move like "e2e4"
    pub fn from_str(string: &str) -> Option<Move> {
        if string.len() != 4 {
            return None;
        }
        let from_square = Square::from_str(&string[0..2])?;
        let to_square = Square::from_str(&string[2..4])?;
        Some(Move { from_square, to_square })
    }
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

#[derive(Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn to_xy(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            Up => (0, 1),
            UpRight => (1, 1),
            Right => (1, 0),
            DownRight => (1, -1),
            Down => (0, -1),
            DownLeft => (-1, -1),
            Left => (-1, 0),
            UpLeft => (-1, 1),
        }
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

/// For player who is currently to move
pub fn find_avail_moves(position: &Position) -> Vec<Move> {
    let whos_move = position.whos_move();
    find_avail_moves_for_player(position, whos_move)
}

pub fn find_avail_moves_for_player(position: &Position, to_move: PieceColour) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for sq in get_squares_with_pieces(position, &position.whos_move()) {
        moves.extend(
            find_avail_moves_for_piece(position, sq).expect("tried to find available moves for a piece that doesn't exist"),
        );
    }
    moves
}

/// Finds moves that the piece would be able to make, doesn't look for king in check.
/// Returns `None` if no piece is at the square.
pub fn find_avail_moves_for_piece(position: &Position, start_square: Square) -> Option<Vec<Move>> {
    use PieceKind::*;
    let sqvalue = position.squares[start_square as usize];
    let mut moves: Vec<Move> = vec![];
    let my_kind;
    let my_colour;
    if let SquareValue::Occupied(Piece { kind, colour }) = sqvalue {
        my_kind = kind;
        my_colour = colour;
    } else {
        return None;
    };

    let start_idx: isize = start_square as isize;
    let mut add_index = |idx: isize| {
        moves.push(Move {
            from_square: start_square,
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
            if let Some(push_square) = get_pawn_push_square(start_square, my_colour) {
                add_index(push_square as isize);
            }
            // push 2 squares from starting position
            if let Some(dbl_push_square) = get_pawn_double_push_square(start_square, my_colour) {
                add_index(dbl_push_square as isize);
            }
            for offset in get_pawn_capture_offsets(my_colour) {
                // capture if there's a piece, or en passant is available
                if position.squares[(start_idx + offset) as usize].is_occupied_by_colour(my_colour.other())
                    || position
                        .en_passant_square
                        .is_some_and(|sq| sq == Square::from_isize(start_idx + offset))
                {
                    add_index(start_idx + offset)
                }
            }
        }
        King => {
            let mut moves = get_king_moves(start_square);
            // dbg!(&moves);
            trim_jump_moves(position, start_square, my_colour, &mut moves);
            // dbg!(&moves);
            add_offsets(&moves);
        }
        Knight => {
            let mut moves = get_knight_moves(start_square);
            trim_jump_moves(position, start_square, my_colour, &mut moves);
            add_offsets(&moves);
        }
        _ => add_offsets(&get_straight_line_moves(
            position,
            Piece {
                kind: my_kind,
                colour: my_colour,
            },
            start_square,
        )),
    }
    return Some(moves);
}

fn get_steps_in_direction(start_square: Square, number_of_steps: usize, direction: Direction) -> Vec<isize> {
    let (step_x, step_y) = direction.to_xy();
    let diff: isize = step_y * BOARD_SIZE as isize + step_x;
    let mut out: Vec<isize> = Vec::with_capacity(number_of_steps as usize);

    for i in 1..=number_of_steps {
        out.push(i as isize * diff);
    }
    return out;
}

fn get_king_moves(square: Square) -> Vec<isize> {
    let s = BOARD_SIZE as isize;
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
        (true, false, true, false) => vec![1, -s, -s + 1],
        (true, false, false, true) => vec![1, s, s + 1],
        (false, true, true, false) => vec![-1, -s, -s + 1],
        (false, true, false, true) => vec![-1, s, s - 1],
        (l, r, t, b) => panic!("Invalid edge combination somehow: {}, {}, {}, {}", l, r, t, b),
    }
}

/// Remove jumping (knight) moves if they would land on a friendly piece
fn trim_jump_moves(position: &Position, start_square: Square, my_colour: PieceColour, moves: &mut Vec<isize>) {
    moves.retain(|offset| {
        let dest = Square::from_isize(start_square as isize + offset);
        if let Some(piece) = position.piece_at(dest) {
            return piece.colour != my_colour;
        } else {
            return true;
        }
    });
}

fn get_avail_king_moves(position: &Position, start_square: Square, colour: PieceColour) -> Vec<isize> {
    let mut moves = get_king_moves(start_square);
    trim_jump_moves(position, start_square, colour, &mut moves);
    moves
}

// TODO: needs fixing, will go off the board
fn get_knight_moves(start_square: Square) -> Vec<isize> {
    let size = BOARD_SIZE as isize;
    let (x, y) = start_square.to_coords();
    let jumps: [(isize, isize); 8] = [(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1), (-1, -2), (-2, -1)];
    jumps
        .iter()
        .filter_map(|(dx, dy)| {
            let newx = x as isize + dx;
            let newy = y as isize + dy;
            if (0..=BOARD_SIZE as isize).contains(&newx) && (0..=BOARD_SIZE as isize).contains(&newy) {
                Some(newx + BOARD_SIZE as isize * newy)
            } else {
                None
            }
        })
        .collect()
}

/// Returns the number of steps in a direction until it takes an enemy piece or is blocked by friendly piece / end of the board
pub fn steps_until_blocked(
    position: &Position,
    start_square: Square,
    my_colour: PieceColour,
    direction: Direction,
) -> usize {
    let (step_x, step_y) = direction.to_xy();
    let max_steps = steps_until_edge(start_square, direction);
    let start_idx = start_square as isize;
    for i in 1..=max_steps {
        let diff = step_x + BOARD_SIZE as isize * step_y;
        let next_square = start_idx + diff;
        if let Some(piece) = position.piece_at(Square::from_isize(next_square)) {
            return if my_colour == piece.colour { i - 1 } else { i };
        } else {
            continue;
        }
    }
    max_steps // not blocked
}

/// Returns the move offsets in a direction until the move is no longer valid
pub fn get_offsets_in_direction(
    position: &Position,
    start_square: Square,
    my_colour: PieceColour,
    direction: Direction,
) -> Vec<isize> {
    let (step_x, step_y) = direction.to_xy();
    let n_steps = steps_until_blocked(position, start_square, my_colour, direction);
    let mut out = Vec::with_capacity(n_steps);
    for i in 1..=n_steps {
        let diff = step_x + BOARD_SIZE as isize * step_y;
        out.push(diff);
    }
    out
}

/// Returns the number of squares until the edge
pub fn steps_until_edge(start_square: Square, direction: Direction) -> usize {
    let (x, y) = start_square.to_coords();
    let (step_x, step_y) = direction.to_xy();
    let max_x_steps = if step_x < 0 { x - 1 } else { BOARD_SIZE - x };
    let max_y_steps = if step_y < 0 { y - 1 } else { BOARD_SIZE - y };
    if step_x == 0 {
        max_y_steps
    } else if step_y == 0 {
        max_x_steps
    } else {
        let max_steps = min(max_x_steps, max_y_steps);
        max_steps
    }
}

/// For pieces like rooks and bishops
pub fn get_straight_line_moves(position: &Position, piece: Piece, start_square: Square) -> Vec<isize> {
    use Direction::*;
    use PieceKind::*;
    let size = BOARD_SIZE as isize;
    let Piece { kind, colour } = piece;
    match kind {
        Rook => {
            let mut r = get_offsets_in_direction(position, start_square, colour, Up);
            r.extend(get_offsets_in_direction(position, start_square, colour, Down));
            r.extend(get_offsets_in_direction(position, start_square, colour, Left));
            r.extend(get_offsets_in_direction(position, start_square, colour, Right));
            r
        }
        Bishop => {
            let mut r = get_offsets_in_direction(position, start_square, colour, UpLeft);
            r.extend(get_offsets_in_direction(position, start_square, colour, DownLeft));
            r.extend(get_offsets_in_direction(position, start_square, colour, UpRight));
            r.extend(get_offsets_in_direction(position, start_square, colour, DownRight));
            r
        }
        Queen => {
            let mut r = get_straight_line_moves(position, Piece { kind: Rook, colour }, start_square);
            r.extend(get_straight_line_moves(
                position,
                Piece { kind: Bishop, colour },
                start_square,
            ));
            r
        }
        _ => panic!("Piece {:?} passed to get_move_pattern", piece),
    }
}

pub fn get_pawn_double_push_square(pawn_position: Square, pawn_colour: PieceColour) -> Option<Square> {
    let forward_offset = get_pawn_push_offset(pawn_colour);
    if pawn_position.is_starting_square(PieceKind::Pawn, pawn_colour) {
        Some(Square::from_isize(
            pawn_position as isize + 2 * (BOARD_SIZE as isize * forward_offset),
        ))
    } else {
        None
    }
}

pub fn get_pawn_push_square(pawn_position: Square, pawn_colour: PieceColour) -> Option<Square> {
    let forward_offset = get_pawn_push_offset(pawn_colour);
    if (pawn_colour == PieceColour::White && !pawn_position.is_top_edge())
        || (pawn_colour == PieceColour::Black && !pawn_position.is_bottom_edge())
    {
        Some(Square::from_isize(
            pawn_position as isize + (BOARD_SIZE as isize * forward_offset),
        ))
    } else {
        None
    }
}

pub fn get_pawn_push_offset(pawn_colour: PieceColour) -> isize {
    match pawn_colour {
        PieceColour::White => 1,
        PieceColour::Black => -1,
    }
}

pub fn get_pawn_capture_offsets(pawn_colour: PieceColour) -> [isize; 2] {
    match pawn_colour {
        PieceColour::White => [7, 9],
        PieceColour::Black => [-7, -9],
    }
}

pub fn move_is_valid(position: &Position, themove: &Move) -> bool {
    if let Some(piece) = position.piece_at(themove.from_square) {
        let my_colour = piece.colour;
        let piece_can_move_like_that = find_avail_moves_for_piece(position, themove.from_square)
            .expect("tried to find available moves for a piece that doesn't exist")
            .contains(themove);
        let my_king_in_check = evaluation::is_king_in_check(position, my_colour);
        piece_can_move_like_that && !my_king_in_check
    } else {
        // no piece at from_square
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction::*, square::Square::*, *};

    #[test]
    fn valid_moves_from_starting_position() {
        let expected: Vec<Move> = vec![];
        let pos = starting_position!();
    }

    #[test]
    fn find_basic_king_moves() {
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
    fn find_basic_knight_moves() {
        let knight_in_middle = fen::parse("8/8/8/3N4/8/8/8/8 w - - 0 1");
        let knight_in_corner = fen::parse("8/8/8/8/8/8/8/N7 w - - 0 1");
        let piece = Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::White,
        };

        let mut expected_moves_corner: Vec<Move> = vec![B3, C2]
            .iter()
            .map(|sq| Move {
                from_square: A1,
                to_square: *sq,
            })
            .collect::<Vec<Move>>();
        let mut expected_moves_middle: Vec<Move> = vec![E7, F6, F4, E3, C3, B4, B6, C7]
            .iter()
            .map(|sq| Move {
                from_square: D5,
                to_square: *sq,
            })
            .collect::<Vec<Move>>();
        expected_moves_middle.sort();
        expected_moves_corner.sort();
        let mut result_middle = moves::find_avail_moves_for_piece(&knight_in_middle, D5).unwrap();
        result_middle.sort();
        let mut result_corner = moves::find_avail_moves_for_piece(&knight_in_corner, A1).unwrap();
        result_corner.sort();
        assert_eq!(result_middle, expected_moves_middle);
        assert_eq!(result_corner, expected_moves_corner);
    }

    #[test]
    fn test_steps_until_edge() {
        assert_eq!(steps_until_edge(A1, Down), 0);
        assert_eq!(steps_until_edge(C7, Up), 1);
        assert_eq!(steps_until_edge(E5, Left), 4);
        assert_eq!(steps_until_edge(C7, Left), 2);
    }
}
