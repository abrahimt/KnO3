mod board;
use board::Chessboard;
use std::io::stdout;
use crossterm::{
    execute,
    style::{ Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

fn main() {
    let mut cb = Chessboard::new();
    cb.print(true);
    println!("New board {:?}", cb.to_string());

    cb = cb.from_string("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");

    cb.print(true);
    println!("E4 only {:?}", cb.to_string());

    cb = cb.from_string("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2");
    
    cb.print(true);
    println!("c5 {:?}", cb.to_string());

    cb = cb.from_string("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
    
    cb.print(true);
    println!("nf3 {:?}", cb.to_string());
}



//min function
fn min<T: Ord>(a: T, b: T) -> T {
    if a < b { a } else { b }
}
//max function
fn max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
