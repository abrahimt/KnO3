use crate::fen_util;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
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
    /* *********** */
    /* Constructors */

    /// Create a new instance of a chessboard, setup to start a new game.
    pub fn new() -> Chessboard {
        Chessboard {
            white_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            white_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            white_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            white_king: 0b0000000000000000000000000000000000000000000000000000000000010000,
            white_queen: 0b0000000000000000000000000000000000000000000000000000000000001000,
            white_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            black_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
            black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            black_king: 0b0001000000000000000000000000000000000000000000000000000000000000,
            black_queen: 0b0000100000000000000000000000000000000000000000000000000000000000,
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

    /// Create a new instance of a chessboard, based on a FEN string.
    /// Forsyth–Edwards Notation Parser.
    ///
    /// # Arguments
    ///
    /// * `fen` - The FEN to be converted to a Chessboard.
    ///
    /// # Return
    ///
    /// Resulting chessboard with the position from the FEN.
    pub fn from_string(fen: &str) -> Result<Chessboard, String> {
        if !fen_util::valid_fen(fen) {
            return Err("Invalid FEN".to_string());
        }

        let mut chessboard = Chessboard::empty();

        let fen_parts: Vec<&str> = fen.split_whitespace().collect();

        fen_util::parse_piece_placement(&mut chessboard, fen_parts[0])?;
        fen_util::parse_whose_turn(&mut chessboard, fen_parts[1]);
        fen_util::parse_castling_rights(&mut chessboard, fen_parts[2]);
        fen_util::parse_en_passant(&mut chessboard, fen_parts[3]);

        Ok(chessboard)
    }

    /* **************** */
    /* Public Functions */

    /// Prints the chessboard to the console
    /// * `pretty` - Print with extra formatting
    pub fn print(&self, pretty: bool) {
        let ranks = [8, 7, 6, 5, 4, 3, 2, 1];
        let files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

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
    }

    /// Generates a list of legal moves for the current position.
    ///
    /// # Returns
    ///
    /// A vector of legal moves represented as strings. Each string should follow the
    /// algebraic notation for chess moves.
    // pub fn legal_moves(&mut self) -> Vec<String> {
    //     let mut legal_moves = Vec::new();

    //     // Iterate through each piece type on the board
    //     for (piece, positions) in self.get_pieces() {
    //         // Iterate through each square on the board
    //         for rank in 1..=8 {
    //             for file in 0..8 {
    //                 // Check if the current piece exists on the current square
    //                 let rank_byte = positions >> ((rank - 1) * 8);
    //                 if (rank_byte & (1 << file)) != 0 {
    //                     // Generate legal moves for the piece on this square
    //                     let moves = self.generate_moves(piece, rank, file);
    //                     legal_moves.extend(moves);
    //                 }
    //             }
    //         }
    //     }

    //     legal_moves
    // }

    /// Generates legal moves for a piece at a specific position.
    ///
    /// # Arguments
    ///
    /// - `piece`: The character representation of the piece.
    /// - `rank`: The rank of the square (1-indexed).
    /// - `file`: The file (A=0) of the square (0-indexed).
    ///
    /// # Returns
    ///
    /// A vector of legal moves represented as strings in algebraic notation.
    // pub fn generate_moves(&mut self, piece: char, rank: usize, file: usize) -> Vec<String> {
    //     // Implement logic to generate legal moves for each piece type
    //     // Use the current state of the board to determine legal moves
    //     let file_char = (b'A' + file as u8) as char;
    //     let new_board = Chessboard::new();
    //     // Example: Generate legal moves for a pawn
    //     // Contains all white pieces
    //     if self.white_turn {
    //         if piece == 'P' {
    //             // TODO: If last rank before promotion (move with promotion)

    //             //If starting square, then can go 1 or 2 moves
    //             //no en passant
    //             if new_board.piece_at_position(rank, file) == self.piece_at_position(rank, file) {
    //                 //check for capture
    //                 if self
    //                     .piece_at_position(
    //                         if rank + 1 > 8 { 7 } else { rank + 1 },
    //                         if file + 1 > 8 { 7 } else { file + 1 },
    //                     )
    //                     .is_lowercase()
    //                 {
    //                     //can capture
    //                 } else if self
    //                     .piece_at_position(
    //                         if rank - 1 < 0 { 1 } else { rank - 1 },
    //                         if file - 1 < 0 { 1 } else { file - 1 },
    //                     )
    //                     .is_lowercase()
    //                 {
    //                     //can capture
    //                 }
    //                 if self.piece_at_position(rank + 1, file) == '.' {
    //                     //can move 1 forward
    //                 }
    //                 if self.piece_at_position(rank + 1, file) == '.'
    //                     && self.piece_at_position(rank + 2, file) == '.'
    //                 {
    //                     //can move 2 forward
    //                     //set en passant
    //                     self.en_passant = rank_file_to_square(rank + 1, file_char) as u8;
    //                 }
    //             } else {
    //                 //check for capture
    //                 if self
    //                     .piece_at_position(
    //                         if rank + 1 > 8 { 7 } else { rank + 1 },
    //                         if file + 1 > 8 { 7 } else { file + 1 },
    //                     )
    //                     .is_lowercase()
    //                 {
    //                     //can capture
    //                 } else if self
    //                     .piece_at_position(
    //                         if rank - 1 < 0 { 1 } else { rank - 1 },
    //                         if file - 1 < 0 { 1 } else { file - 1 },
    //                     )
    //                     .is_lowercase()
    //                 {
    //                     //can capture
    //                 }
    //                 //check for en passant
    //                 if self.en_passant != 0 {
    //                     if rank_file_to_square(rank, file_char) == self.en_passant {
    //                         //can en passant
    //                     }
    //                 }
    //                 if self.piece_at_position(rank + 1, file) == '.' {
    //                     //move 1
    //                 }
    //             }
    //         }
    //     } else {
    //         if piece == 'p' {
    //             //can only move if black turn
    //             //If starting square, then can go 1 or 2 moves
    //             //else if not starting square can go one move
    //             //if en passant square is diagonal from it then it can move to that square
    //             //if other color piece is diagonal from it then it can move to that square and capture
    //             //if it reaches other side of board it can promote to same color of any piece type
    //         }
    //         //handle random pieces that are wrong
    //     }

    //     Vec::new() // Placeholder, replace with actual legal moves
    // }

    pub fn move_piece(&mut self, current_pos: &str, new_pos: &str, piece: char) {
        let two: u64 = 2;
        if let (Some(old_file), Some(old_rank), Some(new_file), Some(new_rank)) = (
            current_pos.chars().next(),
            current_pos.chars().next_back(),
            new_pos.chars().next(),
            new_pos.chars().next_back(),
        ) {
            let old_square =
                fen_util::rank_file_to_square(old_rank.to_digit(10).unwrap() as u8, old_file);
            let new_square =
                fen_util::rank_file_to_square(new_rank.to_digit(10).unwrap() as u8, new_file);
            let clear_old = !two.pow(old_square.try_into().unwrap());
            let add_new = two.pow(new_square.try_into().unwrap());

            // Delete the piece from the old square
            match piece {
                'p' => {
                    self.black_pawns &= clear_old; // Clear old position
                    self.black_pawns |= add_new; // Set new position
                }
                'r' => {
                    self.black_rooks &= clear_old;
                    self.black_rooks |= add_new;
                }
                'b' => {
                    self.black_bishops &= clear_old;
                    self.black_bishops |= add_new;
                }
                'k' => {
                    self.black_king &= clear_old;
                    self.black_king |= add_new;
                }
                'q' => {
                    self.black_queen &= clear_old;
                    self.black_queen |= add_new;
                }
                'n' => {
                    self.black_knights &= clear_old;
                    self.black_knights |= add_new;
                }
                'P' => {
                    self.white_pawns &= clear_old;
                    self.white_pawns |= add_new;
                }
                'R' => {
                    self.white_rooks &= clear_old;
                    self.white_rooks |= add_new;
                }
                'B' => {
                    self.white_bishops &= clear_old;
                    self.white_bishops |= add_new;
                }
                'K' => {
                    self.white_king &= clear_old;
                    self.white_king |= add_new;
                }
                'Q' => {
                    self.white_queen &= clear_old;
                    self.white_queen |= add_new;
                }
                'N' => {
                    self.white_knights &= clear_old;
                    self.white_knights |= add_new;
                }
                //_ => { return Err("Invalid piece in FEN string".to_string()); }
                _ => {}
            }
        } else {
            // Handle the case when unwrapping fails (e.g., invalid input)
            println!("Invalid input positions");
        }
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
    #[rustfmt::skip]
    fn find_bkgnd(&self, rank: usize, file: usize) -> Color {
        let lght = Color::Rgb { r: 190, g: 140, b: 170 };
        let dark = Color::Rgb { r: 255, g: 206, b: 158 };
        if (rank + file) % 2 == 0 { dark }
        else                      { lght }
    }

    /// Retrieve the chess piece at a specific position on the chessboard.
    /// * `rank` - The rank of the square (1-indexed).
    /// * `file` - The file (A=0) of the square (0-indexed).
    /// # Return:
    /// The character representation of the piece at this position.
    /// If there is no piece here, it will return a period.
    pub fn piece_at_position(&self, rank: usize, file: usize) -> char {
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
    #[allow(clippy::all)]
    pub fn to_string(&self) -> String {
        let mut string_array: [&str; 6] = ["", "", "", "", "", ""];

        // Piece placement
        let pieces = &fen_util::get_fen_placement(&self);
        string_array[0] = pieces;

        // Whose turn
        string_array[1] = if self.white_turn { "w" } else { "b" };

        // Castling rights
        let castle = &fen_util::get_fen_castles(&self);
        string_array[2] = castle;

        // En passant
        let passant = &fen_util::get_fen_passant(&self);
        string_array[3] = passant;

        // Set the rest to default values
        string_array[4] = "0";
        string_array[5] = "1";

        // Return the FEN string
        string_array.join(" ")
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
