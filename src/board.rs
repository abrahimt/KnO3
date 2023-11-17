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
        if self.white_turn { "white" } else { "black" }
    }
    

    // parser
    pub fn from_string(&self, fen: &str) -> Chessboard {
        let mut chessboard = Chessboard {
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

        //Split fen with ' ' as delimiter
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();

        // Piece placement
        let board_rows: Vec<&str> = fen_parts[0].split('/').collect();
        for (mut rank, row) in board_rows.iter().rev().enumerate() {
            rank += 1;
            // Initialize the file (column) index
            let mut file = 0;
            // Iterate over each character in the FEN row
            for piece in row.chars() {
                if piece.is_digit(10) {
                    // If the character represents an empty square, skip that number of files
                    let empty_squares = piece.to_digit(10).unwrap() as usize;
                    file += empty_squares;
                } else {
                    // If the character represents a piece, update the corresponding bitboard
                    let square_index = 8 * (rank - 1) + file;

                    if piece.is_ascii_lowercase() {
                        match piece {
                            'p' => {
                                chessboard.black_pawns |= pow(2, square_index);
                            }
                            'r' => {
                                chessboard.black_rooks |= pow(2, square_index);
                            }
                            'b' => {
                                chessboard.black_bishops |= pow(2, square_index);
                            }
                            'k' => {
                                chessboard.black_king |= pow(2, square_index);
                            }
                            'q' => {
                                chessboard.black_queen |= pow(2, square_index);
                            }
                            'n' => {
                                chessboard.black_knights |= pow(2, square_index);
                            }
                            _ => {
                                // Handle other lowercase characters if needed
                            }
                        }
                    } else {
                        match piece {
                            'P' => {
                                chessboard.white_pawns |= pow(2, square_index);
                            }
                            'R' => {
                                chessboard.white_rooks |= pow(2, square_index);
                            }
                            'B' => {
                                chessboard.white_bishops |= pow(2, square_index);
                            }
                            'K' => {
                                chessboard.white_king |= pow(2, square_index);
                            }
                            'Q' => {
                                chessboard.white_queen |= pow(2, square_index);
                            }
                            'N' => {
                                chessboard.white_knights |= pow(2, square_index);
                            }
                            _ => {
                                // Handle other uppercase characters if needed
                            }
                        }
                    }
                    // Move to the next file
                    file += 1;
                }
            }
        }

        // Whose turn it is
        chessboard.white_turn = fen_parts[1] == "w";

        // Castling rights
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
            castles = castles | v;
        }
        chessboard.castling_rights = castles;

        // En passant square
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
                    //gives a number (1-64)
                    chessboard.en_passant = (9 - col_value) + 8 * (row - 1);
                }
            }
        }

        //Ignore rest of the FEN for now
        return chessboard;
    }

    // serializer
    pub fn to_string(chessboard: Chessboard) -> String {
        let mut string_array: Vec<String> = Vec::with_capacity(6);
        let fen_string;

        // Piece placement
        
        
        // Whose turn
        string_array.push(if chessboard.white_turn { "w ".to_string() } else { "b ".to_string() });

        // Castling rights
        string_array.push( match chessboard.castling_rights {
            0 => "- ".to_string(),
            rights => {
                let rights_string = "";

                if rights & 0b1000 != 0 {
                    rights_string.to_string().push('K');
                }
                if rights & 0b0100 != 0 {
                    rights_string.to_string().push('Q');
                }
                if rights & 0b0010 != 0 {
                    rights_string.to_string().push('k');
                }
                if rights & 0b0001 != 0 {
                    rights_string.to_string().push('q');
                }

                rights_string.to_string()
            }
        });

        // En passant
        if chessboard.en_passant == 0 {
            string_array.push("- ".to_string());
        } else {
            let row = (input - 1) / 8 + 1;
            let col = (input - 1) % 8;
        
            let column_char = (b'A' + col) as char;
        
            format!("{}{}", column_char, row)
        }

        // Set the rest to default
        string_array.push("0 ".to_string());
        string_array.push("1".to_string());

        // Combine array elements into a single string
        fen_string = string_array.concat();
        println!("{:?}", fen_string);
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
//Below here is just ideas for functions:

//fn what_occupy square() {
//     check each piece type with the square and see which returns true
// }

// fn is_square_occupied(square) {
// bitwise and every piece together
// search through the new u64 and see where there are 1s
// return the indices of the 1s
// }

// en_passant_square() {
//     // Return the square where an en passant capture is possible, or -1 if not possible
//     // For example, if the last move was a double-step pawn move, return the square where the opponent's pawn can capture en passant.
// }

// castling_rights() {
//     // Use flags or bitmasks to represent castling rights for both players
//     // For example, if kingside castling is allowed for white, set white_kingside_castle = true
//     // Similarly, maintain flags for black's castling rights
// }

// make_move(move) {
//     // Update the bitboards and game state based on the move
//     // For example, if it's a pawn move, update the pawn bitboard
//     // If it's a capture, clear the bit for the captured piece
//     // Update en passant square and castling rights accordingly
//     // Switch turns: white_turn = !white_turn;
// }

// generate_legal_moves() {
//     legal_moves = []

//     for each piece in pieces_of_current_turn {
//         possible_moves = generate_moves_for_piece(piece)
//         for each move in possible_moves {
//             if is_legal_move(move) {
//                 legal_moves.push(move)
//             }
//         }
//     }
//
//     return legal_moves
// }

// generate_moves_for_piece(piece) {
//     // Generate all possible moves for the given piece
//     // Consider piece-specific movement rules (e.g., pawn's initial double move, castling for king, etc.)
// }

// is_legal_move(move) {
//     // Check if the given move is legal
//     // Verify that the move adheres to the rules of chesce-s, including piespecific rules and board state
//     // Check for legality includes considerations like not moving into check, en passant captures, castling rules, etc.
// }
