use crate::board::Chessboard;

pub fn legal_pawn(white: bool, from: i64, to: i64) -> bool {
    let rank = Chessboard::square_to_rank(from);
    let direction = if white { 1 } else { -1 };
    let initial_rank = if white { 2 } else { 7 };
    let diff = to - from;

    if diff == 8 * direction {
        true
    } else {
        rank == initial_rank && diff == 16 * direction
    }
}

pub fn legal_rook(from: i64, to: i64) -> bool {
    to % 8 == from % 8 || to / 8 == from / 8
}

pub fn legal_bishop(mut from: i64, mut to: i64) -> bool {
    from += 1;
    to += 1;
    let same_color = ((from / 8) + from % 8) % 2 == 0 && ((to / 8) + to % 8) == 0
        || ((from / 8) + from % 8) != 0 && ((to / 8) + to % 8) != 0;

    if to > from && same_color {
        (to - from) % 7 == 0 || (to - from) % 9 == 0
    } else if to < from && same_color {
        (from - to) % 7 == 0 || (from - to) % 9 == 0
    } else {
        false
    }
}

pub fn legal_king(from: i64, to: i64) -> bool {
    if from < to {
        to == from + 1 || to == from + 9 || to == from + 8 || to == from + 7
    } else {
        to == from - 1 || to == from - 8 || to == from - 9 || to == from - 7
    }
}

pub fn legal_queen(from: i64, to: i64) -> bool {
    legal_bishop(from, to) && legal_rook(from, to)
}

pub fn legal_knight(from: i64, to: i64) -> bool {
    if from < to {
        to == (from + 17) || to == (from + 15) || to == (from + 10) || to == (from + 6)
    } else {
        to == (from - 10) || to == (from - 6) || to == (from - 17) || to == (from - 15)
    }
}
