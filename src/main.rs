mod board;
use board::Chessboard;
use std::io::stdout;
use crossterm::{
    execute,
    style::{ Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

fn main() {
    let cb = Chessboard::new();
    /*
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
        );
        */
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
