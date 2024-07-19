use super::GameState;
use std::cmp::{min, max};
use std::collections::HashSet;

impl GameState {

    /// Move squares in iterator until a piece is hit
    fn move_until_piece<I>(&self, range: I, white: bool) -> Vec<u8>
    where I: Iterator<Item = u8> {
        let mut result = Vec::new();
        let own = self.board.one_side_pieces(white);
        let opps = self.board.one_side_pieces(!white);

        for square in range {
            let btwise = 1 << square;
            if own & btwise != 0 { break; }

            result.push(square);
            if opps & btwise != 0 { break; }
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

    // TODO: en passant check
    fn possible_pawn_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();
        let rank = from / 8;
        let direction = if white { 1 } else { -1 };
        let initial_rank = if white { 1 } else { 6 };

        let out_of_bounds = (direction == 1 && from > 55) || (direction == -1 && from < 8);
        if out_of_bounds { return result; }

        let opps = self.board.one_side_pieces(!white);
        let taken = self.board.both_side_pieces();
        
        let left_diag = from as i32 + 7 * direction;
        let forward = from as i32 + 8 * direction;
        let right_diag = from as i32 + 9 * direction;

        if taken & (1 << forward) == 0 {
            result.push(forward as u8);
            if rank == initial_rank {
                let double = forward + 8 * direction;
                if taken & (1 << double) == 0 { result.push(double as u8); }
            }
        }

        if opps & (1 << left_diag) != 0 {
            result.push(left_diag as u8);
        }
        if opps & (1 << right_diag) != 0 {
            result.push(right_diag as u8);
        }

        result
    }

    fn possible_rook_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();
        let file = from % 8;
        let left_bound = from - file;
        let right_bound = left_bound + 7;

        result.extend(self.move_until_piece((left_bound..from).rev(), white)); // leftward moves
        result.extend(self.move_until_piece(from+1..=right_bound, white)); // rightwards 
        result.extend(self.move_until_piece( // upward
                (from + 8..=63).step_by(8),
                white
        ));
        result.extend(self.move_until_piece( // downward
                (file..from).step_by(8).rev(),
                white
        ));

        result
    }

    fn possible_bishop_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();

        let file = from % 8; // how many rows we can move right
        let nw_bound = min(56, from + file * 7);
        let sw_bound = from.saturating_sub(file * 9);

        let inv_file = 7 - file; // inverse rank (how many rows we can move left)
        let ne_bound = min(63, from + inv_file * 9);
        let se_bound = max(7, from.saturating_sub(inv_file * 7));

        let nw = (from + 7..=nw_bound).step_by(7); // correct
        let sw = (sw_bound..=from.saturating_sub(9)).rev().step_by(9);
        let ne = (from + 9..=ne_bound).step_by(9); // correct
        let se = (se_bound..=from.saturating_sub(7)).rev().step_by(7);

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

    fn possible_knight_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let file = from % 8;
        let own = self.board.one_side_pieces(white);

        let mut moves = HashSet::from([-6, 6, -10, 10, -15, 15, -17, 17]);
        let north = HashSet::from([ 6,  10,  15,  17]);
        let south = HashSet::from([-6, -10, -15, -17]);
        let east  = HashSet::from([-6,  10, -15,  17]);
        let west  = HashSet::from([ 6, -10,  15, -17]);
        let horz: HashSet<i32> = east.union(&west).cloned().collect();
        let vert: HashSet<i32> = north.union(&south).cloned().collect();

        if from < 8       { moves = moves.difference(&south).cloned().collect(); }
        else if from > 55 { moves = moves.difference(&north).cloned().collect(); }
        else if from < 16 {
            moves = moves.difference(
                &south.difference(&horz).cloned().collect()
            ).cloned().collect();
        }
        else if from > 47 {
            moves = moves.difference(
                &north.difference(&horz).cloned().collect()
            ).cloned().collect();
        }

        if file == 0      { moves = moves.difference(&west).cloned().collect(); }
        else if file == 7 { moves = moves.difference(&east).cloned().collect(); }
        else if file == 1 {
            moves = moves.difference(
                &west.difference(&vert).cloned().collect()
            ).cloned().collect();
        }
        else if file == 6 {
            moves = moves.difference(
                &east.difference(&vert).cloned().collect()
            ).cloned().collect();
        }

