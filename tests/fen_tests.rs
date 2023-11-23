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

#[test]
fn test_get_fen_castles() {
    let mut cb = Chessboard::empty();
    assert!(fen_util::get_fen_castles(&cb) == "-");

    cb.castling_rights = 0b1000;
    assert!(fen_util::get_fen_castles(&cb) == "K");
    cb.castling_rights = 0b0100;
    assert!(fen_util::get_fen_castles(&cb) == "Q");
    cb.castling_rights = 0b0010;
    assert!(fen_util::get_fen_castles(&cb) == "k");
    cb.castling_rights = 0b0001;
    assert!(fen_util::get_fen_castles(&cb) == "q");

    cb.castling_rights = 0b1001;
    assert!(fen_util::get_fen_castles(&cb) == "Kq");
}

#[test]
fn test_parse_castling_rights() {
    let mut cb = Chessboard::empty();
    assert!(cb.castling_rights == 0);

    fen_util::parse_castling_rights(&mut cb, "K");
    assert!(cb.castling_rights == 0b1000);
    fen_util::parse_castling_rights(&mut cb, "Q");
    assert!(cb.castling_rights == 0b0100);
    fen_util::parse_castling_rights(&mut cb, "k");
    assert!(cb.castling_rights == 0b0010);
    fen_util::parse_castling_rights(&mut cb, "q");
    assert!(cb.castling_rights == 0b0001);
    fen_util::parse_castling_rights(&mut cb, "-");
    assert!(cb.castling_rights == 0);


    fen_util::parse_castling_rights(&mut cb, "Kk");
    assert!(cb.castling_rights == 0b1010);
    fen_util::parse_castling_rights(&mut cb, "Kq");
    assert!(cb.castling_rights == 0b1001);
}

#[test]
fn test_get_and_parse_castling_rights() {
    let mut cb = Chessboard::empty();
    fen_util::parse_castling_rights(&mut cb, "-");
    assert!(fen_util::get_fen_castles(&cb) == "-");

    fen_util::parse_castling_rights(&mut cb, "K");
    assert!(fen_util::get_fen_castles(&cb) == "K");
    fen_util::parse_castling_rights(&mut cb, "Q");
    assert!(fen_util::get_fen_castles(&cb) == "Q");
    fen_util::parse_castling_rights(&mut cb, "k");
    assert!(fen_util::get_fen_castles(&cb) == "k");
    fen_util::parse_castling_rights(&mut cb, "q");
    assert!(fen_util::get_fen_castles(&cb) == "q");

    fen_util::parse_castling_rights(&mut cb, "Kk");
    assert!(fen_util::get_fen_castles(&cb) == "Kk");
    fen_util::parse_castling_rights(&mut cb, "Kq");
    assert!(fen_util::get_fen_castles(&cb) == "Kq");
}

#[test]
fn test_get_fen_placement() {
    let mut cb = Chessboard::empty();
    assert!(fen_util::get_fen_placement(&cb) == "8/8/8/8/8/8/8/8");

    cb.black_pawns = 1;
    assert!(fen_util::get_fen_placement(&cb) == "8/8/8/8/8/8/8/p7");

    // reset board
    cb.black_pawns = 0;
    assert!(fen_util::get_fen_placement(&cb) == "8/8/8/8/8/8/8/8");

    cb.black_pawns = 0b11111111;
    assert!(fen_util::get_fen_placement(&cb) == "8/8/8/8/8/8/8/pppppppp");

    cb.black_pawns = 0b1111111100000000;
    assert!(fen_util::get_fen_placement(&cb) == "8/8/8/8/8/8/pppppppp/8");

    cb.black_pawns = 0b1;
    cb.black_rooks = 0b10;
    cb.black_knights = 0b100;
    cb.black_bishops = 0b1000;
    cb.black_queen = 0b10000;
    cb.black_king = 0b100000;
    assert!(fen_util::get_fen_placement(&cb) == "8/8/8/8/8/8/8/prnbqk2");

    cb.white_pawns = 0b1;
    cb.white_rooks = 0b10;
    cb.white_knights = 0b100;
    cb.white_bishops = 0b1000;
    cb.white_queen = 0b10000;
    cb.white_king = 0b100000;
    assert!(fen_util::get_fen_placement(&cb) == "8/8/8/8/8/8/8/PRNBQK2");

    cb = Chessboard::new();
    assert!(fen_util::get_fen_placement(&cb) == "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    cb = Chessboard::empty();
    cb.black_pawns = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111;
    //let s = fen_util::get_fen_placement(&cb);
    //println!("FEN PLACEMENT {}!!!", s);
    assert!(fen_util::get_fen_placement(&cb) == "pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp");
}

#[test]
fn test_place_pieces() {
    let mut cb = Chessboard::empty();
    assert!(cb.black_pawns == 0);

    fen_util::place_pieces(&mut cb, "8/8/8/8/8/8/8/pppppppp");
    assert!(cb.black_pawns == 255);

    cb = Chessboard::empty();
    fen_util::place_pieces(&mut cb, "8/8/8/8/8/8/8/7p");
    assert!(cb.black_pawns == 0b10000000);

    cb = Chessboard::empty();
    fen_util::place_pieces(&mut cb, "8/8/8/8/8/8/8/prbkqn2");
    assert!(cb.black_pawns == 1);
    assert!(cb.black_rooks == 2);
    assert!(cb.black_bishops == 4);
    assert!(cb.black_king == 8);
    assert!(cb.black_queen == 16);
    assert!(cb.black_knights == 32);

    cb = Chessboard::empty();
    fen_util::place_pieces(&mut cb, "8/8/8/8/8/8/8/PRBKQN2");
    assert!(cb.white_pawns == 1);
    assert!(cb.white_rooks == 2);
    assert!(cb.white_bishops == 4);
    assert!(cb.white_king == 8);
    assert!(cb.white_queen == 16);
    assert!(cb.white_knights == 32);

    cb = Chessboard::empty();
    fen_util::place_pieces(&mut cb, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    assert!(cb.black_pawns   == 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000);
    assert!(cb.black_rooks   == 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    assert!(cb.black_bishops == 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    assert!(cb.black_king    == 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    assert!(cb.black_queen   == 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    assert!(cb.black_knights == 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    assert!(cb.white_pawns   == 0b11111111_00000000);
    assert!(cb.white_rooks   == 0b10000001);
    assert!(cb.white_bishops == 0b00100100);
    assert!(cb.white_king    == 0b00010000);
    assert!(cb.white_queen   == 0b00001000);
    assert!(cb.white_knights == 0b01000010);
}
