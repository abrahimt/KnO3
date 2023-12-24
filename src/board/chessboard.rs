use crate::fen_util;

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
}

/// I've never used this before, but the linter wants it
/// I'm not sure what it does, I'm guessing it's the default constructor
/// --Cooper
impl Default for Chessboard {
    fn default() -> Self {
        Self::new()
    }
}
