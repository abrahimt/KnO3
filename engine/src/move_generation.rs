use super::GameState;
use std::cmp::{max, min};

// TODO: Castling
impl GameState {
    /// Move squares in iterator until a piece is hit
    fn move_until_piece<I>(&self, range: I, white: bool) -> u64
    where
        I: Iterator<Item = u8>,
    {
        let mut result = 0;
        let own = self.board.one_side_pieces(white);
        let opps = self.board.one_side_pieces(!white);

        for square in range {
            let btwise = 1 << square;
            if own & btwise != 0 {
                break;
            }

            result |= btwise;
            if opps & btwise != 0 {
                break;
            }
        }

        result
    }

    pub fn possible_moves(&self, square: u8) -> u64 {
        let piece = match self.board.piece_at_position(square) {
            Some(p) => p,
            None => return 0,
        };

        let is_white = piece.is_ascii_uppercase();
        match piece.to_ascii_lowercase() {
            'p' => self.possible_pawn_moves(square, is_white),
            'r' => self.possible_rook_moves(square, is_white),
            'n' => self.possible_knight_moves(square, is_white),
            'b' => self.possible_bishop_moves(square, is_white),
            'k' => self.possible_king_moves(square, is_white),
            'q' => self.possible_queen_moves(square, is_white),
            _ => 0,
        }
    }

    fn possible_pawn_moves(&self, from: u8, white: bool) -> u64 {
        let mut result = 0;
        let rank = from / 8;
        let direction = if white { 1 } else { -1 };
        let initial_rank = if white { rank == 1 } else { rank == 6 };

        let out_of_bounds = (direction == 1 && from > 55) || (direction == -1 && from < 8);
        if out_of_bounds {
            return result;
        }

        let opps = self.board.one_side_pieces(!white);
        let taken = self.board.both_side_pieces();

        let left_diag = (from as i32 + 7 * direction) as u8;
        let forward = from as i32 + 8 * direction;
        let right_diag = (from as i32 + 9 * direction) as u8;

        if taken & (1 << forward) == 0 {
            result |= 1 << forward;
            if initial_rank {
                let double = forward + 8 * direction;
                if taken & (1 << double) == 0 {
                    result |= 1 << double;
                }
            }
        }

        let opp_left = opps & (1 << left_diag) != 0;
        let en_passant_left = !initial_rank && left_diag == self.en_passant;
        if opp_left || en_passant_left {
            result |= 1 << left_diag;
        }

        let opp_right = opps & (1 << right_diag) != 0;
        let en_passant_right = !initial_rank && right_diag == self.en_passant;
        if opp_right || en_passant_right {
            result |= 1 << right_diag;
        }

        result
    }

    fn possible_rook_moves(&self, from: u8, white: bool) -> u64 {
        let mut result = 0;
        let file = from % 8;
        let left_bound = from - file;
        let right_bound = left_bound + 7;

        result |= self.move_until_piece((left_bound..from).rev(), white); // west moves
        result |= self.move_until_piece(from + 1..=right_bound, white); // east moves
        result |= self.move_until_piece(
            (from + 8..=63).step_by(8),
            white
        ); // north moves
        result |= self.move_until_piece(
            (file..from).step_by(8).rev(),
            white
        ); // south moves

        result
    }

    fn rook_attack_map(&self, pos: u8, white: bool) -> u64 {
        let rank = pos / 8;
        let mut result = 0;
        let possible_attacks = self.possible_rook_moves(pos, white) & self.board.one_side_pieces(!white);

        // Find the furthest move in each direction (north, east, south, west)
        let north = !((1 << (pos + 1)) - 1); // everything above `pos` is 1
        let south = (1 << pos) - 1; // everything below `pos` is 1
        let horz = 0xFF << (rank * 8); // everything on the same rank as `pos`
        let east = horz & north;
        let west = horz & south;

        let north_most = (possible_attacks & north).leading_zeros();
        if north_most < 64 { result |= 1 << (63 - north_most); }

        let south_most = (possible_attacks & south).trailing_zeros();
        if south_most < 64 { result |= 1 << south_most; }

        let east_most = (possible_attacks & east).leading_zeros();
        if east_most < 64 { result |= 1 << (63 - east_most); }

        let west_most = (possible_attacks & west).trailing_zeros();
        if west_most < 64 { result |= 1 << west_most; }

        result
    }

