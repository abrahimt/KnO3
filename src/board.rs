use crate::fen_util;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::{io::stdout, u8};

/// Struct representing a chessboard with piece positions and game state.
///
/// Each `piece` is a uint64 bitboard, where each bit represents a square on the board.
/// A set bit indicates this piece is present on that corresponding square.
///
/// Note: The board is represented as a set of bitboards for each piece type, and the
/// `en_passant` square is represented using 6 bits, allowing values 1-64 to represent each
/// square on the board. The `castling_rights` field uses 4 bits to represent kingside and
/// queenside castling rights for both black and white. Castle white king side = 8, castle
/// white queen side = 4, castle black king side = 2, caslte black queen side = 1.
#[rustfmt::skip]
pub struct Chessboard {
    pub black_pawns:   u64,
    pub black_rooks:   u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_queen:   u64,
    pub black_king:    u64,
    pub white_pawns:   u64,
    pub white_rooks:   u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_queen:   u64,
    pub white_king:    u64,
    pub white_turn: bool,    // True if it's white's turn
    pub castling_rights: u8, // KQkq will be represented by 4 bits
    pub en_passant: u8,      // a square that has en passant ability (1-64)
}

impl Chessboard {
    /* *********** */
    /* Constructorrs */

    /// Creates a new instance of a chessboard, set up to start a new game.
    ///
    /// # Returns
    ///
    /// A `Chessboard` with the initial setup for a new game. The bitboards for each piece
    /// type are initialized to represent their starting positions on the chessboard. The
    /// castling rights, en passant square, and whose turn it is (white's turn initially) are
    /// also set to their default values.
    ///
    /// The function creates a new instance of a `Chessboard` with the starting position for
    /// a new game. It can be used to initialize the chessboard at the beginning of a chess match.
    #[rustfmt::skip]
    pub fn new() -> Chessboard {
        Chessboard {
            white_pawns:   0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
            white_knights: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            white_bishops: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            white_king:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            white_queen:   0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            white_rooks:   0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            black_pawns:   0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            black_knights: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_bishops: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_king:    0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_queen:   0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_rooks:   0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            castling_rights: 0b1111,
            en_passant: 0,
            white_turn: true,
        }
    }

    /// Creates a new instance of a chessboard with no pieces on it.
    ///
    /// # Returns
    ///
    /// A `Chessboard` struct initialized with empty positions for all pieces.
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

    /// Clears an instance of a chessboard.
    pub fn clear(&mut self) {
        self.white_pawns = 0;
        self.white_knights = 0;
        self.white_bishops = 0;
        self.white_king = 0;
        self.white_queen = 0;
        self.white_rooks = 0;
        self.black_pawns = 0;
        self.black_knights = 0;
        self.black_bishops = 0;
        self.black_king = 0;
        self.black_queen = 0;
        self.black_rooks = 0;
        self.castling_rights = 0;
        self.en_passant = 0;
        self.white_turn = true;
    }

