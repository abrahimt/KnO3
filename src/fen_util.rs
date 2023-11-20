use crate::board::Chessboard;

/// Places pieces on the chessboard based on the FEN (Forsyth-Edwards Notation) rows provided.
/// 
/// # Arguments
/// 
/// * `chessboard` - A mutable reference to the chessboard.
/// * `fen_rows` - FEN string representing the piece placement on the board.
#[rustfmt::skip]
pub fn place_pieces(chessboard: &mut Chessboard, fen_rows: &str) {
    for (row_index, row_string) in fen_rows.split('/').rev().enumerate() {
        let mut file_ndx: usize = 0;
        for piece in row_string.chars() {
            if piece.is_ascii_digit() {
                file_ndx += piece.to_digit(10).unwrap_or(0) as usize;
                continue;
            }

            let position = 2_u64.pow((8 * row_index + file_ndx) as u32);
            match piece {
                'p' => chessboard.black_pawns   |= position,
                'r' => chessboard.black_rooks   |= position,
                'b' => chessboard.black_bishops |= position,
                'k' => chessboard.black_king    |= position,
                'q' => chessboard.black_queen   |= position,
                'n' => chessboard.black_knights |= position,
                'P' => chessboard.white_pawns   |= position,
                'R' => chessboard.white_rooks   |= position,
                'B' => chessboard.white_bishops |= position,
                'K' => chessboard.white_king    |= position,
                'Q' => chessboard.white_queen   |= position,
                'N' => chessboard.white_knights |= position,
                //_ => { return Err("Invalid piece in FEN string".to_string()); }
                _ => {},
            }
            file_ndx += 1;
        }
    }
}

/// Validates if the given FEN (Forsyth-Edwards Notation) string is well-formed.
///
/// # Arguments
///
/// * `fen` - A FEN string to be validated.
///
/// # Returns
///
/// A boolean indicating whether the FEN string is valid.
#[allow(clippy::all)]
pub fn valid_fen(fen: &str) -> bool {
    let is_valid: bool = true;
    //Check if fen is valid
    is_valid
}

/// Generates a FEN (Forsyth-Edwards Notation) string representing the current state of the chessboard.
pub fn get_fen_placement(chessboard: &Chessboard) -> String {
    let mut result: String = "".to_string();
    for rank in (1..=8).rev() {
        let mut empty_squares = 0;

        for file in 0..=7 {
            let p = chessboard.piece_at_position(rank, file);
            if p == '.' {
                empty_squares += 1;
                continue;
            } else if empty_squares > 0 {
                result.push_str(&empty_squares.to_string());
                empty_squares = 0;
            }
            result.push_str(&p.to_string());
        }
        if empty_squares > 0 {
            result.push_str(&empty_squares.to_string());
        }
        result.push_str(&"/");
    }

    let mut c = result.chars();
    c.next_back();
    c.as_str().to_string()
}

/// Get the castling right substring of a FEN
/// * `chessboard` - The chessboard containing the current game state
/// # Returns
/// A string representing the caslting rights in FEN format
#[rustfmt::skip]
pub fn get_fen_castles(chessboard: &Chessboard) -> String {
    let state = chessboard.castling_rights;
    let rights: String = ['K', 'Q', 'k', 'q']
        .iter()
        .filter(|&&c| state & match c {
            'K' => 0b1000,
            'Q' => 0b0100,
            'k' => 0b0010,
            'q' => 0b0001,
            _ => 0 // TODO: Throw error here?
        } != 0)
        .collect();

    if rights.is_empty() { "-".to_string() } else { rights }
}

pub fn get_fen_passant(chessboard: &Chessboard) -> String {
    let passant = chessboard.en_passant;
    if passant == 0 {
        return "-".to_string();
    }
    let row = (passant - 1) / 8 + 1;
    let col = (passant - 1) % 8;
    let chr = (b'A' + col) as char;
    format!("{}{}", chr, row)
}

/// Parse the piece placement part of the FEN string.
pub fn parse_piece_placement(
    chessboard: &mut Chessboard,
    piece_placement: &str,
) -> Result<(), String> {
    self::place_pieces(chessboard, piece_placement);
    Ok(())
}

/// Parse whose turn it is.
pub fn parse_whose_turn(chessboard: &mut Chessboard, whose_turn: &str) {
    chessboard.white_turn = whose_turn == "w";
}

/// Parse castling rights.
pub fn parse_castling_rights(chessboard: &mut Chessboard, castle_rights: &str) {
    for c in castle_rights.chars() {
        let v = match c {
            'K' => 0b1000,
            'Q' => 0b0100,
            'k' => 0b0010,
            'q' => 0b0001,
            _ => 0b0,
        };
        chessboard.castling_rights |= v;
    }
}

/// Parse en passant square.
pub fn parse_en_passant(chessboard: &mut Chessboard, en_passant: &str) {
    if en_passant != "-" {
        if let (Some(col), Some(row)) = (
            en_passant.chars().next().map(|c| c.to_ascii_uppercase()),
            en_passant.chars().nth(1).and_then(|c| c.to_digit(10)),
        ) {
            if (1..=8).contains(&row) {
                let col_value: u8 = match col {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    'D' => 4,
                    'E' => 5,
                    'F' => 6,
                    'G' => 7,
                    'H' => 8,
                    _ => 0,
                };
                chessboard.en_passant = col_value + 8 * (row as u8 - 1);
            }
        }
    }
}
