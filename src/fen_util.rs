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
            if piece.is_digit(10) {
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
pub fn valid_fen(fen: &str) -> bool {
    let is_valid: bool = true;
    //Check if fen is valid
    is_valid
}

/// Generates a FEN (Forsyth-Edwards Notation) string representing the current state of the chessboard.
/// 
/// # Arguments
/// 
/// * `chessboard` - A mutable reference to the chessboard.
/// * `string_array` - A mutable vector of strings to store intermediate FEN string components.
pub fn set_pieces(chessboard: &mut Chessboard, string_array: &mut Vec<String>) {
    for rank in (1..=8).rev() {
        let mut empty_squares = 0;
        let mut row_string = String::new();

        for file in 1..=8 {
            let piece = chessboard.piece_at_position(rank, file);

            if piece == '.' {
                empty_squares += 1;
            } else {
                if empty_squares > 0 {
                    row_string.push_str(&empty_squares.to_string());
                    empty_squares = 0;
                }
                row_string.push(piece);
            }
        }

        if empty_squares > 0 {
            row_string.push_str(&empty_squares.to_string());
        }

        string_array.push(row_string);
        string_array.push("/".to_owned());
    }

    let binding = string_array.concat();
    let mut fen_string = binding.chars();
    fen_string.next_back();
    let fen_string_no_slash = fen_string.as_str();
    for item in string_array.iter_mut() {
        item.clear();
    }
    string_array.push(fen_string_no_slash.to_owned());
    string_array.push(" ".to_owned());
}

/// Sets the castling rights information in the FEN (Forsyth-Edwards Notation) string.
/// 
/// # Arguments
/// 
/// * `chessboard` - A mutable reference to the chessboard.
/// * `string_array` - A mutable vector of strings to store intermediate FEN string components.
pub fn set_castling_rights(chessboard: &mut Chessboard, string_array: &mut Vec<String>) {
    string_array.push(match chessboard.castling_rights {
        0 => "- ".to_string(),
        rights => {
            let rights_string = "KQkq"
                .chars()
                .filter(|&c| {
                    (rights
                        & match c {
                            'K' => 0b1000,
                            'Q' => 0b0100,
                            'k' => 0b0010,
                            'q' => 0b0001,
                            _ => 0,
                        })
                        != 0
                })
                .collect::<String>();

            format!("{} ", rights_string)
        }
    });
}

/// Sets the en passant information in the FEN (Forsyth-Edwards Notation) string.
/// 
/// # Arguments
/// 
/// * `chessboard` - A mutable reference to the chessboard.
/// * `string_array` - A mutable vector of strings to store intermediate FEN string components.
pub fn set_en_passant(chessboard: &mut Chessboard, string_array: &mut Vec<String>) {
    if chessboard.en_passant == 0 {
        string_array.push("- ".to_string());
    } else {
        let row = (chessboard.en_passant - 1) / 8 + 1;
        let col = (chessboard.en_passant - 1) % 8;
        let column_char = (b'A' + col) as char;

        string_array.push(format!("{}{}", column_char, row));
        string_array.push(" ".to_owned());
    }
}