    /// Creates a new instance of a chessboard based on a Forsyth–Edwards Notation (FEN) string.
    ///
    /// # Arguments
    ///
    /// - `fen`: The FEN string to be converted to a `Chessboard`.
    ///
    /// # Returns
    ///
    /// An `Option` containing the resulting `Chessboard` with the game state from the FEN string.
    ///
    /// # Example
    ///
    /// ```
    /// use kn_o3::board::Chessboard;
    /// let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let cb = match Chessboard::from_string(fen_string) {
    ///   Some(cb) => cb,
    ///   None => panic!("Failed to create chessboard")
    /// };
    /// ```
    pub fn from_string(fen: &str) -> Option<Chessboard> {
        if !fen_util::valid_fen(fen) {
            return None;
        }

        let mut chessboard = Chessboard::empty();
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();

        fen_util::place_pieces(&mut chessboard, fen_parts[0]);
        fen_util::parse_whose_turn(&mut chessboard, fen_parts[1]);
        fen_util::parse_castling_rights(&mut chessboard, fen_parts[2]);
        fen_util::parse_en_passant(&mut chessboard, fen_parts[3]);

        Some(chessboard)
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
    /// use kn_o3::board::Chessboard;
    /// Chessboard::new().print(true);
    /// ```
    ///
    /// The function prints the current state of the chessboard to the console. If `pretty` is
    /// set to `true`, it adds extra formatting, including colors for pieces and backgrounds.
    /// Otherwise, it prints a simple representation of the board with piece characters.
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

    /// Converts a square index (0-63) to its corresponding chess rank and file.
    ///
    /// # Arguments
    ///
    /// * `square` - The square index (0-63) on the chessboard.
    ///
    /// # Returns
    ///
    /// A tuple containing the corresponding file and rank for the given square.
    ///
    /// # Example
    ///
    /// ```
    /// use kn_o3::board::Chessboard;
    /// let (file, rank) = Chessboard::square_to_rank_file(35);
    /// println!("File: {}, Rank: {}", file, rank);
    /// // Output: File: 'D', Rank: 5
    /// ```
    pub fn square_to_rank_file(square: u8) -> (char, usize) {
        let row = (square - 1) / 8 + 1;
        let col = (square - 1) % 8;
        let file = (b'A' + col) as char;
        (file, row as usize)
    }

    /// Converts a chess rank and file to its corresponding square index (0-63).
    ///
    /// # Arguments
    ///
    /// * `rank` - The rank of the chessboard (1-8).
    /// * `file` - The file of the chessboard (character 'A' to 'H').
    ///
    /// # Returns
    ///
    /// The result of the square index (0-63) corresponding to the given rank and file.
    ///
    /// # Example
    ///
    /// ```
    /// use kn_o3::board::Chessboard;
    /// let square = Chessboard::rank_file_to_square(5, 'D').unwrap();
    /// println!("Square: {square}");
    /// // Output: Square: 35
    /// ```
    pub fn rank_file_to_square(rank: u8, file: char) -> Result<u64, String> {
        if rank < 1 || rank > 8 {
            return Err("Invalid rank".to_string());
        }
        if file < 'A' || file > 'H' {
            return Err("Invalid file".to_string());
        }
        Ok((rank - 1) as u64 * 8 + (file as u8 - b'A') as u64)
    }

    /// Converts a chess rank and file coordinate to its corresponding square index (0-63).
    /// # Arguments
    /// * `str` - The coordinate string (`A1` - `H8`)
    /// # Returns
    /// The result of the square index (0-63) corresponding to the given rank and file.
    pub fn string_to_square(str: &str) -> Result<u64, String> {
        let chars: Vec<char> = str.chars().collect();
        if chars.len() != 2 { return Err("Invalid coordinate input".to_string()); }

        let rank = chars[1].to_digit(10).unwrap_or(9) as u8;
        let file = chars[0];

        Chessboard::rank_file_to_square(rank, file)
    }

    /// Moves a chess piece on the chessboard from the current position to the new position.
    ///
    /// # Arguments
    ///
    /// * `current_pos` - The current position of the piece in algebraic notation (e.g., "E2").
    /// * `new_pos` - The new position to move the piece to in algebraic notation (e.g., "E4").
    /// * `piece` - The type of chess piece to be moved (e.g., 'p' for pawn, 'R' for rook).
    ///
    /// # Example
    ///
    /// ```
    /// use kn_o3::board::Chessboard;
    /// let mut chessboard = Chessboard::new();
    /// chessboard.move_piece("E2", "E4", 'P');
    /// ```
    pub fn move_piece(&mut self, current_pos: &str, new_pos: &str, piece: char) {
        let old_square = match Chessboard::string_to_square(current_pos) {
            Ok(square) => square,
            _ => return
        };
        let new_square = match Chessboard::string_to_square(new_pos) {
            Ok(square) => square,
            _ => return
        };


        let two: u64 = 2;
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
            _ => {}
        }
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
    /// use kn_o3::board::Chessboard;
    /// let initial_position = Chessboard::new();
    /// let piece_at_a1 = initial_position.piece_at_position(1, 0);
    /// println!("Piece at a1: {}", piece_at_a1);
    /// ```
    /// Note: Uppercase pieces are white and lowercase pieces are black.
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
    /// # Returns
    ///
    /// A FEN string representing the game state.
    ///
    /// # Example
    ///
    /// ```
    /// use kn_o3::board::Chessboard;
    /// let initial_position = Chessboard::new();
    /// let fen_string = initial_position.to_string();
    /// println!("FEN: {}", fen_string);
    /// ```
    ///
    /// Note: The FEN string is used to represent the state of a chess position in a concise
    /// and human-readable format. It captures information about piece placement, castling rights,
    /// en passant targets, and other game state details.
    #[allow(clippy::all)]
    pub fn to_string(&self) -> String {
        let mut string_array: [&str; 6] = ["", "", "", "", "", ""];

        // Piece placement
        let pieces = fen_util::get_fen_placement(&self);
        string_array[0] = &pieces;

        // Whose turn
        string_array[1] = if self.white_turn { "w" } else { "b" };

        // Castling rights
        let castle = fen_util::get_fen_castles(&self);
        string_array[2] = &castle;

        // En passant
        let passant = fen_util::get_fen_passant(&self);
        string_array[3] = &passant;

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
    #[rustfmt::skip]
    fn find_bkgnd(&self, rank: usize, file: usize) -> Color {
        let lght = Color::Rgb { r: 190, g: 140, b: 170 };
        let dark = Color::Rgb { r: 255, g: 206, b: 158 };
        if (rank + file) % 2 == 0 { dark }
        else                      { lght }
    }

    //LEGAL MOVES CODE BELOW

    pub fn legal_move(cb: &Chessboard, piece: char, current_pos: &str, new_pos: &str) -> bool {
        let mut is_legal = false;
        //make sure it's actually moving and not same square and the right color is moving
        if current_pos == new_pos
            || cb.white_turn & piece.is_lowercase()
            || !cb.white_turn & piece.is_uppercase()
        {
            return false;
        }
        // Extract old and new square coordinates
        if let (Some(old_file), Some(old_rank), Some(new_file), Some(new_rank)) = (
            current_pos.chars().next(),
            current_pos.chars().next_back(),
            new_pos.chars().next(),
            new_pos.chars().next_back(),
        ) {
            let old_square =
                Chessboard::rank_file_to_square(old_rank.to_digit(10).unwrap() as u8, old_file);
            let new_square =
                Chessboard::rank_file_to_square(new_rank.to_digit(10).unwrap() as u8, new_file);

            //check if new square is out of bounds (can't be less than 0 because u64)
            if old_square > 63 || new_square > 63 {
                return false;
            }

            if cb.piece_at_position(
                old_rank.to_digit(10).unwrap() as usize,
                (old_file as u8 - b'A') as usize,
            ) != piece
                || (piece.is_lowercase()
                    & cb.piece_at_position(
                        new_rank.to_digit(10).unwrap() as usize,
                        (new_file as u8 - b'A') as usize,
                    )
                    .is_lowercase())
                || (piece.is_uppercase()
                    & cb.piece_at_position(
                        new_rank.to_digit(10).unwrap() as usize,
                        (new_file as u8 - b'A') as usize,
                    )
                    .is_uppercase())
            {
                return false;
            }

            match piece {
                'p' => is_legal = Chessboard::legal_pawn(cb, old_square, new_square),
                'r' => is_legal = Chessboard::legal_rook(old_square, new_square),
                'b' => is_legal = Chessboard::legal_bishop(old_square, new_square),
                'k' => is_legal = Chessboard::legal_king(old_square, new_square),
                'q' => is_legal = Chessboard::legal_queen(old_square, new_square),
                'n' => is_legal = Chessboard::legal_knight(old_square, new_square),
                'P' => is_legal = Chessboard::legal_pawn(cb, old_square, new_square),
                'R' => is_legal = Chessboard::legal_rook(old_square, new_square),
                'B' => is_legal = Chessboard::legal_bishop(old_square, new_square),
                'K' => is_legal = Chessboard::legal_king(old_square, new_square),
                'Q' => is_legal = Chessboard::legal_queen(old_square, new_square),
                'N' => is_legal = Chessboard::legal_knight(old_square, new_square),
                _ => return false,
            }
        }
        is_legal
    }

    fn legal_pawn(cb: &Chessboard, old_square: u64, new_square: u64) -> bool {
        //if there is a piece diagonal

        //if en passant

        // if white turn and first position
        if cb.white_turn && old_square < 15 && old_square > 7 {
            new_square == old_square + 8 || new_square == old_square + 16
        // if black turn and first position
        } else if !cb.white_turn && old_square < 56 && old_square > 47 {
            new_square == old_square - 8 || new_square == old_square - 16
        //if white turn and not first position
        } else if cb.white_turn {
            new_square == old_square + 8
        //if black turn and not first position
        } else {
            new_square == old_square - 8
        }

        //move 1 or 2 if on start square (and nothing in front of it)
        //move 1 otherwise (and nothing in front of it)
        //if opposing piece is diagonal to it
        //if en passant is diagonal to it
        //promote if at end
    }
    fn legal_knight(old_square: u64, new_square: u64) -> bool {
        new_square == (old_square + 17)
            || new_square == (old_square + 15)
            || new_square == (old_square + 10)
            || new_square == (old_square - 10)
            || new_square == (old_square - 6)
            || new_square == (old_square + 6)
            || new_square == (old_square - 17)
            || new_square == (old_square - 15)
    }
    fn legal_bishop(old_square: u64, new_square: u64) -> bool {
        //add and subtract multiples of 7 and 9 (max 8 long diagonal)
        if new_square > old_square {
            (new_square - old_square) % 7 == 0 || (new_square - old_square) % 9 == 0
        } else {
            (old_square - new_square) % 7 == 0 || (old_square - new_square) % 9 == 0
        }
    }
    fn legal_rook(old_square: u64, new_square: u64) -> bool {
        if new_square % 8 == old_square % 8 || new_square / 8 == old_square / 8 {
            return true;
        }
        false
    }
    fn legal_king(old_square: u64, new_square: u64) -> bool {
        //TODO: can't go into a checked square
        //TODO: implement castling
        new_square == old_square + 1 //right
            || new_square == old_square - 1 //left
            || new_square == old_square + 8 //up
            || new_square == old_square - 8 //down
            || new_square == old_square + 9 //diag up right
            || new_square == old_square - 9 //diag down left
            || new_square == old_square - 7 //diag down right
    }
    fn legal_queen(old_square: u64, new_square: u64) -> bool {
        Chessboard::legal_bishop(old_square, new_square)
            && Chessboard::legal_rook(old_square, new_square)
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

/// I've never used this before, but the linter wants it
/// I'm not sure what it does, I'm guessing it's the default constructor
/// --Cooper
impl Default for Chessboard {
    fn default() -> Self {
        Self::new()
    }
}
