use crate::fen_util;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::{io::stdout, u8};

/// Struct representing a chessboard with piece positions and game state.
///
/// Each `piece` is a uint64 bitboard, where each bit represents the presence of a piece
/// on the corresponding square. The bit at index `i` represents the presence of a piece on
/// the square at rank `i/8` and file `i%8`.
///
/// # Fields
///
/// - `black_pawns`: Bitboard representing the positions of black pawns.
/// - `black_rooks`: Bitboard representing the positions of black rooks.
/// - `black_knights`: Bitboard representing the positions of black knights.
/// - `black_bishops`: Bitboard representing the positions of black bishops.
/// - `black_queen`: Bitboard representing the position of the black queen.
/// - `black_king`: Bitboard representing the position of the black king.
/// - `white_pawns`: Bitboard representing the positions of white pawns.
/// - `white_rooks`: Bitboard representing the positions of white rooks.
/// - `white_knights`: Bitboard representing the positions of white knights.
/// - `white_bishops`: Bitboard representing the positions of white bishops.
/// - `white_queen`: Bitboard representing the position of the white queen.
/// - `white_king`: Bitboard representing the position of the white king.
/// - `white_turn`: Boolean indicating whether it's white's turn to move.
/// - `castling_rights`: 4-bit value representing the castling rights, where the bits represent
///   kingside and queenside castling rights for both black and white.
/// - `en_passant`: 6-bit value representing the square that has en passant ability (1-64),
///   or 0 if there is no en passant square.
///
/// Note: The board is represented as a set of bitboards for each piece type, and the
/// `en_passant` square is represented using 6 bits, allowing values 1-64 to represent each
/// square on the board. The `castling_rights` field uses 4 bits to represent kingside and
/// queenside castling rights for both black and white.
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

    /// Creates a new instance of a chessboard, set up to start a new game.
    ///
    /// # Returns
    ///
    /// A `Chessboard` with the initial setup for a new game. The bitboards for each piece
    /// type are initialized to represent their starting positions on the chessboard. The
    /// castling rights, en passant square, and whose turn it is (white's turn initially) are
    /// also set to their default values.
    ///
    /// # Example
    ///
    /// ```
    /// use chess_engine::Chessboard;
    ///
    /// let new_game_board = Chessboard::new();
    /// println!("New Game Chessboard:\n{:#?}", new_game_board);
    /// ```
    ///
    /// The function creates a new instance of a `Chessboard` with the starting position for
    /// a new game. It can be used to initialize the chessboard at the beginning of a chess match.
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

    /// Creates a new instance of a chessboard based on a Forsyth–Edwards Notation (FEN) string.
    ///
    /// # Arguments
    ///
    /// - `fen`: The FEN string to be converted to a `Chessboard`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the resulting `Chessboard` with the position from the FEN string.
    /// If the FEN string is invalid, an `Err` variant with an error message is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use chess_engine::Chessboard;
    ///
    /// let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// match Chessboard::from_string(fen_string) {
    ///     Ok(chessboard) => {
    ///         println!("Chessboard created from FEN:\n{:#?}", chessboard);
    ///     },
    ///     Err(error) => {
    ///         println!("Error creating chessboard: {}", error);
    ///     },
    /// }
    /// ```
    ///
    /// The function parses a valid FEN string and constructs a `Chessboard` with the specified
    /// piece placement, turn, castling rights, and en passant details. If the FEN string is
    /// invalid, it returns an `Err` variant with an error message.
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

    /// Prints the chessboard to the console.
    ///
    /// # Arguments
    ///
    /// - `pretty`: A boolean indicating whether to print with extra formatting.
    ///
    /// # Example
    ///
    /// ```
    /// use chess_engine::Chessboard;
    ///
    /// let initial_position = Chessboard::new();
    /// initial_position.print(true);
    /// ```
    ///
    /// The function prints the current state of the chessboard to the console. If `pretty` is
    /// set to `true`, it adds extra formatting, including colors for pieces and backgrounds.
    /// Otherwise, it prints a simple representation of the board with piece characters.
    ///
    /// Note: The function uses ANSI escape codes for color formatting when `pretty` is set to
    /// `true`. The piece characters are centered within each square for improved display.
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
    }

    /// Retrieves the chess piece at a specific position on the chessboard.
    ///
    /// # Arguments
    ///
    /// - `rank`: The rank of the square (1-indexed).
    /// - `file`: The file (A=0) of the square (0-indexed).
    ///
    /// # Returns
    ///
    /// The character representation of the piece at the specified position. If there is no piece
    /// at the given position, it returns a period ('.').
    ///
    /// # Example
    ///
    /// ```
    /// use chess_engine::Chessboard;
    ///
    /// let initial_position = Chessboard::new();
    /// let piece_at_a1 = initial_position.piece_at_position(1, 0);
    /// println!("Piece at a1: {}", piece_at_a1);
    /// ```
    ///
    /// The function iterates through each piece type on the chessboard, checking whether the
    /// specified position is occupied by a piece of that type. If a piece is found, its character
    /// representation is returned. If no piece is found at the specified position, a period ('.')
    /// is returned.
    pub fn piece_at_position(&self, rank: usize, file: usize) -> char {
        for (p_type, positions) in self.get_pieces() {
            let rank_byte = positions >> ((rank - 1) * 8);
            if (rank_byte & (1 << file)) != 0 {
                return p_type;
            }
        }
        '.'
    }

    /// Serializes a chessboard position into Forsyth–Edwards Notation (FEN).
    ///
    /// # Arguments
    ///
    /// - `chessboard`: The chessboard position to be converted to FEN.
    ///
    /// # Returns
    ///
    /// A FEN string representing the board's position.
    ///
    /// # Example
    ///
    /// ```
    /// use chess_engine::Chessboard;
    ///
    /// let initial_position = Chessboard::new();
    /// let fen_string = initial_position.to_string();
    /// println!("FEN: {}", fen_string);
    /// ```
    ///
    /// The resulting FEN string will consist of six space-separated fields:
    /// 1. Piece placement
    /// 2. Whose turn it is ('w' for white, 'b' for black)
    /// 3. Castling rights
    /// 4. En passant target square (or "-" if none)
    /// 5. Halfmove clock (always "0" in this implementation)
    /// 6. Fullmove number (always "1" in this implementation)
    ///
    /// Note: The FEN string is used to represent the state of a chess position in a concise
    /// and human-readable format. It captures information about piece placement, castling rights,
    /// en passant targets, and other game state details.
    #[allow(clippy::all)]
    pub fn to_string(self) -> String {
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

    /* *************** */
    /* PRIVATE FUNCTIONS */

    /// Maps the pieces on the chessboard to their character representations in the console.
    ///
    /// # Returns
    ///
    /// A vector of tuples, where each tuple consists of a chess piece character and its
    /// corresponding bitboard positions. The characters represent different chess pieces,
    /// and the bitboard positions indicate the squares occupied by those pieces on the board.
    ///
    /// # Example
    ///
    /// ```
    /// use chess_engine::Chessboard;
    ///
    /// let initial_position = Chessboard::new();
    /// let pieces_mapping = initial_position.get_pieces();
    /// for (piece_char, positions) in pieces_mapping {
    ///     println!("Piece: {} | Bitboard Positions: {}", piece_char, positions);
    /// }
    /// ```
    ///
    /// The function returns a vector containing tuples, each associating a chess piece
    /// character ('P', 'N', 'B', 'K', 'Q', 'R', 'p', 'n', 'b', 'k', 'q', 'r') with its
    /// corresponding bitboard positions on the chessboard.
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

    /// Determines the foreground color based on the chess piece color.
    ///
    /// # Arguments
    ///
    /// - `p`: The character representation of a chess piece.
    ///
    /// # Returns
    ///
    /// The `Color` of the chess piece. If the piece is uppercase (representing a white piece),
    /// it returns `Color::White`; otherwise, it returns `Color::Black`.
    ///
    /// # Example
    ///
    /// ```
    /// use chess_display::Color;
    /// use chess_display::ChessboardDisplay;
    ///
    /// let display = ChessboardDisplay::new();
    /// let piece_color = display.find_fg('K');
    /// println!("Color of white king: {:?}", piece_color);
    /// ```
    ///
    /// The function determines the color of a chess piece based on its character representation.
    /// Uppercase letters represent white pieces, and lowercase letters represent black pieces.
    /// The resulting color is returned as either `Color::White` or `Color::Black`.
    #[rustfmt::skip]
    fn find_fg(&self, p: char) -> Color {
        if p.is_uppercase() { Color::White }
        else                { Color::Black }
    }

    /// Determines the background color of the chessboard at a specific position.
    ///
    /// # Arguments
    ///
    /// - `rank`: The rank of the square (1-indexed).
    /// - `file`: The file (A=0) of the square (0-indexed).
    ///
    /// # Returns
    ///
    /// The `Color` of the board at the specified position. The color is represented by an RGB
    /// value. Light squares are represented by (r: 190, g: 140, b: 170), and dark squares are
    /// represented by (r: 255, g: 206, b: 158).
    ///
    /// # Example
    ///
    /// ```
    /// use chess_display::Color;
    /// use chess_display::ChessboardDisplay;
    ///
    /// let display = ChessboardDisplay::new();
    /// let background_color = display.find_bkgnd(1, 0);
    /// println!("Background color at a1: {:?}", background_color);
    /// ```
    ///
    /// The function calculates the background color of a square based on its position on the
    /// chessboard. Light squares are determined by an even sum of the rank and file, while dark
    /// squares are determined by an odd sum. The resulting color is returned as an RGB value.
    #[rustfmt::skip]
    fn find_bkgnd(&self, rank: usize, file: usize) -> Color {
        let lght = Color::Rgb { r: 190, g: 140, b: 170 };
        let dark = Color::Rgb { r: 255, g: 206, b: 158 };
        if (rank + file) % 2 == 0 { lght }
        else                      { dark }
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
