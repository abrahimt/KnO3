use super::GameState;
use std::cmp::{min, max};

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
        result.extend(self.move_until_piece( // upward
                (from + 8..=63).step_by(8),
                white
        ));
        result.extend(self.move_until_piece( // downward
                (0..from).step_by(8).rev(),
                white
        ));

        result
    }

    fn possible_bishop_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();

        let rank = from % 8; // how many rows we can move right
        let nw_bound = min(56, from + rank * 7);
        let sw_bound = max(0, from - rank * 9);

        let inv_rank = 7 - rank; // inverse rank (how many rows we can move left)
        let ne_bound = min(63, from + inv_rank * 9);
        let se_bound = max(7, from - inv_rank * 7);

        let nw = (from + 7..=nw_bound).step_by(7);
        let sw = (sw_bound..from).rev().step_by(9);
        let ne = (from + 9..=ne_bound).step_by(9);
        let se = (se_bound..from).rev().step_by(7);

        result.extend(self.move_until_piece(nw, white));
        result.extend(self.move_until_piece(sw, white));
        result.extend(self.move_until_piece(ne, white));
        result.extend(self.move_until_piece(se, white));

        result
    }

    fn possible_queen_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = self.possible_rook_moves(from, white);
        result.extend(self.possible_bishop_moves(from, white));
        result
    }

    // TODO: Make sure they are not moving into check/mate
    fn possible_king_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();
        let directions: [i8; 8] = [-1, 1, -7, 7, -8, 8, -9, 9];
        let own = self.board.one_side_pieces(white);

        for &direction in &directions {
            let target = from as i8 + direction;
            if target >= 0 && target <= 63 {
                if own & (1 << target) == 0 {
                    result.push(target as u8);
                }
            }
        }

        result
    }

}
