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
    pub(crate) castling_rights: u32, //KQkq
    pub(crate) en_passant: u32, //a square that has en passant ability (1-64) 0 means no en passant
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

    // FEN Starting position
    // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    // FEN After E4
    // rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1

    // parser
    pub fn from_string(&self, fen: &str) -> Chessboard {
        let mut chessboard = Chessboard {
            black_pawns: 0,
            black_rooks: 0,
            black_knights: 0,
            black_bishops: 0,
            black_queen: 0,
            black_king: 0,
            white_pawns: 0,
            white_rooks: 0,
            white_knights: 0,
            white_bishops: 0,
            white_queen: 0,
            white_king: 0,
            castling_rights: 0,
            white_turn: true,
            en_passant: 0,
        };

        println!("White Pawns:   {:064b}", chessboard.white_pawns);
        println!("White Knights: {:064b}", chessboard.white_knights);
        println!("White Bishops: {:064b}", chessboard.white_bishops);
        println!("White King:    {:064b}", chessboard.white_king);
        println!("White Queen:   {:064b}", chessboard.white_queen);
        println!("White Rooks:   {:064b}", chessboard.white_rooks);

        println!("Black Pawns:   {:064b}", chessboard.black_pawns);
        println!("Black Knights: {:064b}", chessboard.black_knights);
        println!("Black Bishops: {:064b}", chessboard.black_bishops);
        println!("Black King:    {:064b}", chessboard.black_king);
        println!("Black Queen:   {:064b}", chessboard.black_queen);
        println!("Black Rooks:   {:064b}", chessboard.black_rooks);

        println!("En passant:    {:064b}", chessboard.en_passant);
        println!("Castling:    {:064b}", chessboard.castling_rights);
        println!("White turn {}", chessboard.white_turn);

        //Split fen with ' ' as delimiter
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();
        println!("FEN parts whitespace split {:?}", fen_parts);
        // ["rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR", "b", "KQkq", "e3", "0", "1"]

        //Get the piece placement
        let board_rows: Vec<&str> = fen_parts[0].split('/').collect();
        println!("board_rows slash split {:?}", board_rows);

        //Get whose turn it is
        chessboard.white_turn = fen_parts[1] == "w";

        //Get the castling rights
        let fen_castle = fen_parts[2];
        let mut castles = 0;
        for c in fen_castle.chars() {
            let v = match c {
                'K' => 0b1000,
                'Q' => 0b0100,
                'k' => 0b0010,
                'q' => 0b0001,
                _ => 0b0,
            };
            castles = castles | v;
        } 
        chessboard.castling_rights = castles;

        //Get the en passant square
        let fen_passant = fen_parts[3];
        if fen_passant != "-" {
            if let (Some(col), Some(row)) = (
                fen_passant.chars().nth(0).map(|c| c.to_ascii_uppercase()),
                fen_passant.chars().nth(1).and_then(|c| c.to_digit(10)),
            ) {
                if (1..=8).contains(&row) {
                    let col_value = match col {
                        'A' => 1,
                        'B' => 2,
                        'C' => 3,
                        'D' => 4,
                        'E' => 5,
                        'F' => 6,
                        'G' => 7,
                        'H' => 8,
                        _ => 0, // Handle unexpected characters
                    };
                    //gives a number (1-64)
                    chessboard.en_passant = (9 - col_value) + 8 * (row - 1);
                }
            }
        }

        println!("White Pawns:   {:064b}", chessboard.white_pawns);
        println!("White Knights: {:064b}", chessboard.white_knights);
        println!("White Bishops: {:064b}", chessboard.white_bishops);
        println!("White King:    {:064b}", chessboard.white_king);
        println!("White Queen:   {:064b}", chessboard.white_queen);
        println!("White Rooks:   {:064b}", chessboard.white_rooks);

        println!("Black Pawns:   {:064b}", chessboard.black_pawns);
        println!("Black Knights: {:064b}", chessboard.black_knights);
        println!("Black Bishops: {:064b}", chessboard.black_bishops);
        println!("Black King:    {:064b}", chessboard.black_king);
        println!("Black Queen:   {:064b}", chessboard.black_queen);
        println!("Black Rooks:   {:064b}", chessboard.black_rooks);

        println!("En passant:    {:064b}", chessboard.en_passant);
        println!("Castling:    {:064b}", chessboard.castling_rights);
        println!("White turn {}", chessboard.white_turn);

        //Ignore rest of the FEN for now
        return chessboard;
    }

    // serializer
    // pub fn to_string(cb: Chessboard) -> &str {

    // }

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
