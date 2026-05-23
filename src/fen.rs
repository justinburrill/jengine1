use crate::*;

pub fn parse_fen_pieces(input: &str) -> [SquareValue; 64] {
    let mut squares: [SquareValue; 64] = [SquareValue::Empty; 64];
    let mut current_square: usize = 0;
    for rank in input.split('/').rev() {
        for ch in rank.chars() {
            if ('1'..'9').contains(&ch) {
                let num: usize = ch.to_digit(10).unwrap().try_into().unwrap();
                current_square += num;
                continue;
            }
            let kind: PieceKind = match ch.to_ascii_lowercase() {
                'r' => PieceKind::Rook,
                'n' => PieceKind::Knight,
                'b' => PieceKind::Bishop,
                'q' => PieceKind::Queen,
                'k' => PieceKind::King,
                'p' => PieceKind::Pawn,
                _ => panic!("recieved unknown character in FEN position: {}", ch),
            };
            let colour: PieceColour = if ch.is_uppercase() {
                PieceColour::White
            } else {
                PieceColour::Black
            };

            let piece = Piece { kind, colour };
            squares[current_square] = SquareValue::Occupied(piece);
            current_square += 1;
        }
    }
    return squares;
}

pub fn parse(input: &str) -> Position {
    let sections: Vec<&str> = input.split_whitespace().collect();
    let squares = parse_fen_pieces(sections[0]);
    let white_to_move: bool = match sections[1] {
        "w" => true,
        "b" => false,
        _ => panic!(
            "recieved unknown string in turn specifier, should be w or b: {}",
            sections[1]
        ),
    };
    let mut castle_availability: Vec<CastleAvailability> = vec![];
    for ch in sections[2].chars() {
        castle_availability.push(match ch {
            'K' => CastleAvailability::WhiteKingside,
            'Q' => CastleAvailability::WhiteQueenside,
            'k' => CastleAvailability::BlackKingside,
            'q' => CastleAvailability::BlackQueenside,
            '-' => break,
            _ => panic!(
                "Recieved unknown character in FEN castle availability: {}",
                ch
            ),
        });
    }
    let en_passant_square = Square::from_str(sections[3]);
    let half_moves_since_capture_or_pawn_advance: i32 = sections[4]
        .parse()
        .expect("Recieved invalid number in FEN half move count");
    let full_move_count: i32 = sections[5]
        .parse()
        .expect("Recieved invalid number in FEN full move count");
    return Position {
        squares,
        white_to_move,
        castle_availability,
        en_passant_square,
        half_moves_since_capture_or_pawn_advance,
        full_move_count,
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_starting_position() {
        let expected: Position = starting_position!();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(expected, fen::parse(fen));
    }

    #[test]
    fn test_e4() {
        let expected: Position = Position {
            white_to_move: false,
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
                no!(),
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
            en_passant_square: None,
        };
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        assert_eq!(expected, fen::parse(fen));
    }

    #[test]
    fn complicated_position() {
        let fen = "r1bqk1nr/pppp2pp/2n2p2/2b1p3/4P1P1/2N2N2/PPPPKP1P/R1BQ1B1R b kq - 3 5";

        let expected: Position = Position {
            white_to_move: false,
            full_move_count: 5,
            half_moves_since_capture_or_pawn_advance: 3,
            en_passant_square: None,
            castle_availability: vec![
                CastleAvailability::BlackKingside,
                CastleAvailability::BlackQueenside,
            ],
            squares: [
                wr!(),
                no!(),
                wb!(),
                wq!(),
                no!(),
                wb!(),
                no!(),
                wr!(),
                wp!(),
                wp!(),
                wp!(),
                wp!(),
                wk!(),
                wp!(),
                no!(),
                wp!(),
                no!(),
                no!(),
                wn!(),
                no!(),
                no!(),
                wn!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                wp!(),
                no!(),
                wp!(),
                no!(),
                no!(),
                no!(),
                bb!(),
                no!(),
                bp!(),
                no!(),
                no!(),
                no!(),
                no!(),
                no!(),
                bn!(),
                no!(),
                no!(),
                bp!(),
                no!(),
                no!(),
                bp!(),
                bp!(),
                bp!(),
                bp!(),
                no!(),
                no!(),
                bp!(),
                bp!(),
                br!(),
                no!(),
                bb!(),
                bq!(),
                bk!(),
                no!(),
                bn!(),
                br!(),
            ],
        };

        assert_eq!(expected, fen::parse(fen));
    }
}
