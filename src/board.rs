use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use num_traits::pow;
use std::{io::stdout, u8};

/// Struct representing a chessboard with piece positions and game state
/// Each `piece` is a uint64 bitboard. Each byte represents a rank and a 1 indicates a presence in
/// that position.
pub struct Chessboard {
    pub(crate) black_pawns: u64,
    pub(crate) black_rooks: u64,
    pub(crate) black_knights: u64,
    pub(crate) black_bishops: u64,
    pub(crate) black_queen: u64,
    pub(crate) black_king: u64,
    pub(crate) white_pawns: u64,
    pub(crate) white_rooks: u64,
    pub(crate) white_knights: u64,
    pub(crate) white_bishops: u64,
    pub(crate) white_queen: u64,
    pub(crate) white_king: u64,
    pub(crate) white_turn: bool,    // True if it's white's turn
    pub(crate) castling_rights: u8, // KQkq will be represented by 4 bits
    pub(crate) en_passant: u8,      //a square that has en passant ability (1-64)
}

impl Chessboard {
    /// Create a new instance of a chessboard, setup to start a new game.
    pub fn new() -> Chessboard {
        Chessboard {
            white_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            white_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            white_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            white_king: 0b0000000000000000000000000000000000000000000000000000000000001000,
            white_queen: 0b0000000000000000000000000000000000000000000000000000000000010000,
            white_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            black_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
            black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            black_king: 0b0000100000000000000000000000000000000000000000000000000000000000,
            black_queen: 0b0001000000000000000000000000000000000000000000000000000000000000,
            black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
            castling_rights: 0b1111,
            en_passant: 0,
            white_turn: true,
        }
    }

    /// Create a new instance of a chessboard, with no pieces on it.
    pub fn empty() -> Chessboard {
        Chessboard {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_king: 0,
            white_queen: 0,
            white_rooks: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_king: 0,
            black_queen: 0,
            black_rooks: 0,
            castling_rights: 0,
            en_passant: 0,
            white_turn: true,
        }
    }


