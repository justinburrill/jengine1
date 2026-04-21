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