    fn possible_bishop_moves(&self, from: u8, white: bool) -> u64 {
        let mut result = 0;

        let file = from % 8; // how many rows we can move right
        let nw_bound = min(56, from + file * 7);
        let sw_bound = from.saturating_sub(file * 9);

        let inv_file = 7 - file; // inverse rank (how many rows we can move left)
        let ne_bound = min(63, from + inv_file * 9);
        let se_bound = max(0, from.saturating_sub(inv_file * 7));

        let nw = (from + 7..=nw_bound).step_by(7);
        let sw = (sw_bound..=from.saturating_sub(9)).rev().step_by(9);
        let ne = (from + 9..=ne_bound).step_by(9);
        let se = (se_bound..=from.saturating_sub(7)).rev().step_by(7);

        result |= self.move_until_piece(nw, white);
        result |= self.move_until_piece(sw, white);
        result |= self.move_until_piece(ne, white);
        result |= self.move_until_piece(se, white);

        result
    }

    fn bishop_attack_map(&self, pos: u8, white: bool) -> u64 {
        let mut result = 0;
        let file = pos % 8;
        let possible_attacks = self.possible_bishop_moves(pos, white) & self.board.one_side_pieces(!white);

        let west_of_file = (1 << file) - 1_u64;
        let east_of_file = 0xFF & !((1 << (file + 1)) - 1_u64);

        let west = possible_attacks &
            (west_of_file |
            west_of_file << 8 |
            west_of_file << (8 * 2) |
            west_of_file << (8 * 3) |
            west_of_file << (8 * 4) |
            west_of_file << (8 * 5) |
            west_of_file << (8 * 6) |
            west_of_file << (8 * 7));

        let east = possible_attacks &
            (east_of_file |
            east_of_file << 8 |
            east_of_file << (8 * 2) |
            east_of_file << (8 * 3) |
            east_of_file << (8 * 4) |
            east_of_file << (8 * 5) |
            east_of_file << (8 * 6) |
            east_of_file << (8 * 7));

        let ne_most = east.leading_zeros();
        if ne_most < 64 { result |= 1 << (63 - ne_most); }

        let nw_most = west.leading_zeros();
        if nw_most < 64 { result |= 1 << (63 - nw_most); }

        let se_most = east.trailing_zeros();
        if se_most < 64 { result |= 1 << se_most; }

        let sw_most = west.trailing_zeros();
        if sw_most < 64 { result |= 1 << sw_most; }

        result
    }

    fn possible_queen_moves(&self, from: u8, white: bool) -> u64 {
        self.possible_rook_moves(from, white) | self.possible_bishop_moves(from, white)
    }

    // TODO: Make sure they are not moving into check/mate
    fn possible_king_moves(&self, from: u8, white: bool) -> u64 {
        let mut result = 0;
        let directions: [i8; 8] = [-1, 1, -7, 7, -8, 8, -9, 9];
        let own = self.board.one_side_pieces(white);

        for &direction in &directions {
            let target = from as i8 + direction;
            if (0..=63).contains(&target) && own & (1 << target) == 0 {
                result |= 1 << target;
            }
        }

        result
    }

    fn possible_knight_moves(&self, from: u8, white: bool) -> u64 {
        let file = from % 8;
        let rank = from / 8;
        let own = self.board.one_side_pieces(white);
        let mut result = 0;

        let moves: [(i8, i8); 8] = [
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (-2, -1),
            (-2, 1),
            (2, -1),
            (2, 1),
        ];

        for (d_rank, d_file) in moves.iter() {
            let n_rank = d_rank + rank as i8;
            let n_file = d_file + file as i8;
            if !(0..8).contains(&n_rank) || !(0..8).contains(&n_file) {
                continue;
            }

            let target = (n_rank * 8 + n_file) as u8;
            if own & (1 << target) == 0 {
                result |= 1 << target;
            }
        }

        result
    }

