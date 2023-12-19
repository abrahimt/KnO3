use kn_o3::board::Chessboard;

#[test]
fn test_legal_pawn() {
    assert!(Chessboard::is_valid_move_for_piece('P', 0, 8)); // One square forward
    assert!(Chessboard::is_valid_move_for_piece('p', 8, 0));

    assert!(!Chessboard::is_valid_move_for_piece('P', 0, 16)); // Two squares forward
    assert!(!Chessboard::is_valid_move_for_piece('p', 16, 0));
    assert!(Chessboard::is_valid_move_for_piece('P', 8, 24));
    assert!(Chessboard::is_valid_move_for_piece('P', 15, 31));
    assert!(Chessboard::is_valid_move_for_piece('p', 48, 32));
    assert!(Chessboard::is_valid_move_for_piece('p', 55, 39));

    assert!(!Chessboard::is_valid_move_for_piece('P', 8, 0)); // One square backward
    assert!(!Chessboard::is_valid_move_for_piece('p', 0, 8));
    assert!(!Chessboard::is_valid_move_for_piece('p', 8, 16));

    assert!(!Chessboard::is_valid_move_for_piece('P', 1, 8)); // Adjacent
    assert!(!Chessboard::is_valid_move_for_piece('P', 1, 0));
    assert!(!Chessboard::is_valid_move_for_piece('P', 1, 10));
    assert!(!Chessboard::is_valid_move_for_piece('P', 1, 2));
    assert!(!Chessboard::is_valid_move_for_piece('p', 57, 56));
    assert!(!Chessboard::is_valid_move_for_piece('p', 57, 48));
    assert!(!Chessboard::is_valid_move_for_piece('p', 57, 58));
    assert!(!Chessboard::is_valid_move_for_piece('p', 57, 50));

    assert!(!Chessboard::is_valid_move_for_piece('P', 1, 1)); // Same spot
    assert!(!Chessboard::is_valid_move_for_piece('p', 1, 1));
}

#[test]
fn test_legal_rook() {
    // Same file
    assert!(Chessboard::is_valid_move_for_piece('r', 0, 8));
    assert!(Chessboard::is_valid_move_for_piece('r', 0, 16));
    assert!(Chessboard::is_valid_move_for_piece('r', 0, 56));
    assert!(Chessboard::is_valid_move_for_piece('r', 56, 0));
    assert!(Chessboard::is_valid_move_for_piece('r', 8, 15));

    // Same file
    assert!(Chessboard::is_valid_move_for_piece('R', 0, 8));
    assert!(Chessboard::is_valid_move_for_piece('R', 0, 16));
    assert!(Chessboard::is_valid_move_for_piece('R', 0, 56));
    assert!(Chessboard::is_valid_move_for_piece('R', 56, 0));
    assert!(Chessboard::is_valid_move_for_piece('R', 8, 15));

    // Same rank
    assert!(Chessboard::is_valid_move_for_piece('r', 0, 1));
    assert!(Chessboard::is_valid_move_for_piece('r', 0, 2));
    assert!(Chessboard::is_valid_move_for_piece('r', 0, 7));
    assert!(Chessboard::is_valid_move_for_piece('r', 7, 0));
    assert!(Chessboard::is_valid_move_for_piece('r', 8, 48));

    // Same rank
    assert!(Chessboard::is_valid_move_for_piece('R', 0, 1));
    assert!(Chessboard::is_valid_move_for_piece('R', 0, 2));
    assert!(Chessboard::is_valid_move_for_piece('R', 0, 7));
    assert!(Chessboard::is_valid_move_for_piece('R', 7, 0));
    assert!(Chessboard::is_valid_move_for_piece('R', 8, 48));

    // Different file && rank
    assert!(!Chessboard::is_valid_move_for_piece('r', 0, 9));
    assert!(!Chessboard::is_valid_move_for_piece('r', 0, 17));
    assert!(!Chessboard::is_valid_move_for_piece('r', 0, 39));
    assert!(!Chessboard::is_valid_move_for_piece('r', 0, 57));
    assert!(!Chessboard::is_valid_move_for_piece('r', 0, 63));
    assert!(!Chessboard::is_valid_move_for_piece('r', 9, 0));
    assert!(!Chessboard::is_valid_move_for_piece('r', 17, 0));
    assert!(!Chessboard::is_valid_move_for_piece('r', 39, 0));
    assert!(!Chessboard::is_valid_move_for_piece('r', 57, 0));
    assert!(!Chessboard::is_valid_move_for_piece('r', 63, 0));

    // Different file && rank
    assert!(!Chessboard::is_valid_move_for_piece('R', 0, 9));
    assert!(!Chessboard::is_valid_move_for_piece('R', 0, 17));
    assert!(!Chessboard::is_valid_move_for_piece('R', 0, 39));
    assert!(!Chessboard::is_valid_move_for_piece('R', 0, 57));
    assert!(!Chessboard::is_valid_move_for_piece('R', 0, 63));
    assert!(!Chessboard::is_valid_move_for_piece('R', 9, 0));
    assert!(!Chessboard::is_valid_move_for_piece('R', 17, 0));
    assert!(!Chessboard::is_valid_move_for_piece('R', 39, 0));
    assert!(!Chessboard::is_valid_move_for_piece('R', 57, 0));
    assert!(!Chessboard::is_valid_move_for_piece('R', 63, 0));

    // Same square
    assert!(!Chessboard::is_valid_move_for_piece('r', 0, 0));
    assert!(!Chessboard::is_valid_move_for_piece('r', 57, 57));

    // Same square
    assert!(!Chessboard::is_valid_move_for_piece('R', 0, 0));
    assert!(!Chessboard::is_valid_move_for_piece('R', 57, 57));
}

