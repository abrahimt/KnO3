extern crate num_traits;
use std::string;

use num_traits::pow;

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
    pub(crate) white_turn: bool,
    pub(crate) castling_rights: u8, //KQkq
    pub(crate) en_passant: u32, //a square that has en passant ability (1-64) 0 means no en passant
}

impl Chessboard {
    // Initializes the chessboard with the starting positions of all the pieces
    // and resets turn, castling, and en passant.
    pub fn initialize_board(&mut self) {
        // white pieces
        self.white_pawns = 0b0000000000000000000000000000000000000000000000001111111100000000;
        self.white_knights = 0b0000000000000000000000000000000000000000000000000000000001000010;
        self.white_bishops = 0b0000000000000000000000000000000000000000000000000000000000100100;
        self.white_king = 0b0000000000000000000000000000000000000000000000000000000000001000;
        self.white_queen = 0b0000000000000000000000000000000000000000000000000000000000010000;
        self.white_rooks = 0b0000000000000000000000000000000000000000000000000000000010000001;
        // black pieces
        self.black_pawns = 0b0000000011111111000000000000000000000000000000000000000000000000;
        self.black_knights = 0b0100001000000000000000000000000000000000000000000000000000000000;
        self.black_bishops = 0b0010010000000000000000000000000000000000000000000000000000000000;
        self.black_king = 0b0000100000000000000000000000000000000000000000000000000000000000;
        self.black_queen = 0b0001000000000000000000000000000000000000000000000000000000000000;
        self.black_rooks = 0b1000000100000000000000000000000000000000000000000000000000000000;
        // turn
        self.white_turn = true;
        // castling
        self.castling_rights = 0b1111;
        // en_passant
        self.en_passant = 0;
    }

    // This function returns a string representing whose turn it is in the chess game.
    // It checks the boolean flag `white_turn` to determine if it's white's turn or black's turn.
    fn whose_turn(&self) -> &str {
        if self.white_turn {
            "white"
        } else {
            "black"
        }
    }

    // Parser function that converts a FEN (Forsyth–Edwards Notation) string to a Chessboard struct
    pub fn from_string(&self, fen: &str) -> Chessboard {
        // Initialize a new Chessboard with default values
        let mut chessboard = Chessboard {
            // Initialize bitboards for each piece and other game state variables
            black_pawns: 0,
            black_rooks: 0,
            black_knights: 0,
            black_bishops: 0,
            black_queen: 0,
            black_king: 0,
            white_pawns: 0,
            white_rooks: 0,
            white_knights: 0,
            white_bishops: 0,
            white_queen: 0,
            white_king: 0,
            castling_rights: 0,
            white_turn: true,
            en_passant: 0,
        };

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
                    if piece.is_ascii_lowercase() {
                        // Black pieces
                        match piece {
                            'p' => chessboard.black_pawns |= pow(2, square_index),
                            'r' => chessboard.black_rooks |= pow(2, square_index),
                            'b' => chessboard.black_bishops |= pow(2, square_index),
                            'k' => chessboard.black_king |= pow(2, square_index),
                            'q' => chessboard.black_queen |= pow(2, square_index),
                            'n' => chessboard.black_knights |= pow(2, square_index),
                            _ => { /* Handle other lowercase characters if needed */ }
                        }
                    } else {
                        // White pieces
                        match piece {
                            'P' => chessboard.white_pawns |= pow(2, square_index),
                            'R' => chessboard.white_rooks |= pow(2, square_index),
                            'B' => chessboard.white_bishops |= pow(2, square_index),
                            'K' => chessboard.white_king |= pow(2, square_index),
                            'Q' => chessboard.white_queen |= pow(2, square_index),
                            'N' => chessboard.white_knights |= pow(2, square_index),
                            _ => { /* Handle other uppercase characters if needed */ }
                        }
                    }
                    file += 1; // Move to the next file
                }
            }
        }

        // Parse whose turn it is
        chessboard.white_turn = fen_parts[1] == "w";

        // Parse castling rights
        let fen_castle = fen_parts[2];
        let mut castles = 0;
        for c in fen_castle.chars() {
            let v = match c {
                'K' => 0b1000,
                'Q' => 0b0100,
                'k' => 0b0010,
                'q' => 0b0001,
                _ => 0b0,
            };
            castles |= v;
        }
        chessboard.castling_rights = castles;

        // Parse en passant square
        let fen_passant = fen_parts[3];
        if fen_passant != "-" {
            if let (Some(col), Some(row)) = (
                fen_passant.chars().nth(0).map(|c| c.to_ascii_uppercase()),
                fen_passant.chars().nth(1).and_then(|c| c.to_digit(10)),
            ) {
                if (1..=8).contains(&row) {
                    let col_value = match col {
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
                    chessboard.en_passant = col_value + 8 * (row - 1);
                }
            }
        }

        // Ignore the rest of the FEN string for now
        return chessboard;
    }

    // Serializer function that converts a Chessboard struct to a FEN (Forsyth–Edwards Notation) string
    pub fn to_string(chessboard: Chessboard) -> String {
        // Initialize a vector to store FEN components as strings
        let mut string_array: Vec<String> = Vec::with_capacity(6);
        let fen_string;

        // Piece placement


        // Whose turn
        string_array.push(if chessboard.white_turn {
            "w ".to_string()
        } else {
            "b ".to_string()
        });

        // Castling rights
        string_array.push(match chessboard.castling_rights {
            0 => "- ".to_string(),
            rights => {
                let mut rights_string = String::new();

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

                rights_string
            }
        });

        // En passant
        if chessboard.en_passant == 0 {
            string_array.push("- ".to_string());
        } else {
            // Convert en passant square to algebraic notation
            let row = (chessboard.en_passant - 1) / 8 + 1;
            let col = (chessboard.en_passant - 1) % 8;

            let column_char = (b'A' + col) as char;

            string_array.push(format!("{}{}", column_char, row));
        }

        // Set the rest to default values
        string_array.push("0 ".to_string());
        string_array.push("1".to_string());

        // Combine array elements into a single string
        fen_string = string_array.concat();

        // Print the FEN string (for debugging purposes)
        println!("{:?}", fen_string);

        // Return the FEN string
        fen_string
    }

    //MINIMAX Function Pseudo-code
    // fn minimax(position, depth, alpha, beta, maximixing_player) {
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
}
