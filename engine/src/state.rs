use super::Chessboard;
impl Chessboard {
    pub fn pieces(&self) -> [(char, i64); 12] {
        [
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

    pub fn piece(&mut self, piece: char) -> Result<&mut i64, String> {
        match piece {
            'p' => Ok(&mut self.black_pawns),
            'r' => Ok(&mut self.black_rooks),
            'n' => Ok(&mut self.black_knights),
            'b' => Ok(&mut self.black_bishops),
            'k' => Ok(&mut self.black_king),
            'q' => Ok(&mut self.black_queen),
            'P' => Ok(&mut self.white_pawns),
            'R' => Ok(&mut self.white_rooks),
            'N' => Ok(&mut self.white_knights),
            'B' => Ok(&mut self.white_bishops),
            'K' => Ok(&mut self.white_king),
            'Q' => Ok(&mut self.white_queen),
            _ => Err(format!("Invalid piece type: {piece}"))
        }
    }

    pub fn piece_at_position(&self, square: i64) -> Option<char> {
        let btwise = 1 << square;
        for (p_type, positions) in self.pieces() {
            if btwise & positions != 0 { return Some(p_type); }
        }
        None
    }

    pub fn one_side_pieces(&self, white: bool) -> i64 {
        if white {
            self.white_bishops
                & self.white_king
                & self.white_knights
                & self.white_pawns
                & self.white_rooks
                & self.white_queen
        } else {
            self.black_bishops
                & self.black_king
                & self.black_knights
                & self.black_pawns
                & self.black_rooks
                & self.black_queen
        }
    }

    pub fn both_side_pieces(&self) -> i64 {
        self.one_side_pieces(true) | self.one_side_pieces(false)
    }
}
