mod game_state; // FEN stuff
mod chessboard; // board representation
mod state; // Seeing where pieces already are
mod move_generation; // Seeing which pieces can go where

pub mod position; // converting square <-> coordinate
pub use chessboard::Chessboard;
pub use game_state::GameState;
