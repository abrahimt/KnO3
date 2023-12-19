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
