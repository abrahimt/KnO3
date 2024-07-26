mod chessboard; // board representation
mod game_state; // FEN stuff
mod move_generation;
mod state; // Seeing where pieces already are // Seeing which pieces can go where

pub mod position; // converting square <-> coordinate
pub use chessboard::Chessboard;
pub use game_state::GameState;
