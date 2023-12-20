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

pub fn legal_bishop(from: i64, to: i64) -> bool {
    let from_color = (from / 8 + from % 8) % 2 == 0;
    let to_color = (to / 8 + to % 8) % 2 == 0;

    if from_color == to_color {
        let diff = (to - from).abs();
        diff % 7 == 0 || diff % 9 == 0
    } else {
        false
    }
}

pub fn legal_king(from: i64, to: i64) -> bool {
    let rank_diff = (from / 8 - to / 8).abs();
    let file_diff = (from % 8 - to % 8).abs();
    rank_diff <= 1 && file_diff <= 1
}

pub fn legal_queen(from: i64, to: i64) -> bool {
    legal_bishop(from, to) && legal_rook(from, to)
}

pub fn legal_knight(from: i64, to: i64) -> bool {
    let spaces = (to - from).abs();
    match spaces {
        6 | 10 | 15 | 17 => true,
        _ => false
    }
}
