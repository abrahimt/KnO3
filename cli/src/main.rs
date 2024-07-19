use crate::display::DisplayBoard;
mod display;
use std::fmt;
use kno3_chess_engine::GameState;
use kno3_chess_engine::position;
use clap::{Arg, Command};

#[derive(Debug)]
enum Error {
    FENParsingError(String),
    ArgumentError(String)
}

// I don't think this is doing anything but it's suppressing a warning -Cooper
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FENParsingError(msg) => write!(f, "FEN Parsing error: {}", msg),
            Error::ArgumentError(msg) => write!(f, "Argument error: {}", msg)
        }
    }
}

fn main() -> Result<(), Error> {
    let matches = Command::new("KnO3 Chess CLI")
        .version("1.0")
        .about("CLI for interacting with chess games")
        .arg(
            Arg::new("fen")
                .short('f')
                .long("fen")
                .value_name("FEN")
                .help("FEN string representing current game")
                .required(true)
        )
        .arg(
            Arg::new("show")
                .long("show")
                .help("Prints the state of the board")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("get-moves")
                .long("get-moves")
                .short('m')
                .value_name("POSITION")
                .help("Get possible moves for piece at the given position (ex: 'e2')")
        ).get_matches();

    let fen = matches.get_one::<String>("fen").ok_or(Error::ArgumentError("FEN string required".to_string()))?;
    let gs = GameState::from_string(fen).map_err(|e| Error::FENParsingError(e.to_string()))?;

    // Setters //


    // Show //
    if matches.get_flag("show") { gs.board.display(); }

    // Getters //
    if let Some(position) = matches.get_one::<String>("get-moves") {
        let square = position::string_to_square(position).map_err(|e| Error::ArgumentError(e.to_string()))?;
        let moves = gs.possible_moves(square);
        println!("{:?}", moves);
    }

    Ok(())
}
// cargo run -- -f "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50" --get-moves f7 --show
