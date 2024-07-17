use crate::display::DisplayBoard;
mod display;

use kno3_chess_engine::Chessboard;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// FEN string representing board state
    #[clap(short, long)]
    fen: String
}

fn main() {
    let args: Args = Args::parse();
    let cb = Chessboard::new();
    cb.display();
}