    fn knight_attack_map(&self, pos: u8, white: bool) -> u64 {
        self.possible_knight_moves(pos, white) & self.board.one_side_pieces(!white)
    }

    /// Can this square be taken by the opponent next turn?
    fn position_under_attack(&self, square: u8, white: bool) -> bool {
        let opp_rooks = if white { self.board.black_rooks | self.board.black_queen } else { self.board.white_rooks | self.board.white_queen };

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Chessboard;

    use super::*;

    #[test]
    fn test_pawn_moves() {
        let mut gs = GameState::new();
        assert_eq!(gs.possible_pawn_moves(17, true), 1 << 25, "Failed normal move");
        assert_eq!(gs.possible_pawn_moves(9, true), 1 << 17 | 1 << 25, "Failed beginning move");
        assert_eq!(gs.possible_pawn_moves(49, false), 1 << 41 | 1 << 33, "Failed black start move");
        assert_eq!(gs.possible_pawn_moves(42, true), 1 << 49 | 1 << 51, "Failed white->black capture");

        // out of bounds
        assert_eq!(gs.possible_pawn_moves(57, true), 0, "Failed white oob");
        assert_eq!(gs.possible_pawn_moves(1, false), 0, "Failed black oob");

        gs.en_passant = 16;
        assert_eq!(gs.possible_pawn_moves(9, true), 1 << 17 | 1 << 25, "En-passanted own piece");
        gs.en_passant = 24;
        assert_eq!(gs.possible_pawn_moves(17, true), 1 << 24 | 1 << 25, "Did not en passant");

        assert_eq!(gs.possible_pawn_moves(1, true), 0, "Moved behind own piece");
        gs.board.black_pawns |= 1 << 8; // place a black pawn on 8
        assert_eq!(gs.possible_pawn_moves(1, true), 1 << 8, "Failed capture");
    }

    #[test]
    fn test_rook_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_rook_moves(0, true), 0, "Captured own");
        assert_eq!(
            gs.possible_rook_moves(33, true),
            1 << 32 | 1 << 34 | 1 << 35 | 1 << 36 | 1 << 37 | 1 << 38 | 1 << 39 | 1 << 41 | 1 << 49 | 1 << 25 | 1 << 17,
            "Failed normal move"
        );

    }

    #[test]
    fn test_rook_attacks() {
        let mut gs = GameState::new();
        assert_eq!(gs.rook_attack_map(8, true), 1 << 48, "Did not attack north");
        assert_eq!(gs.rook_attack_map(49, false), 1 << 9, "Did not attack south");

        gs.board = Chessboard::empty();
        assert_eq!(gs.rook_attack_map(0, true), 0, "Attacking nothing");

        gs.board.black_pawns |= 1 << 1;
        assert_eq!(gs.rook_attack_map(0, true), 1 << 1, "Did not attack east");
        assert_eq!(gs.rook_attack_map(7, true), 1 << 1, "Did not attack west");

        // surround a rook with pieces
        let pawns = 1 << 1 | 1 << 8 | 1 << 17 | 1 << 10;
        gs.board.black_pawns = pawns;
        assert_eq!(gs.rook_attack_map(9, true), pawns, "Did not attack all directions");

    }

    #[test]
    fn test_bishop_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_bishop_moves(2, true), 0, "Captured own");
        assert_eq!(
            gs.possible_bishop_moves(34, true),
            1 << 41 | 1 << 48 | 1 << 25 | 1 << 16 | 1 << 43 | 1 << 52 | 1 << 27 | 1 << 20,
            "Failed normal move"
        );
    }

    #[test]
    fn test_bishop_attacks() {
        let mut gs = GameState::new();
        gs.board = Chessboard::empty();

        assert_eq!(gs.bishop_attack_map(0, true), 0, "Attacking nothing");

        gs.board.black_pawns |= 1 << 18;
        assert_eq!(gs.bishop_attack_map(0, true), 1 << 18);
        gs.board.black_pawns |= 1 << 9;
        assert_eq!(gs.bishop_attack_map(0, true), 1 << 9, "Attacked through piece");

        let pawns = 1 << 16 | 1 << 18 | 1 | 1 << 2;
        gs.board.black_pawns = pawns;
        assert_eq!(gs.bishop_attack_map(9, true), pawns, "Did not attack all directions");

        gs.board.black_pawns = 0;
        gs.board.white_pawns = pawns;
        assert_eq!(gs.bishop_attack_map(9, false), pawns, "Black did not attack all directions");
    }

    #[test]
    fn test_knight_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_knight_moves(1, true), 1 << 16 | 1 << 18); // white left starting
        assert_eq!(gs.possible_knight_moves(1, false), 1 << 11 | 1 << 16 | 1 << 18); // black taking
        assert_eq!(gs.possible_knight_moves(6, true), 1 << 21 | 1 << 23); // white right starting
        assert_eq!(
            gs.possible_knight_moves(34, true),
            1 << 24 | 1 << 28 | 1 << 40 | 1 << 44 | 1 << 17 | 1 << 19 | 1 << 49 | 1 << 51,
            "Failed normal move"
        );
        assert_eq!(gs.possible_knight_moves(62, false), 1 << 45 | 1 << 47); // black right starting
    }

    #[test]
    fn test_king_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_king_moves(4, true), 0, "Captured own");
                                                             // TODO: this will fail when checking is added
        assert_eq!(gs.possible_king_moves(4, false), 1 << 3 | 1 << 5 | 1 << 11 | 1 << 12 | 1 << 13);
        assert_eq!(
            gs.possible_king_moves(34, true),
            1 << 33 | 1 << 35 | 1 << 27 | 1 << 41 | 1 << 26 | 1 << 42 | 1 << 25 | 1 << 43,
            "Failed normal move"
        );
    }

    #[test]
    fn test_move_until_piece() {
        let gs = GameState::new();
        let itr = (18..=58).step_by(8);
        assert_eq!(
            gs.move_until_piece(itr.clone(), true),
            1 << 18 | 1 << 26 | 1 << 34 | 1 << 42 | 1 << 50,
            "White should move in straight line to black"
        );
        assert_eq!(gs.move_until_piece(itr, false), 1 << 18 | 1 << 26 | 1 << 34 | 1 << 42, "Black should move to next black");

        assert_eq!(gs.move_until_piece(0..7, true), 0);
        assert_eq!(gs.move_until_piece(0..7, false), 1); // can eat this piece
    }

    #[test]
    fn test_possible_moves() {
        let mut gs = GameState::new();
        assert_eq!(gs.possible_moves(9), gs.possible_pawn_moves(9, true)); // pawn begin
        assert_eq!(gs.possible_moves(42), 0); // nothing here
        gs.board.white_pawns |= 1 << 42;
        assert_eq!(gs.possible_moves(42), gs.possible_pawn_moves(42, true)); // pawn eat
        assert_eq!(gs.possible_moves(50), gs.possible_pawn_moves(50, false)); // black pawn blocked

        gs.board.white_pawns = 0;
        gs.board.black_pawns = 0;
        assert_eq!(gs.possible_moves(0), gs.possible_rook_moves(0, true));
        assert_eq!(gs.possible_moves(56), gs.possible_rook_moves(56, false));
        assert_eq!(gs.possible_moves(1), gs.possible_knight_moves(1, true));
        assert_eq!(gs.possible_moves(57), gs.possible_knight_moves(57, false));
        assert_eq!(gs.possible_moves(2), gs.possible_bishop_moves(2, true));
        assert_eq!(gs.possible_moves(58), gs.possible_bishop_moves(58, false));
        assert_eq!(gs.possible_moves(3), gs.possible_queen_moves(3, true));
        assert_eq!(gs.possible_moves(59), gs.possible_queen_moves(59, false));
        assert_eq!(gs.possible_moves(4), gs.possible_king_moves(4, true));
        assert_eq!(gs.possible_moves(60), gs.possible_king_moves(60, false));
    }
}
