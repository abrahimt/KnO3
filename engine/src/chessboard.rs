pub struct Chessboard {
    black_pawns: i64,
    black_rooks: i64,
    black_knights: i64,
    black_bishops: i64,
    black_queen: i64,
    black_king: i64,
    white_pawns: i64,
    white_rooks: i64,
    white_knights: i64,
    white_bishops: i64,
    white_queen: i64,
    white_king: i64,

    white_turn: bool,
    caslting_rights: u8,
    en_passant: u8
}

impl Chessboard {
    pub fn new() -> Chessboard {
        let pawns = 0xFF;
        let rooks = 0x81;
        let knights = 0x42;
        let bishops = 0x24;
        let queen = 0x08;
        let king = 0x10;

        let top_row = 56; // 7 rows * bit bits

        Chessboard {
            black_pawns: pawns << (top_row - 8),
            white_pawns: pawns << 8,
            black_rooks: rooks << top_row,
            white_rooks: rooks,
            black_knights: knights << top_row,
            white_knights: knights,
            black_bishops: bishops << top_row,
            white_bishops: bishops,
            black_queen: queen << top_row,
            white_queen: queen,
            black_king: king << top_row,
            white_king: king,

            castling_rights: 0x0F,
            en_passant: 0,
            white_turn: true
        }
    }
}