#[test]
fn test_legal_bishop() {
    // Diagonal
    assert!(Chessboard::is_valid_move_for_piece('b', 35, 26));
    assert!(Chessboard::is_valid_move_for_piece('b', 35, 8));
    assert!(Chessboard::is_valid_move_for_piece('b', 35, 42));
    assert!(Chessboard::is_valid_move_for_piece('b', 35, 56));
    assert!(Chessboard::is_valid_move_for_piece('b', 35, 44));
    assert!(Chessboard::is_valid_move_for_piece('b', 35, 62));
    assert!(Chessboard::is_valid_move_for_piece('b', 35, 28));
    assert!(Chessboard::is_valid_move_for_piece('b', 35, 7));
    assert!(Chessboard::is_valid_move_for_piece('b', 0, 63));
    assert!(Chessboard::is_valid_move_for_piece('b', 63, 0));

    // Diagonal
    assert!(Chessboard::is_valid_move_for_piece('B', 35, 26));
    assert!(Chessboard::is_valid_move_for_piece('B', 35, 8));
    assert!(Chessboard::is_valid_move_for_piece('B', 35, 42));
    assert!(Chessboard::is_valid_move_for_piece('B', 35, 56));
    assert!(Chessboard::is_valid_move_for_piece('B', 35, 44));
    assert!(Chessboard::is_valid_move_for_piece('B', 35, 62));
    assert!(Chessboard::is_valid_move_for_piece('B', 35, 28));
    assert!(Chessboard::is_valid_move_for_piece('B', 35, 7));
    assert!(Chessboard::is_valid_move_for_piece('B', 0, 63));
    assert!(Chessboard::is_valid_move_for_piece('B', 63, 0));

    // Horizontal
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 34));
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 32));
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 36));
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 39));

    // Horizontal
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 34));
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 32));
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 36));
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 39));

    // Vertical
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 27));
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 3));
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 43));
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 59));

    // Vertical
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 27));
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 3));
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 43));
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 59));

    // Same square
    assert!(!Chessboard::is_valid_move_for_piece('b', 35, 35));

    // Same square
    assert!(!Chessboard::is_valid_move_for_piece('B', 35, 35));
}

#[test]
fn test_legal_king() {
    // Move 1 square
    assert!(Chessboard::is_valid_move_for_piece('k', 35, 44)); // ne
    assert!(Chessboard::is_valid_move_for_piece('k', 35, 43)); // up
    assert!(Chessboard::is_valid_move_for_piece('k', 35, 42)); // nw
    assert!(Chessboard::is_valid_move_for_piece('k', 35, 36)); // right
    assert!(Chessboard::is_valid_move_for_piece('k', 35, 34)); // left
    assert!(Chessboard::is_valid_move_for_piece('k', 35, 28)); // se
    assert!(Chessboard::is_valid_move_for_piece('k', 35, 27)); // down
    assert!(Chessboard::is_valid_move_for_piece('k', 35, 26)); // sw

    // Move 1 square
    assert!(Chessboard::is_valid_move_for_piece('K', 35, 44)); // ne
    assert!(Chessboard::is_valid_move_for_piece('K', 35, 43)); // up
    assert!(Chessboard::is_valid_move_for_piece('K', 35, 42)); // nw
    assert!(Chessboard::is_valid_move_for_piece('K', 35, 36)); // right
    assert!(Chessboard::is_valid_move_for_piece('K', 35, 34)); // left
    assert!(Chessboard::is_valid_move_for_piece('K', 35, 28)); // se
    assert!(Chessboard::is_valid_move_for_piece('K', 35, 27)); // down
    assert!(Chessboard::is_valid_move_for_piece('K', 35, 26)); // sw

    // Same square
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 35));

    // Same square
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 35));

    // Move more than 1 square
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 47));
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 51));
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 45));
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 37));
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 33));
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 25));
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 19));
    assert!(!Chessboard::is_valid_move_for_piece('k', 35, 22));

    // Move more than 1 square
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 47));
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 51));
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 45));
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 37));
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 33));
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 25));
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 19));
    assert!(!Chessboard::is_valid_move_for_piece('K', 35, 22));
}

#[test]
fn test_legal_knight() {
    assert!(Chessboard::is_valid_move_for_piece('n', 0, 17));
    assert!(Chessboard::is_valid_move_for_piece('n', 0, 15));
    assert!(Chessboard::is_valid_move_for_piece('n', 0, 10));
    assert!(Chessboard::is_valid_move_for_piece('n', 0, 6));

    assert!(!Chessboard::is_valid_move_for_piece('n', 0, 16));
    assert!(!Chessboard::is_valid_move_for_piece('n', 0, 8));
    assert!(!Chessboard::is_valid_move_for_piece('n', 0, 9));
    assert!(!Chessboard::is_valid_move_for_piece('n', 0, 7));

    assert!(Chessboard::is_valid_move_for_piece('N', 0, 17));
    assert!(Chessboard::is_valid_move_for_piece('N', 0, 15));
    assert!(Chessboard::is_valid_move_for_piece('N', 0, 10));
    assert!(Chessboard::is_valid_move_for_piece('N', 0, 6));

    assert!(!Chessboard::is_valid_move_for_piece('N', 0, 16));
    assert!(!Chessboard::is_valid_move_for_piece('N', 0, 8));
    assert!(!Chessboard::is_valid_move_for_piece('N', 0, 9));
    assert!(!Chessboard::is_valid_move_for_piece('N', 0, 7));
}
