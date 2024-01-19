use super::Chessboard;
use super::position;

impl Chessboard {
    pub fn get_score(&self) -> i32 {
        let mut score = 0;

        // Material balance
        score += 1 * (self.white_pawns.count_ones() as i32 - self.black_pawns.count_ones() as i32);
        score += 3 * (self.white_knights.count_ones() as i32 - self.black_knights.count_ones() as i32);
        score += 3 * (self.white_bishops.count_ones() as i32 - self.black_bishops.count_ones() as i32);
        score += 5 * (self.white_rooks.count_ones() as i32 - self.black_rooks.count_ones() as i32);
        score += 9 * (self.white_queen.count_ones() as i32 - self.black_queen.count_ones() as i32);

        score
    }

    // I would like to clean up the nesting in here --Cooper
    pub fn get_legal_pawn_moves(&self, from: i64, white: bool) -> Vec<i64> {
        let rank = position::square_to_rank(from);
        let direction = if white { 1 } else { -1 };
        let initial_rank = if white { 2 } else { 7 };

        let mut result = Vec::new();
        let left_diag = from + 7 * direction;
        let forward = from + 8 * direction;
        let right_diag = from + 9 * direction;

        if self.piece_at_position(forward).is_none() {
            result.push(forward);
            if rank == initial_rank {
                let double = from + 16 * direction;
                if self.piece_at_position(double).is_none() {
                    result.push(double);
                }
            }
        }

        if let Some(color) = self.white_at_position(left_diag) {
            if color != white {
                result.push(left_diag);
            }
        }

        if let Some(color) = self.white_at_position(right_diag) {
            if color != white {
                result.push(right_diag);
            }
        }


        result.retain(|&square| square >= 0 && square <= 63); // stay within bounds
        result
    }
}
