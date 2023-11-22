use kn_o3::board::Chessboard;
use kn_o3::fen_util;

#[test]
fn test_whose_turn() {
    let mut cb = Chessboard::empty();

    fen_util::parse_whose_turn(&mut cb, "w");
    assert!(cb.white_turn == true);

    fen_util::parse_whose_turn(&mut cb, "b");
    assert!(cb.white_turn == false);
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