        let mut result = Vec::new();
        for &mve in &moves {
            let target = from as i32 + mve;
            if target >= 0 && target <= 63 {
                if own & (1 << target) == 0 {
                    result.push(target as u8);
                }
            }
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_moves() {
        let mut gs = GameState::new();
        assert_eq!(gs.possible_pawn_moves(17, true), vec![25]); // normal move
        assert_eq!(gs.possible_pawn_moves(9, true), vec![17, 25]); // beginning move
        assert_eq!(gs.possible_pawn_moves(49, false), vec![41, 33]); // beginning move
        assert_eq!(gs.possible_pawn_moves(42, true), vec![49, 51]); // capture

        // out of bounds
        assert_eq!(gs.possible_pawn_moves(57, true), vec![]);
        assert_eq!(gs.possible_pawn_moves(1, false), vec![]);

        assert_eq!(gs.possible_pawn_moves(1, true), vec![]); // behind another white piece
        gs.board.black_pawns |= 1 << 8; // place a black pawn on 8
        assert_eq!(gs.possible_pawn_moves(1, true), vec![8]); // should be able to take 8 now
    }

    #[test]
    fn test_rook_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_rook_moves(0, true), vec![]); // blocked
        assert_eq!(gs.possible_rook_moves(33, true), vec![32, 34, 35, 36, 37, 38, 39, 41, 49, 25, 17]); // normal move
    }

    #[test]
    fn test_bishop_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_bishop_moves(2, true), vec![]); // blocked
        assert_eq!(gs.possible_bishop_moves(34, true), vec![41, 48, 25, 16, 43, 52, 27, 20]);
    }

    #[test]
    fn test_knight_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_knight_moves(1, true), vec![16, 18]); // white left starting
        assert_eq!(gs.possible_knight_moves(6, true), vec![21, 23]); // white right starting
        assert_eq!(gs.possible_knight_moves(34, true), vec![49, 51, 44, 28, 19, 17, 24, 40]); // normal move
        assert_eq!(gs.possible_knight_moves(62, false), vec![45, 47]); // black right starting
    }

    #[test]
    fn test_king_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_king_moves(4, true), vec![]); // blocked
        // TODO: this will fail when checking is added
        assert_eq!(gs.possible_king_moves(4, false), vec![3, 5, 11, 12, 13]);
        assert_eq!(gs.possible_king_moves(34, true), vec![33, 35, 27, 41, 26, 42, 25, 43]); // normal move
    }

    #[test]
    fn test_move_until_piece() {
        let gs = GameState::new();
        let itr = (18..=58).step_by(8);
        // white moves in straight line to black
        assert_eq!(gs.move_until_piece(itr.clone(), true), vec![18, 26, 34, 42, 50]);
        // white black moves in straight line to black
        assert_eq!(gs.move_until_piece(itr, false), vec![18, 26, 34, 42]);

        assert_eq!(gs.move_until_piece(0..7, true), vec![]);
        assert_eq!(gs.move_until_piece(0..7, false), vec![0]); // can eat this piece
    }

    #[test]
    fn test_possible_moves() {
        let mut gs = GameState::new();
        assert_eq!(gs.possible_moves(9).expect("White pawn"), gs.possible_pawn_moves(9, true)); // pawn begin
        assert_eq!(gs.possible_moves(42), None); // nothing here
        gs.board.white_pawns |= 1 << 42;
        assert_eq!(gs.possible_moves(42).expect("White pawn"), gs.possible_pawn_moves(42, true)); // pawn eat
        assert_eq!(gs.possible_moves(50).expect("Black pawn"), gs.possible_pawn_moves(50, false)); // black pawn blocked

        gs.board.white_pawns = 0;
        gs.board.black_pawns = 0;
        assert_eq!(gs.possible_moves(0).expect("White rook"), gs.possible_rook_moves(0, true));
        assert_eq!(gs.possible_moves(56).expect("Black rook"), gs.possible_rook_moves(0, false));
        assert_eq!(gs.possible_moves(1).expect("White knight"), gs.possible_knight_moves(1, true));
        assert_eq!(gs.possible_moves(57).expect("Black knight"), gs.possible_knight_moves(57, false));
        assert_eq!(gs.possible_moves(2).expect("White bishop"), gs.possible_bishop_moves(2, true));
        assert_eq!(gs.possible_moves(58).expect("Black bishop"), gs.possible_bishop_moves(58, false));
        assert_eq!(gs.possible_moves(3).expect("White king"), gs.possible_king_moves(3, true));
        assert_eq!(gs.possible_moves(59).expect("Black king"), gs.possible_king_moves(59, false));
        assert_eq!(gs.possible_moves(4).expect("White queen"), gs.possible_queen_moves(4, true));
        assert_eq!(gs.possible_moves(60).expect("Black queen"), gs.possible_queen_moves(60, false));
    }
}
