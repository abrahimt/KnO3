mod board;
mod fen_util;
use board::Chessboard;

fn main() {
    let mut cb = Chessboard::new();
    cb.print(true);

    // Move a white pawn from E2 to E3
    cb.move_piece("E2", "E3", 'p');
    println!("{}", cb.to_string());
    cb.print(true);
    // Move a black knight from G8 to F6
    cb.move_piece("G8", "F6", 'n');
    println!("{}", cb.to_string());
    cb.print(true);
    // Move a white queen from D1 to H5
    cb.move_piece("D1", "H5", 'Q');
    println!("{}", cb.to_string());
    cb.print(true);
    // println!("E4 only {:?}", cb.to_string());

    let x = Chessboard::square_to_rank_file(22);
    println!("{} {}", x.0, x.1);

    cb = Chessboard::from_string("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2")
        .unwrap();
    cb.print(true);

    let mut legal: bool = false;
    legal = Chessboard::legal_move(cb, 'p', "E4", "E5");
    println!("{}", legal);
}

// //min function
// #[rustfmt::skip]
// fn min<T: Ord>(a: T, b: T) -> T {
//     if a < b { a } else { b }
// }
// //max function
// #[rustfmt::skip]
// fn max<T: Ord>(a: T, b: T) -> T {
//     if a > b { a } else { b }
// }
