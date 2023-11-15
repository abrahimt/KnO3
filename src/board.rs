const BOARD_SIZE: usize = 64;

struct Chessboard {
    black_pawns: u64,
    black_rooks: u64,
    black_knights: u64,
    black_bishops: u64,
    black_queen: u64,
    black_king: u64,
    white_pawns: u64,
    white_rooks: u64,
    white_knights: u64,
    white_bishops: u64,
    white_queen: u64,
    white_king: u64,
    white_turn: bool,
}

impl Chessboard {

    fn whose_turn(&self) -> &str {
        if self.white_turn {
            "white"
        } else {
            "black"
        }
    }

    //fn what_occupy square() {
    //     check each piece type with the square and see which returns true
    // }

    // fn is_square_occupied(square) {
    // bitwise and every piece together
    // search through the new u64 and see where there are 1s
    // return the indices of the 1s
    // }

    // en_passant_square() {
    //     // Return the square where an en passant capture is possible, or -1 if not possible
    //     // For example, if the last move was a double-step pawn move, return the square where the opponent's pawn can capture en passant.
    // }

    // castling_rights() {
    //     // Use flags or bitmasks to represent castling rights for both players
    //     // For example, if kingside castling is allowed for white, set white_kingside_castle = true
    //     // Similarly, maintain flags for black's castling rights
    // }

    // make_move(move) {
    //     // Update the bitboards and game state based on the move
    //     // For example, if it's a pawn move, update the pawn bitboard
    //     // If it's a capture, clear the bit for the captured piece
    //     // Update en passant square and castling rights accordingly
    //     // Switch turns: white_turn = !white_turn;
    // }

    // generate_legal_moves() {
    //     legal_moves = []

    //     for each piece in pieces_of_current_turn {
    //         possible_moves = generate_moves_for_piece(piece)
    //         for each move in possible_moves {
    //             if is_legal_move(move) {
    //                 legal_moves.push(move)
    //             }
    //         }
    //     }
    //
    //     return legal_moves
    // }

    // generate_moves_for_piece(piece) {
    //     // Generate all possible moves for the given piece
    //     // Consider piece-specific movement rules (e.g., pawn's initial double move, castling for king, etc.)
    // }

    // is_legal_move(move) {
    //     // Check if the given move is legal
    //     // Verify that the move adheres to the rules of chesce-s, including piespecific rules and board state
    //     // Check for legality includes considerations like not moving into check, en passant captures, castling rules, etc.
    // }
}
