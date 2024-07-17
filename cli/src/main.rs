use crate::display::DisplayBoard;
mod display;

use kno3_chess_engine::GameState;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// FEN string representing board state
    #[clap(short, long)]
    fen: String
}

fn main() {
    let args: Args = Args::parse();
    let gs = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    gs.board.display();
}
