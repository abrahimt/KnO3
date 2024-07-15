use super::position;
use super::Chessboard;

impl Chessboard {
    pub fn get_score(&self) -> i32 {
        let mut score = 0;

        // Material balance
        score += self.white_pawns.count_ones() as i32 - self.black_pawns.count_ones() as i32;
        score +=
            3 * (self.white_knights.count_ones() as i32 - self.black_knights.count_ones() as i32);
        score +=
            3 * (self.white_bishops.count_ones() as i32 - self.black_bishops.count_ones() as i32);
        score += 5 * (self.white_rooks.count_ones() as i32 - self.black_rooks.count_ones() as i32);
        score += 9 * (self.white_queen.count_ones() as i32 - self.black_queen.count_ones() as i32);

        // TODO: Take into account amount of legal moves per piece
        score
    }

    /// This function does not validate that there is a pawn at this position
    pub fn get_pawn_moves(&self, from: i64, white: bool) -> Vec<i64> {
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

        let opponent_pieces = self.one_side_pieces(!white);
        if opponent_pieces & (1 << left_diag) != 0 {
            result.push(left_diag);
        }
        if opponent_pieces & (1 << right_diag) != 0 {
            result.push(right_diag);
        }

        result.retain(|&square| (0..=63).contains(&square)); // stay within bounds
        result
    }

    pub fn get_rook_moves(&self, from: i64, white: bool) -> Vec<i64> {
        let mut result = Vec::new();
        let (rank, file) = position::square_to_rank_file(from);

        // right moves
        let test = ((file as u8) + 1)..(b'H' + 1);
        for f in ((file as u8) + 1)..(b'H' + 1) {
            let square = position::rank_file_to_square(rank as u8, (b'A' + f as u8) as char).unwrap();
            if let Some(_) = self.piece_at_position(square) {
                break;
            }
            result.push(square);
        }

        // left moves
        for f in ((b'A')..file as u8).rev() {
            let square = position::rank_file_to_square(rank as u8, f as char).unwrap();
            if let Some(_) = self.piece_at_position(square) {
                break;
            }
            result.push(square);
        }

        // up moves
        for r in (rank + 1)..9 {
            let square = position::rank_file_to_square(r as u8, file).unwrap();
            if let Some(_) = self.piece_at_position(square) { break; }
            result.push(square);
        }

        // down moves
        for r in (1..rank).rev() {
            let square = position::rank_file_to_square(r as u8, file).unwrap();
            if let Some(_) = self.piece_at_position(square) { break; }
            result.push(square);
        }

        result
    }

    pub fn get_bishop_moves(&self, from: i64, white: bool) -> Vec<i64> {
        let mut result = Vec::new();
        let (rank, file) = position::square_to_rank_file(from);

        // ne
        for (r, f) in (rank + 1..9).zip((file as u8 + 1)..(b'H' + 1)) {
            let square = position::rank_file_to_square(r as u8, f as char).unwrap();
            if let Some(_) = self.piece_at_position(square) { break; }
            result.push(square);
        }

        // nw
        for (r, f) in (rank + 1..9).zip((b'A'..file as u8).rev()) {
            let square = position::rank_file_to_square(r as u8, f as char).unwrap();
            if let Some(_) = self.piece_at_position(square) { break; }
            result.push(square);
        }

        // se
        for (r, f) in (1..rank).rev().zip((file as u8 + 1)..(b'H' + 1)) {
            let square = position::rank_file_to_square(r as u8, f as char).unwrap();
            if let Some(_) = self.piece_at_position(square) { break; }
            result.push(square);
        }

        // sw
        for (r, f) in (1..rank).rev().zip((b'A'..file as u8).rev()) {
            let square = position::rank_file_to_square(r as u8, f as char).unwrap();
            if let Some(_) = self.piece_at_position(square) { break; }
            result.push(square);
        }

        result
    }

    pub fn get_queen_moves(&self, from: i64, white: bool) -> Vec<i64> {
        let mut result = self.get_rook_moves(from, white);
        result.extend(self.get_bishop_moves(from, white));
        result
    }

    pub fn get_king_moves(&self, from: i64, white: bool) -> Vec<i64> {
        let directions = [-1, 1, -7, 7, -8, 8, -9, 9];
        let mut result = Vec::new();

        let own = self.one_side_pieces(white);

        for &direction in &directions {
            let target = from + direction;
            if target >= 0 && target <= 63 {
                if own & (1 << target) == 0 {
                    result.push(target);
                }
            }
        }

        result
    }

    pub fn get_knight_moves(&self, from: i64, white: bool) -> Vec<i64> {
        Vec::new()
    }
}
