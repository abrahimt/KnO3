use crate::fen::FEN;

pub struct Chessboard {
    pub black_pawns: i64,
    pub black_rooks: i64,
    pub black_knights: i64,
    pub black_bishops: i64,
    pub black_queen: i64,
    pub black_king: i64,
    pub white_pawns: i64,
    pub white_rooks: i64,
    pub white_knights: i64,
    pub white_bishops: i64,
    pub white_queen: i64,
    pub white_king: i64,
}

impl Chessboard {
    pub fn from_string(fen: &str) -> Option<Chessboard> {
        todo!()
    }

    pub fn to_string(&self) -> String {
        "".to_string()
    }
    pub fn new() -> Chessboard {
        let pawns = 0xFF;
        let rooks = 0x81;
        let knights = 0x42;
        let bishops = 0x24;
        let queen = 0x08;
        let king = 0x10;

        let top_row = 56; // 7 rows * bit bits

        Chessboard {
            white_rooks: rooks,
            white_knights: knights,
            white_bishops: bishops,
            white_pawns: pawns << 8,
            white_queen: queen,
            white_king: king,

            black_rooks: rooks << top_row,
            black_knights: knights << top_row,
            black_bishops: bishops << top_row,
            black_queen: queen << top_row,
            black_king: king << top_row,
            black_pawns: pawns << (top_row - 8),
        }
    }
}