    /// Create a new instance of a chessboard, based on a FEN string
    /// Forsyth–Edwards Notation Parser
    /// * `fen` - The FEN to be converted to a Chessboard.
    /// # Return: Chessboard with the position from the FEN.
    pub fn from_string(fen: &str) -> Chessboard {
        let mut chessboard = Chessboard::new();

        // Split the FEN string into parts using ' ' as the delimiter
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();

        // Parse the piece placement part of the FEN string
        let board_rows: Vec<&str> = fen_parts[0].split('/').collect();
        for (mut rank, row) in board_rows.iter().rev().enumerate() {
            rank += 1;
            let mut file = 0; // Initialize the file (column) index
            for piece in row.chars() {
                if piece.is_digit(10) {
                    let empty_squares = piece.to_digit(10).unwrap() as usize;
                    file += empty_squares; // Skip empty squares
                } else {
                    let square_index = 8 * (rank - 1) + file;
                    // Update the corresponding bitboard based on the piece type and color
                    match piece {
                        'p' => chessboard.black_pawns |= pow(2, square_index),
                        'r' => chessboard.black_rooks |= pow(2, square_index),
                        'b' => chessboard.black_bishops |= pow(2, square_index),
                        'k' => chessboard.black_king |= pow(2, square_index),
                        'q' => chessboard.black_queen |= pow(2, square_index),
                        'n' => chessboard.black_knights |= pow(2, square_index),
                        'P' => chessboard.white_pawns |= pow(2, square_index),
                        'R' => chessboard.white_rooks |= pow(2, square_index),
                        'B' => chessboard.white_bishops |= pow(2, square_index),
                        'K' => chessboard.white_king |= pow(2, square_index),
                        'Q' => chessboard.white_queen |= pow(2, square_index),
                        'N' => chessboard.white_knights |= pow(2, square_index),
                        _ => { /* Handle other characters if needed */ }
                    }
                    file += 1; // Move to the next file
                }
            }
        }

        // Parse whose turn it is
        chessboard.white_turn = fen_parts[1] == "w";

        // Parse castling rights
        let fen_castle = fen_parts[2];
        for c in fen_castle.chars() {
            let v = match c {
                'K' => 0b1000,
                'Q' => 0b0100,
                'k' => 0b0010,
                'q' => 0b0001,
                _ => 0b0,
            };
            chessboard.castling_rights |= v;
        }

        // Parse en passant square
        let fen_passant = fen_parts[3];
        if fen_passant != "-" {
            if let (Some(col), Some(row)) = (
                fen_passant.chars().nth(0).map(|c| c.to_ascii_uppercase()),
                fen_passant.chars().nth(1).and_then(|c| c.to_digit(10)),
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
                        _ => 0, // Handle unexpected characters
                    };
                    chessboard.en_passant = col_value + 8 * (row as u8 - 1);
                }
            }
        }

        // Ignore the rest of the FEN string for now
        return chessboard;
    }


    /// Prints the chessboard to the console
    /// * `pretty` - Print with extra formatting
    pub fn print(&self, pretty: bool) {
        let ranks = [8, 7, 6, 5, 4, 3, 2, 1];
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        for rank in ranks.iter() {
            print!("{rank} ");
            for file in 0..files.len() {
                let piece = self.piece_at_position(*rank, file);
                if !pretty {
                    print!("{piece} ");
                    continue;
                }

                let fg = self.find_fg(piece);
                let frmt_piece = format!("{:^3}", piece);
                let bk = self.find_bkgnd(*rank, file);
                let _ = execute!(
                    stdout(),
                    SetForegroundColor(fg),
                    SetBackgroundColor(bk),
                    Print(frmt_piece),
                    ResetColor
                );
            }
            println!();
        }

        print!("  ");
        for file in files.iter() {
            if pretty {
                print!(" {file} ")
            } else {
                print!("{file} ");
            }
        }
        println!();
        return;
    }


    /* *************** */
    /* PRIVATE FUNCTIONS */

    /// Maps the pieces on the board to the character that represents them in the console.
    /// # Return:
    /// A vector of tuples, where each tuple contains a chess piece character and it's
    /// corresponding bitboard positions.
    fn get_pieces(&self) -> Vec<(char, u64)> {
        vec![
            ('P', self.white_pawns),
            ('N', self.white_knights),
            ('B', self.white_bishops),
            ('K', self.white_king),
            ('Q', self.white_queen),
            ('R', self.white_rooks),
            ('p', self.black_pawns),
            ('n', self.black_knights),
            ('b', self.black_bishops),
            ('k', self.black_king),
            ('q', self.black_queen),
            ('r', self.black_rooks),
        ]
    }

    /// Foreground color to display for this piece
    /// # Return: The color of the piece
    #[rustfmt::skip]
    fn find_fg(&self, p: char) -> Color {
        if p.is_uppercase() { Color::White }
        else                { Color::Black }
    }

    /// # Return: The color of the board at this position
    fn find_bkgnd(&self, rank: usize, file: usize) -> Color {
        if (rank + file) % 2 == 0 {
            return Color::Rgb {
                r: 190,
                g: 140,
                b: 170,
            };
        } else {
            return Color::Rgb {
                r: 255,
                g: 206,
                b: 158,
            };
        }
    }

    /// Retrieve the chess piece at a specific position on the chessboard.
    /// * `rank` - The rank of the square.
    /// * `file` - The file (A=0) of the square.
    /// # Return:
    /// The character representation of the piece at this position.
    /// If there is no piece here it will return a period.
    fn piece_at_position(&self, rank: usize, file: usize) -> char {
        for (p_type, positions) in self.get_pieces() {
            let rank_byte = positions >> ((rank - 1) * 8);
            if (rank_byte & (1 << file)) != 0 {
                return p_type;
            }
        }
        '.'
    }

    /// Forsyth–Edwards Notation Serializer
    /// * `chessboard` - The chessboard position to be converted to a FEN.
    /// # Return: FEN string representing the board's position.
    pub fn to_string(&self) -> String {
        // Initialize a vector to store FEN components as strings
        let mut string_array: Vec<String> = Vec::with_capacity(6);
        let mut fen_string;

        // Piece placement
        for rank in (1..=8).rev() {
            let mut empty_squares = 0;
            let mut row_string = String::new();

            // Iterate through each file (column) in the rank
            for file in 1..=8 {
                let square_ndx = (rank - 1) * 8 + (file - 1);

                for &(piece, mask) in self.get_pieces().iter() {
                    if (mask >> square_ndx) & 1 != 0 {
                        if empty_squares > 0 { 
                            row_string.push_str(&empty_squares.to_string());
                        }
                        row_string.push(piece);
                        empty_squares = 0;
                    }
                }
            }

            // Append the count of empty squares at the end of the row string
            if empty_squares > 0 {
                row_string.push_str(&empty_squares.to_string());
            }

            // Add the row string to the FEN components vector
            string_array.push(row_string);
            string_array.push("/".to_owned());
        }

        let binding = string_array.concat();
        fen_string = binding.chars();
        fen_string.next_back();
        let fen_string_no_slash = fen_string.as_str();
        for item in &mut string_array {
            item.clear();
        }
        string_array.push(fen_string_no_slash.to_owned());

        string_array.push(" ".to_owned());

        // Whose turn
        string_array.push(if self.white_turn {
            "w ".to_string()
        } else {
            "b ".to_string()
        });

        // Castling rights
        string_array.push(match self.castling_rights {
            0 => "- ".to_string(),
            rights => {
                let mut rights_string = String::new();

                // Check individual castling rights and append to rights_string
                if rights & 0b1000 != 0 {
                    rights_string.push('K');
                }
                if rights & 0b0100 != 0 {
                    rights_string.push('Q');
                }
                if rights & 0b0010 != 0 {
                    rights_string.push('k');
                }
                if rights & 0b0001 != 0 {
                    rights_string.push('q');
                }

                rights_string.push(' ');
                rights_string
            }
        });

        // En passant
        if self.en_passant == 0 {
            string_array.push("- ".to_string());
        } else {
            // Convert en passant square to algebraic notation
            let row = (self.en_passant - 1) / 8 + 1;
            let col = (self.en_passant - 1) % 8;

            let column_char = (b'A' + col) as char;

            string_array.push(format!("{}{}", column_char, row));
            string_array.push(" ".to_owned());
        }

        // Set the rest to default values
        string_array.push("0 ".to_string());
        string_array.push("1".to_string());

        // Return the FEN string
        string_array.concat()
    }
}

//MINIMAX Function Pseudo-code
// fn minimax(position, depth, alpha, beta, maximizing_player) {
//     if depth == 0 or game over in position
//         return static evaluation of position
//     if maximizing_player (white)
//         max_eval = -infinity
//         for each child of position
//             eval = minimax(child, depth - 1, alpha, beta, false)
//             max_eval = max(max_eval, eval)
//             alpha = max(alpha, eval)
//             if beta <= alpha
//                 break
//         return max_eval
//     else
//         min_eval = +infinity
//         for each child of position
//         eval = minimax(child, depth - 1, alpha, beta, true)
//         min_eval = min(min_eval, eval)
//         beta = min(beta, eval)
//         if beta <= alpha
//             break
//     return min_eval
// }
