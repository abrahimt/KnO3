mod main;
use main::min;
use main::max;

pub struct Chessboard {
    pub(crate) black_pawns: u64,
    pub(crate) black_rooks: u64,
    pub(crate) black_knights: u64,
    pub(crate) black_bishops: u64,
    pub(crate) black_queen: u64,
    pub(crate) black_king: u64,
    pub(crate) white_pawns: u64,
    pub(crate) white_rooks: u64,
    pub(crate) white_knights: u64,
    pub(crate) white_bishops: u64,
    pub(crate) white_queen: u64,
    pub(crate) white_king: u64,
    pub(crate) white_turn: bool,
    pub(crate) white_castle: u8, //11, 01 (representing sides of the board)
    pub(crate) black_castle: u8, //11, 01 (representing sides of the board)
    pub(crate) en_passant: u8,   //a square that has en passant ability (1-64)
}

impl Chessboard {
    pub fn initialize_board(&mut self) {
        // white pieces
        self.white_pawns = 0b0000000000000000000000000000000000000000000000001111111100000000;
        self.white_knights = 0b0000000000000000000000000000000000000000000000000000000001000010;
        self.white_bishops = 0b0000000000000000000000000000000000000000000000000000000000100100;
        self.white_king = 0b0000000000000000000000000000000000000000000000000000000000001000;
        self.white_queen = 0b0000000000000000000000000000000000000000000000000000000000010000;
        self.white_rooks = 0b0000000000000000000000000000000000000000000000000000000010000001;
        // black pieces
        self.black_pawns = 0b0000000011111111000000000000000000000000000000000000000000000000;
        self.black_knights = 0b0100001000000000000000000000000000000000000000000000000000000000;
        self.black_bishops = 0b0010010000000000000000000000000000000000000000000000000000000000;
        self.black_king = 0b0000100000000000000000000000000000000000000000000000000000000000;
        self.black_queen = 0b0001000000000000000000000000000000000000000000000000000000000000;
        self.black_rooks = 0b1000000100000000000000000000000000000000000000000000000000000000;
    }

    fn whose_turn(&self) -> &str {
        if self.white_turn {
            "white"
        } else {
            "black"
        }
    }


    //MINIMAX Function Pseudo-code
    // fn minimax(position, depth, alpha, beta, maximixing_player) {
    //     if depth == 0 or game over in position
    //         return static evaluation of position
    //     if maximizing_player (white)
    //         max_eval = -infinity
    //         for each child of position
    //             eval = minimax(child, depth - 1, alpha, beta, false)
    //             max_eval = max(max_eval, eval)
    //             alpha = max(alpha, eval)
    //             if beta <= alpha
    //                 break
    //         return max_eval
    //     else
    //         min_eval = +infinity
    //         for each child of position
    //         eval = minimax(child, depth - 1, alpha, beta, true)
    //         min_eval = min(min_eval, eval)
    //         beta = min(beta, eval)
    //         if beta <= alpha
    //             break
    //     return min_eval
    // }
}
//Below here is just ideas for functions:

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