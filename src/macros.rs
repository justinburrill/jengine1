#[macro_export]
macro_rules! square_from_num {
    ($num:expr) => {
        match $num {
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,
            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,
            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,
            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,
            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,
            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,
            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,
            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,
            _ => panic!("Invalid square index passed: {}", ($num)),
        }
    };
}

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
            en_passant_square: None,
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
