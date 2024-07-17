use crate::position::rank_file_to_square;

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
    /// piece placement portion of the FEN string
    pub fn from_string(piece_placement: &str) -> Result<Self, String> {
        let mut result = Chessboard::empty();

        let mut rank_ndx = 8;
        for pieces in piece_placement.split('/') {
            let mut file_ndx = b'A';

            for piece in pieces.chars() {
                if piece.is_ascii_digit() {
                    file_ndx += piece.to_digit(10).expect("Already checked") as u8;
                    continue;
                }


                let square = rank_file_to_square(rank_ndx, file_ndx as char)?;
                *result.piece_bitboard(piece)? |= 1_i64 << square;
                file_ndx += 1;
            }
            rank_ndx -= 1;
        }
        
        Ok(result)
    }

    pub fn to_string(&self) -> String {
        todo!()
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

    pub fn empty() -> Self {
        Self {
            white_rooks: 0,
            white_knights: 0,
            white_bishops: 0,
            white_pawns: 0,
            white_queen: 0,
            white_king: 0,

            black_rooks: 0,
            black_knights: 0,
            black_bishops: 0,
            black_queen: 0,
            black_king: 0,
            black_pawns: 0
        }
    }
}
