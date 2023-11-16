use termion::{color, style};

enum DynamicColor { White, Black }
impl DynamicColor {
    fn to_termion(&self) -> &dyn color::Color {
        match self {
            DynamicColor::White => &color::White,
            DynamicColor::Black => &color::Black
        }
    }
}

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
    pub fn new() -> Chessboard {
        Chessboard {
            white_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            white_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            white_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            white_king: 0b0000000000000000000000000000000000000000000000000000000000001000,
            white_queen: 0b0000000000000000000000000000000000000000000000000000000000010000,
            white_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            black_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
            black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            black_king: 0b0000100000000000000000000000000000000000000000000000000000000000,
            black_queen: 0b0001000000000000000000000000000000000000000000000000000000000000,
            black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
            white_castle: 3,
            black_castle: 3,
            en_passant: 0,
            white_turn: true
        }
    }

    fn get_pieces(&self) -> Vec<(char, u64)> {
        vec![
            ('P', self.white_pawns),
            ('N', self.white_knights),
            ('B', self.white_bishops),
            ('K', self.white_king),
            ('Q', self.white_queen),
            ('R', self.white_rooks),
            ('p', self.black_pawns),
            ('n', self.black_knights),
            ('b', self.black_bishops),
            ('k', self.black_king),
            ('q', self.black_queen),
            ('r', self.black_rooks)
        ]
    }

    pub fn print(&self) {
        let ranks = [8, 7, 6, 5, 4, 3, 2, 1];
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        print!("  ");
        for file in files.iter() { print!("{file} "); }
        println!();

        for rank in ranks.iter() {
            print!("{rank} ");
            for file in 0..files.len() {
                let p = self.piece_at_position(*rank, file);
                print!("{p} ");
            }
            println!();
        }
    }

    pub fn pretty_print(&self) {
        let ranks = [8, 7, 6, 5, 4, 3, 2, 1];
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        print!("  ");
        for file in files.iter() { print!("{} ", self.format_piece(*file)); }
        println!();

        for rank in ranks.iter() {
            print!("{rank} ");
            for file in 0..files.len() {
                let p = self.piece_at_position(*rank, file);
                let np = self.format_piece(p);
                let bk = format!("{}{}", self.format_background(*rank, file), np);

                print!("{bk} ");
            }
            println!();
        }
    }


    fn format_piece(&self, piece: char) -> String {
        let dc: DynamicColor = if piece.is_uppercase() { DynamicColor::White } else { DynamicColor::Black };
        let color_code = dc.to_termion();
        let spaced = format!("{:^3}", piece);
        let colored = format!("{}{}{}", color::Fg(color_code), spaced, style::Reset);
        return colored;
    }

    fn format_background(&self, rank: usize, file: usize) -> String {
        let bg_color = match (rank + file) % 2 == 0 {
            true =>  color::Bg(color::Rgb(190, 140, 170)),
            false => color::Bg(color::Rgb(255, 206, 158))
        };
        format!("{}", bg_color)
    }

    fn piece_at_position(&self, rank: usize, file: usize) -> char { 
        for (p_type, positions) in self.get_pieces() {
            let rank_byte = positions >> ((rank - 1) * 8);
            if (rank_byte & (1 << file)) != 0 { return p_type; }
        }
        '.'
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
