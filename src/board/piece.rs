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
    false
}

pub fn legal_bishop(from: u64, to: u64) -> bool {
    false
}

pub fn legal_king(from: u64, to: u64) -> bool {
    false
}

pub fn legal_queen(from: u64, to: u64) -> bool {
    false
}

pub fn legal_knight(from: u64, to: u64) -> bool {
    false
}

/*

fn legal_pawn(cb: &Chessboard, old_square: u64, new_square: u64) -> bool {
    //if there is a piece diagonal

    //if en passant

    // if white turn and first position
    if cb.white_turn && old_square < 15 && old_square > 7 {
        new_square == old_square + 8 || new_square == old_square + 16
    // if black turn and first position
    } else if !cb.white_turn && old_square < 56 && old_square > 47 {
        new_square == old_square - 8 || new_square == old_square - 16
    //if white turn and not first position
    } else if cb.white_turn {
        new_square == old_square + 8
    //if black turn and not first position
    } else {
        new_square == old_square - 8
    }

    //move 1 or 2 if on start square (and nothing in front of it)
    //move 1 otherwise (and nothing in front of it)
    //if opposing piece is diagonal to it
    //if en passant is diagonal to it
    //promote if at end
}
fn legal_knight(old_square: u64, new_square: u64) -> bool {
    new_square == (old_square + 17)
        || new_square == (old_square + 15)
        || new_square == (old_square + 10)
        || new_square == (old_square - 10)
        || new_square == (old_square - 6)
        || new_square == (old_square + 6)
        || new_square == (old_square - 17)
        || new_square == (old_square - 15)
}
fn legal_bishop(old_square: u64, new_square: u64) -> bool {
    //add and subtract multiples of 7 and 9 (max 8 long diagonal)
    if new_square > old_square {
        (new_square - old_square) % 7 == 0 || (new_square - old_square) % 9 == 0
    } else {
        (old_square - new_square) % 7 == 0 || (old_square - new_square) % 9 == 0
    }
}
fn legal_rook(old_square: u64, new_square: u64) -> bool {
    if new_square % 8 == old_square % 8 || new_square / 8 == old_square / 8 {
        return true;
    }
    false
}
fn legal_king(old_square: u64, new_square: u64) -> bool {
    //TODO: can't go into a checked square
    //TODO: implement castling
    new_square == old_square + 1 //right
        || new_square == old_square - 1 //left
        || new_square == old_square + 8 //up
        || new_square == old_square - 8 //down
        || new_square == old_square + 9 //diag up right
        || new_square == old_square - 9 //diag down left
        || new_square == old_square - 7 //diag down right
}
fn legal_queen(old_square: u64, new_square: u64) -> bool {
    Chessboard::legal_bishop(old_square, new_square)
        && Chessboard::legal_rook(old_square, new_square)
}
*/

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
}
