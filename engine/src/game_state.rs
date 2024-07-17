use crate::position;
use crate::Chessboard;

// https://www.chess.com/terms/fen-chess
pub struct GameState {
    piece_placement: String,
    white_turn: bool,
    castling: u8, // KQkq will be represented by 4 bits
    en_passant: u8, // a square that has en passant ability
    half_clock: u32,
    move_count: u32,
    pub board: Chessboard
}

impl GameState {
    pub fn new() -> Self {
        Self {
            piece_placement: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
            white_turn: true,
            castling: 0x0F,
            en_passant: 255,
            half_clock: 0,
            move_count: 1,
            board: Chessboard::new()
        }
    }

    pub fn from_string(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 { return Err("Invalid FEN string".to_string()); }

        let passant = match parts[3] {
            "-" => 255, // out of range
            _ => position::string_to_square(parts[3])?
        };

        Ok(Self {
            piece_placement: parts[0].to_string(),
            white_turn: parts[1] == "w",
            castling: parse_castling_rights(parts[2]),
            en_passant: passant,
            half_clock: parts[4].parse().unwrap(),
            move_count: parts[5].parse().unwrap(),
            board: Chessboard::from_string(parts[0])?
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen = GameState::from_string(fen_str).unwrap();

        assert_eq!(fen.piece_placement, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(fen.white_turn, true);
        assert_eq!(fen.castling, 0b1111);
        assert_eq!(fen.en_passant, 255);
        assert_eq!(fen.half_clock, 0);
        assert_eq!(fen.move_count, 1);
    }

    #[test]
    fn test_passant() {
        let mut fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(fen.unwrap().en_passant, 255);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 0 1");
        assert_eq!(fen.unwrap().en_passant, 20);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq i3 0 1");
        assert!(fen.is_err());
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e9 0 1");
        assert!(fen.is_err());
    }

    #[test]
    fn test_castles() {
        let mut fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(fen.castling, 0b1111);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kk - 0 1").unwrap();
        assert_eq!(fen.castling, 0b1010);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qq - 0 1").unwrap();
        assert_eq!(fen.castling, 0b0101);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1").unwrap();
        assert_eq!(fen.castling, 0);
    }

    #[test]
    fn test_invalid_fen() {
        let mut result = GameState::from_string("invalid fen string");
        assert!(result.is_err());

        result = GameState::from_string("positions turn castles passant clock move");
        assert!(result.is_err());
    }
}
