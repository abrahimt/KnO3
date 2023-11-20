use crate::fen_util;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::{error::Error, io::stdout, u8};

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
    /* Constructos */

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
    #[rustfmt::skip]
    fn find_bkgnd(&self, rank: usize, file: usize) -> Color {
        let lght = Color::Rgb { r: 190, g: 140, b: 170 };
        let dark = Color::Rgb { r: 255, g: 206, b: 158 };
        if (rank + file) % 2 == 0 { lght }
        else                      { dark }
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
    pub fn to_string(&mut self) -> String {
        let mut string_array: [&str; 6];

        // Piece placement
        fen_util::get_fen_placement(self, &mut string_array);

        // Whose turn
        string_array[1] = if self.white_turn { " w " } else { " b " };

        // Castling rights
        fen_util::get_fen_castles(self, &mut string_array);

        // En passant
        fen_util::get_fen_passant(self, &mut string_array);

        // Set the rest to default values
        string_array[4] = "0 ";
        string_array[5] = "1";

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
