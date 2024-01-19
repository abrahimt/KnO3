use super::Chessboard;

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
}
