use crate::position;

// https://www.chess.com/terms/fen-chess
pub struct FEN {
    piece_placement: String,
    white_turn: bool,
    castling: u8, // KQkq will be represented by 4 bits
    en_passant: i64, // a square that has en passant ability
    half_clock: u32,
    move_count: u32,
}

impl FEN {
    // pub fn new() -> Self {
    //     Self {
    //         piece_placement: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
    //         white_turn: true,
    //         castling: 0x0F,
    //         en_passant: 0,
    //     }
    // }

    pub fn from_string(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 { return Err("Invalid FEN string".to_string()); }

        Ok(Self {
            piece_placement: parts[0].to_string(),
            white_turn: parts[1] == "w",
            castling: parse_castling_rights(parts[2]),
            en_passant: position::string_to_square(parts[3]),
            half_clock: parts[4].parse().unwrap(),
            move_count: parts[5].parse().unwrap()
        })
    }
}

/// rights: The portion of the fen string that marks castling
fn parse_castling_rights(part: &str) -> u8 {
    let mut result = 0;
    for c in part.chars() {
        let v = match c {
            'K' => 0b1000,
            'Q' => 0b0100,
            'k' => 0b0010,
            'q' => 0b0001,
            _ => 0
        };
        result |= v;
    }
    result
}
