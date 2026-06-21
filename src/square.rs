use crate::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
#[repr(u8)]
pub enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}

impl Square {
    /// Returns (x, y) from white's bottom left corner, (1, 1)-based
    pub fn to_coords(&self) -> (u8, u8) {
        let board_size = 8;
        let idx = *self as u8;
        let y = idx / board_size;
        let x = idx - (y * board_size);
        (x + 1, y + 1)
    }

    pub fn from_coords(col: u8, row: u8) -> Square {
        Square::from_usize(((row * 8) + col) as usize)
    }

    pub fn from_usize(int: usize) -> Square {
        match int {
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
            _ => panic!("Invalid square index passed: {}", int),
        }
    }

    pub fn from_isize(int: isize) -> Square {
        Square::from_usize(int.try_into().unwrap())
    }

    pub fn from_str(string: &str) -> Option<Square> {
        if string.len() != 2 {
            return None;
        }
        let mut chars = string.chars();
        let col_s: char = chars.next().unwrap();
        let row_s: char = chars.next().unwrap();
        if !('a'..='h').contains(&col_s)
            || !('A'..='H').contains(&col_s)
            || !('1'..='8').contains(&row_s)
        {
            return None;
        }
        let col: u8 = if col_s.is_ascii_uppercase() {
            (col_s as u8) - ('A' as u8) + 1
        } else {
            col_s as u8 - ('a' as u8) + 1
        };
        let row: u8 = (row_s as u8) - ('0' as u8);
        return Some(Square::from_coords(col, row));
    }

    /// Returns the square across the board.
    pub fn mirror_opposite(&self) -> Square {
        let (x, y) = self.to_coords();
        Square::from_coords(x, opposite_row(y))
    }

    /// Returns square that is a 180deg rotation from this square.
    pub fn pivot_opposite(&self) -> Square {
        let (x, y) = self.to_coords();
        Square::from_coords(opposite_row(x), opposite_row(y))
    }

    pub fn moves_from_back_rank(&self, colour: &PieceColour) -> u8 {
        match colour {
            PieceColour::White => *self as u8 / 8,
            PieceColour::Black => self
                .pivot_opposite()
                .moves_from_back_rank(&PieceColour::White),
        }
    }

    pub fn moves_from_center(&self) -> u8 {
        let (mut x, mut y) = self.to_coords();
        if x > 4 {
            x -= 1
        }
        if y > 4 {
            y -= 1
        }
        let dx = (x as i8 - 4).abs();
        let dy = (y as i8 - 4).abs();
        std::cmp::max(dx, dy) as u8
    }

    pub fn exists(idx: isize) -> bool {
        idx >= 0 && idx <= 63
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Square::A1 => "A1",
                Square::A2 => "A2",
                Square::A3 => "A3",
                Square::A4 => "A4",
                Square::A5 => "A5",
                Square::A6 => "A6",
                Square::A7 => "A7",
                Square::A8 => "A8",
                Square::B1 => "B1",
                Square::B2 => "B2",
                Square::B3 => "B3",
                Square::B4 => "B4",
                Square::B5 => "B5",
                Square::B6 => "B6",
                Square::B7 => "B7",
                Square::B8 => "B8",
                Square::C1 => "C1",
                Square::C2 => "C2",
                Square::C3 => "C3",
                Square::C4 => "C4",
                Square::C5 => "C5",
                Square::C6 => "C6",
                Square::C7 => "C7",
                Square::C8 => "C8",
                Square::D1 => "D1",
                Square::D2 => "D2",
                Square::D3 => "D3",
                Square::D4 => "D4",
                Square::D5 => "D5",
                Square::D6 => "D6",
                Square::D7 => "D7",
                Square::D8 => "D8",
                Square::E1 => "E1",
                Square::E2 => "E2",
                Square::E3 => "E3",
                Square::E4 => "E4",
                Square::E5 => "E5",
                Square::E6 => "E6",
                Square::E7 => "E7",
                Square::E8 => "E8",
                Square::F1 => "F1",
                Square::F2 => "F2",
                Square::F3 => "F3",
                Square::F4 => "F4",
                Square::F5 => "F5",
                Square::F6 => "F6",
                Square::F7 => "F7",
                Square::F8 => "F8",
                Square::G1 => "G1",
                Square::G2 => "G2",
                Square::G3 => "G3",
                Square::G4 => "G4",
                Square::G5 => "G5",
                Square::G6 => "G6",
                Square::G7 => "G7",
                Square::G8 => "G8",
                Square::H1 => "H1",
                Square::H2 => "H2",
                Square::H3 => "H3",
                Square::H4 => "H4",
                Square::H5 => "H5",
                Square::H6 => "H6",
                Square::H7 => "H7",
                Square::H8 => "H8",
            },
        )
    }
}

