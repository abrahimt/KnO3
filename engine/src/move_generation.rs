use super::GameState;

impl GameState {

    /// Move squares in iterator until a piece is hit
    fn move_until_piece(&self, range: impl Iterator<Item = u8>, white: bool) -> Vec<u8> {
        let mut result = Vec::new();

        for square in range {
            let piece = match self.board.piece_at_position(square) {
                Some(p) => p,
                None => { result.push(square); continue; }
            };

            // Am I dumb or is there no way to simplify this logic?
            let same_color = piece.is_ascii_uppercase() && white || piece.is_ascii_lowercase() && !white;
            if !same_color {
                result.push(square);
            }
            break;
        }

        result
    }

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
        let mut result = Vec::new();
        let left_bound = from - from % 8;
        let right_bound = left_bound + 7;

        result.extend(self.move_until_piece((left_bound..from).rev(), white)); // leftward moves
        result.extend(self.move_until_piece(from+1..=right_bound, white)); // rightwards 
        result.extend(self.move_until_piece(
                (from + 8..=63).step_by(8),
                white
        ));
        result.extend(self.move_until_piece(
                (0..from).step_by(8).rev(),
                white
        ))

        result
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
