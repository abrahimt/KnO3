use crate::board::Chessboard;

// I would love to cutdown the nesting here --Cooper
pub fn legal_pawn(white: bool, from: u64, to: u64) -> bool {
    let rank = Chessboard::square_to_rank(from);

    if white {
        if rank == 2 {
            from + 16 == to || from + 8 == to
        } else {
            from + 8 == to
        }
    } else if rank == 7 {
        from - 16 == to || from - 8 == to
    } else {
        if rank == 7 {
            from - 16 == to || from - 8 == to
        } else {
            if from < 8 { return false; } // already on the back row, has been promoted
            from - 8 == to
        }
    }
}

pub fn legal_rook(from: u64, to: u64) -> bool {
    to % 8 == from % 8 || to / 8 == from / 8
}

pub fn legal_bishop(from: u64, to: u64) -> bool {
    if to > from && (from % 2 == 0 && to % 2 == 0 || from % 2 != 0 && to % 2 != 0) {
        (to - from) % 7 == 0 || (to - from) % 9 == 0
    } else if to < from && (from % 2 == 0 && to % 2 == 0 || from % 2 != 0 && to % 2 != 0) {
        (from - to) % 7 == 0 || (from - to) % 9 == 0
    } else {
        false
    }
}

pub fn legal_king(from: u64, to: u64) -> bool {
    to == from + 1 //right
    || to == from - 1 //left
    || to == from + 8 //up
    || to == from - 8 //down
    || to == from + 9 //diag up right
    || to == from - 9 //diag down left
    || to == from - 7 //diag down right
}

pub fn legal_queen(from: u64, to: u64) -> bool {
    legal_bishop(from, to) && legal_rook(from, to)
}

pub fn legal_knight(from: u64, to: u64) -> bool {
    to == (from + 17)
        || to == (from + 15)
        || to == (from + 10)
        || to == (from - 10)
        || to == (from - 6)
        || to == (from + 6)
        || to == (from - 17)
        || to == (from - 15)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_pawn() {
        assert!(legal_pawn(true, 0, 8)); // One square forward
        assert!(legal_pawn(false, 8, 0));

        assert!(!legal_pawn(true, 0, 16)); // Two squares forward
        assert!(!legal_pawn(false, 16, 0));
        assert!(legal_pawn(true, 8, 24));
        assert!(legal_pawn(true, 15, 31));
        assert!(legal_pawn(false, 48, 32));
        assert!(legal_pawn(false, 55, 39));

        assert!(!legal_pawn(true, 8, 0)); // One square backward
        assert!(!legal_pawn(false, 0, 8));
        assert!(!legal_pawn(false, 8, 16));

        assert!(!legal_pawn(true, 1, 8)); // Adjacent
        assert!(!legal_pawn(true, 1, 0));
        assert!(!legal_pawn(true, 1, 10));
        assert!(!legal_pawn(true, 1, 2));
        assert!(!legal_pawn(false, 57, 56));
        assert!(!legal_pawn(false, 57, 48));
        assert!(!legal_pawn(false, 57, 58));
        assert!(!legal_pawn(false, 57, 50));

        assert!(!legal_pawn(true, 1, 1)); // Same spot
        assert!(!legal_pawn(false, 1, 1));
    }


    #[test]
    fn test_legal_rook() {
        // Same file
        assert!(legal_rook(0, 8));
        assert!(legal_rook(0, 16));
        assert!(legal_rook(0, 56));
        assert!(legal_rook(56, 0));
        assert!(legal_rook(8, 15));

        // Same rank
        assert!(legal_rook(0, 1));
        assert!(legal_rook(0, 2));
        assert!(legal_rook(0, 7));
        assert!(legal_rook(7, 0));
        assert!(legal_rook(8, 48));

        // Different file && rank
        assert!(!legal_rook(0, 9));
        assert!(!legal_rook(0, 17));
        assert!(!legal_rook(0, 39));
        assert!(!legal_rook(0, 57));
        assert!(!legal_rook(0, 63));
        assert!(!legal_rook(9, 0));
        assert!(!legal_rook(17, 0));
        assert!(!legal_rook(39, 0));
        assert!(!legal_rook(57, 0));
        assert!(!legal_rook(63, 0));

        // Same square
        assert!(!legal_rook(0,0));
        assert!(!legal_rook(57, 57));
    }
    }
}