fn opposite_row(row: u8) -> u8 {
    9 - row
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SquareValue {
    #[default]
    Empty,
    Occupied(Piece),
}

impl SquareValue {
    pub fn is_occupied(&self) -> bool {
        match self {
            SquareValue::Occupied(_) => true,
            SquareValue::Empty => false,
        }
    }

    pub fn is_occupied_by_colour(&self, colour: PieceColour) -> bool {
        match self {
            SquareValue::Occupied(p) => p.colour == colour,
            SquareValue::Empty => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum PieceKind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

impl PieceKind {
    pub fn piece_value(self: &PieceKind) -> u8 {
        match self {
            PieceKind::King => 0,
            PieceKind::Queen => 9,
            PieceKind::Rook => 5,
            PieceKind::Bishop => 3,
            PieceKind::Knight => 3,
            PieceKind::Pawn => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum PieceColour {
    White,
    Black,
}

impl PieceColour {
    pub fn other(self: &PieceColour) -> PieceColour {
        match self {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub colour: PieceColour,
}

impl Piece {
    pub fn to_letter(&self) -> Option<char> {
        match self.kind {
            PieceKind::Rook => Some('R'),
            PieceKind::Bishop => Some('B'),
            PieceKind::Queen => Some('Q'),
            PieceKind::King => Some('K'),
            PieceKind::Knight => Some('N'),
            PieceKind::Pawn => None,
        }
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.colour.partial_cmp(&other.colour) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.kind.partial_cmp(&other.kind)
    }
}

#[cfg(test)]
mod tests {
    use crate::square::*;

    #[test]
    fn mirror_and_pivot() {
        assert_eq!(Square::A1.pivot_opposite(), Square::H8);
        assert_eq!(Square::A2.pivot_opposite(), Square::H7);
        assert_eq!(Square::B2.pivot_opposite(), Square::G7);

        assert_eq!(Square::B2.mirror_opposite(), Square::B7);
        assert_eq!(Square::B7.mirror_opposite(), Square::B2);
        assert_eq!(Square::H7.mirror_opposite(), Square::H2);
        assert_eq!(Square::A1.mirror_opposite(), Square::A8);
    }

    #[test]
    fn moves_from_back_rank() {
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

    #[test]
    fn coords() {
        assert_eq!(Square::A1.to_coords(), (1, 1));
        assert_eq!(Square::H1.to_coords(), (8, 1));
        assert_eq!(Square::A8.to_coords(), (1, 8));
        assert_eq!(Square::H8.to_coords(), (8, 8));
        assert_eq!(Square::D4.to_coords(), (4, 4));
        assert_eq!(Square::C6.to_coords(), (3, 6));
        assert_eq!(Square::E6.to_coords(), (5, 6));
        assert_eq!(Square::E4.to_coords(), (5, 4));
        assert_eq!(Square::F7.to_coords(), (6, 7));
    }

    #[test]
    fn moves_from_center() {
        assert_eq!(Square::E4.moves_from_center(), 0);
        assert_eq!(Square::E5.moves_from_center(), 0);
        assert_eq!(Square::D4.moves_from_center(), 0);
        assert_eq!(Square::D5.moves_from_center(), 0);
        assert_eq!(Square::H1.moves_from_center(), 3);
        assert_eq!(Square::H8.moves_from_center(), 3);
        assert_eq!(Square::A1.moves_from_center(), 3);
        assert_eq!(Square::A8.moves_from_center(), 3);
    }

    #[test]
    fn from_str() {
        assert_eq!(Square::from_str("A1"), Some(Square::A1));
        assert_eq!(Square::from_str("A2"), Some(Square::A2));
        assert_eq!(Square::from_str("h6"), Some(Square::H6));
        assert_eq!(Square::from_str("G8"), Some(Square::G8));
        assert_eq!(Square::from_str("d4"), Some(Square::D4));
        assert_eq!(Square::from_str("f3"), Some(Square::F3));
        assert_eq!(Square::from_str(""), None);
        assert_eq!(Square::from_str("A11"), None);
        assert_eq!(Square::from_str("A9"), None);
        assert_eq!(Square::from_str("I5"), None);
    }
}
