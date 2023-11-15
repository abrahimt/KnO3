mod board;
use board::Chessboard;

fn main() {
    let cb = Chessboard::new();
    cb.print();
}
