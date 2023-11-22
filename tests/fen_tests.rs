use kn_o3::board::Chessboard;
use kn_o3::fen_util;

/**
  Forsynth-Edwards Notation (FEN)
  https://www.chess.com/terms/fen-chess

  rows rnbqkbnr
  turn w|b
  castle KQkq
  enpassant c3 | -
  halfmove clock < 50
  fullmove number += 1 on black turn
*/


#[test]
fn test_whose_turn() {
    let mut cb = Chessboard::empty();

    fen_util::parse_whose_turn(&mut cb, "w");
    assert!(cb.white_turn == true);

    fen_util::parse_whose_turn(&mut cb, "b");
    assert!(cb.white_turn == false);
}

#[test]
fn test_valid_fen() {
    // Empty
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 w - - 0 0"));
    // Start of a new game
    assert!(fen_util::valid_fen("rnbkqbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBKQBNR w KQkq - 0 0"));

    // en passant
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 w - a1 0 0"));
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 w - a8 0 0"));
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 w - h1 0 0"));
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 w - h8 0 0"));

    // turn
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 w - h8 0 0"));
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 b - h8 0 0"));

    // Castling
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 b KQkq h8 0 0"));
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 b kq h8 0 0"));
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 b KQ h8 0 0"));
    assert!(fen_util::valid_fen("8/8/8/8/8/8/8/8 b KQk h8 0 0"));

    // Pieces
    assert!(fen_util::valid_fen("7p/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7P/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7r/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7R/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7b/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7B/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7k/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7K/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7q/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7Q/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7n/8/8/8/8/8/8/8 w - - 0 0"));
    assert!(fen_util::valid_fen("7N/8/8/8/8/8/8/8 w - - 0 0"));
}

#[test]
fn test_invalid_fen() {
    // Only positions
    assert!(!fen_util::valid_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8"));
    assert!(!fen_util::valid_fen("8/7k/8/8/8/8/8/8"));

    // Missing parts
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w - h8 0")); //move number
    assert!(!fen_util::valid_fen("/8/8/8/8/8/8/8 w - h8 0 0")); // rank
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8 w - h8 0 0")); // rank
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 - h8 0 0")); // turn
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w h8 0 0")); // castles
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w - 0 0")); // en passant

    // Invalid parts
    assert!(!fen_util::valid_fen("9/8/8/8/8/8/8/8 w - a1 0 0")); // file out of range
    assert!(!fen_util::valid_fen("0/8/8/8/8/8/8/8 w - a1 0 0")); // file out of range
    assert!(!fen_util::valid_fen("7p1/8/8/8/8/8/8/8 w - a1 0 0")); // file out of range
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8/8 w - a1 0 0")); // extra rank
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 x - a1 0 0")); // invalid turn
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 wb - a1 0 0")); // two turns
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w x a1 0 0")); // invalid castle
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 x kqx a1 0 0")); // invalid castles
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w KQK a1 0 0")); // invalid castle
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w - z1 0 0")); // invalid en passant
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w - a0 0 0")); // invalid en passant
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w - a9 0 0")); // invalid en passant
    assert!(!fen_util::valid_fen("8/8/8/8/8/8/8/8 w - 9 0 0")); // invalid en passant
    assert!(!fen_util::valid_fen("7x/8/8/8/8/8/8/8 w - a1 0 0")); // invalid piece
}

/*
   Functions to test
   valid_fen

   place_pieces
   get_fen_placement
   parse_piece_placement

   get_fen_castles
   parse_castling_rights

   get_fen_passant
   parse_en_passant
*/
