use crate::board::Chessboard;

pub fn legal_pawn(white: bool, from: u64, to: u64) -> bool {
    let rank = Chessboard::square_to_rank(from);
    let direction = if white { 1 } else { -1 };
    let initial_rank = if white { 2 } else { 7 };
    let diff = to as i64 - from as i64;

    if diff == 8 * direction {
        true
    } else {
        rank == initial_rank && diff == 16 * direction
    }
}

pub fn legal_rook(from: u64, to: u64) -> bool {
    to % 8 == from % 8 || to / 8 == from / 8
}

pub fn legal_bishop(mut from: u64, mut to: u64) -> bool {
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

pub fn legal_king(from: u64, to: u64) -> bool {
    if from < to {
        to == from + 1 || to == from + 9 || to == from + 8 || to == from + 7
    } else {
        to == from - 1 || to == from - 8 || to == from - 9 || to == from - 7
    }
}

pub fn legal_queen(from: u64, to: u64) -> bool {
    legal_bishop(from, to) && legal_rook(from, to)
}

pub fn legal_knight(from: u64, to: u64) -> bool {
    if from < to {
        to == (from + 17) || to == (from + 15) || to == (from + 10) || to == (from + 6)
    } else {
        to == (from - 10) || to == (from - 6) || to == (from - 17) || to == (from - 15)
    }
}

pub fn pawn_vision(white: bool, square: u64) -> u64 {
    //make a bitboard of the three spots the pawn could potentially move to
    //and them with opposing pieces
    //or them with the three squares bitboard
    if white {
        
    } else {

    }
    0
    //check if moves are legal
}

pub fn knight_vision(white: bool, square: u64) -> u64 {
    //make a bitboard of the spots the knight could potentially move to
    //and them with opposing pieces
    //or them with the three squares bitboard

    //check if moves are legal
    0
}

pub fn bishop_vision(white: bool, square: u64) -> u64 {
    0
}

pub fn rook_vision(white: bool, square: u64) -> u64 {
    0
}

pub fn king_vision(white: bool, square: u64) -> u64 {
    0
}

pub fn queen_vision(white: bool, square: u64) -> u64 {
    rook_vision(white, square) & bishop_vision(white, square)
}
