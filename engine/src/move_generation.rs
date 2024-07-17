use super::GameState;

impl GameState {
    pub fn possible_moves(&self, square: u8) -> Option<Vec<u8>> {
        let piece = match self.board.piece_at_position(square) {
            Some(p) => p,
            None => return None
        };

        let is_white = piece.is_ascii_uppercase();

        let possible_moves = match piece.to_ascii_lowercase() {
            'p' => self.possible_pawn_moves(square, is_white),
            'r' => self.possible_rook_moves(square, is_white),
            'n' => self.possible_knight_moves(square, is_white),
            'b' => self.possible_bishop_moves(square, is_white),
            'k' => self.possible_king_moves(square, is_white),
            'q' => self.possible_queen_moves(square, is_white),
            _ => return None
        };
        Some(possible_moves)
    }

    fn possible_pawn_moves(&self, from: u8, white: bool) -> Vec<u8> {
        todo!()
    }

    fn possible_knight_moves(&self, from: u8, white: bool) -> Vec<u8> {
        todo!()
    }

    fn possible_rook_moves(&self, from: u8, white: bool) -> Vec<u8> {
        todo!()
    }

    fn possible_bishop_moves(&self, from: u8, white: bool) -> Vec<u8> {
        todo!()
    }

    fn possible_queen_moves(&self, from: u8, white: bool) -> Vec<u8> {
        todo!()
    }

    fn possible_king_moves(&self, from: u8, white: bool) -> Vec<u8> {
        todo!()
    }

}
