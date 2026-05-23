#[macro_export]
macro_rules! starting_position {
    () => {
        Position {
            white_to_move: true,
            full_move_count: 1,
            half_moves_since_capture_or_pawn_advance: 0,
            castle_availability: vec![
                CastleAvailability::WhiteKingside,
                CastleAvailability::WhiteQueenside,
                CastleAvailability::BlackKingside,
                CastleAvailability::BlackQueenside,
            ],
            squares: [
                wr!(),
                wn!(),
                wb!(),
                wq!(),
                wk!(),
                wb!(),
                wn!(),
                wr!(),
                wp!(),
                wp!(),
                wp!(),
                wp!(),
                wp!(),
                wp!(),
                wp!(),
                wp!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                bp!(),
                bp!(),
                bp!(),
                bp!(),
                bp!(),
                bp!(),
                bp!(),
                bp!(),
                br!(),
                bn!(),
                bb!(),
                bq!(),
                bk!(),
                bb!(),
                bn!(),
                br!(),
            ],
        }
    };
}

#[macro_export]
macro_rules! br {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Rook,
            colour: PieceColour::Black,
        })
    };
}

#[macro_export]
macro_rules! bb {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Bishop,
            colour: PieceColour::Black,
        })
    };
}

#[macro_export]
macro_rules! bn {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::Black,
        })
    };
}

#[macro_export]
macro_rules! bq {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Queen,
            colour: PieceColour::Black,
        })
    };
}

#[macro_export]
macro_rules! bk {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::King,
            colour: PieceColour::Black,
        })
    };
}

#[macro_export]
macro_rules! bp {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Pawn,
            colour: PieceColour::Black,
        })
    };
}

#[macro_export]
macro_rules! wr {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Rook,
            colour: PieceColour::White,
        })
    };
}

#[macro_export]
macro_rules! wb {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Bishop,
            colour: PieceColour::White,
        })
    };
}

#[macro_export]
macro_rules! wn {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::White,
        })
    };
}

#[macro_export]
macro_rules! wq {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Queen,
            colour: PieceColour::White,
        })
    };
}

#[macro_export]
macro_rules! wk {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::King,
            colour: PieceColour::White,
        })
    };
}

#[macro_export]
macro_rules! wp {
    () => {
        SquareValue::Occupied(Piece {
            kind: PieceKind::Pawn,
            colour: PieceColour::White,
        })
    };
}

#[macro_export]
macro_rules! no {
    () => {
        SquareValue::Empty
    };
}
