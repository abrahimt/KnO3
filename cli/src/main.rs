mod display;

use display::DisplayBoard;
use kno3_chess_engine::Chessboard;

fn main() {
    let cb = Chessboard::new();
    cb.display();
    println!("Hello, world!");
}
