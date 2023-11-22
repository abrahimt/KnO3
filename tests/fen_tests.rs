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

#[test]
fn test_parse_en_passant() {
    let mut cb = Chessboard::empty();

    let passant = "-"; // empty
    fen_util::parse_en_passant(&mut cb, passant);
    assert!(cb.en_passant == 0);

    let passant = "a1";
    fen_util::parse_en_passant(&mut cb, passant);
    assert!(cb.en_passant == 1);

    let passant = "A1";
    fen_util::parse_en_passant(&mut cb, passant);
    assert!(cb.en_passant == 1);

    let passant = "h8";
    fen_util::parse_en_passant(&mut cb, passant);
    assert!(cb.en_passant == 64);

    let passant = "-"; // set it back to empty
    fen_util::parse_en_passant(&mut cb, passant);
    //assert!(cb.en_passant == 0);

    // INVALID
    let passant = "a0";
    fen_util::parse_en_passant(&mut cb, passant);
    println!("The enpassant is {}", cb.en_passant);
    //assert!(cb.en_passant > 64 || cb.en_passant == 0);

    let passant = "h9";
    fen_util::parse_en_passant(&mut cb, passant);
    println!("The enpassant is {}", cb.en_passant);
    //assert!(cb.en_passant > 64 || cb.en_passant == 0);

    let passant = "z1";
    fen_util::parse_en_passant(&mut cb, passant);
    println!("The enpassant is {}", cb.en_passant);
    assert!(cb.en_passant > 64 || cb.en_passant == 0);
}

#[test]
fn test_get_en_passant() {
    let mut cb = Chessboard::empty();
    assert!(fen_util::get_fen_passant(&cb) == "-"); // empty

    cb = Chessboard::new();
    assert!(fen_util::get_fen_passant(&cb) == "-"); // new game

    cb.en_passant = 0;
    assert!(fen_util::get_fen_passant(&cb) == "-");

    cb.en_passant = 1;
    assert!(fen_util::get_fen_passant(&cb) == "A1");

    cb.en_passant = 64;
    assert!(fen_util::get_fen_passant(&cb) == "H8");

    cb.en_passant = 65;
    //assert!(fen_util::get_fen_passant(&cb) == "-");

    cb.en_passant = 255;
    //assert!(fen_util::get_fen_passant(&cb) == "-");
}

#[test]
fn test_parse_and_get_en_passant() {
    let mut cb = Chessboard::empty();
    let passant_str = fen_util::get_fen_passant(&cb);
    fen_util::parse_en_passant(&mut cb, &passant_str);
    assert!(cb.en_passant == 0);

    cb.en_passant = 64;
    let passant_str = fen_util::get_fen_passant(&cb);
    fen_util::parse_en_passant(&mut cb, &passant_str);
    assert!(cb.en_passant == 64);
}

/*
   place_pieces
   get_fen_placement
   parse_piece_placement

   get_fen_castles
   parse_castling_rights
*/
