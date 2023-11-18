mod board;
use board::Chessboard;

fn main() {
    let cb = Chessboard::new();
    cb.print(true);
}



//min function
fn min<T: Ord>(a: T, b: T) -> T {
    if a < b { a } else { b }
}
//max function
fn max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
