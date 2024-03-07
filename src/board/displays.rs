use super::{position, Chessboard};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::{fmt, io::stdout};

impl Chessboard {
    /// Prints the chessboard to the console.
    ///
    /// # Arguments
    ///
    /// - `pretty`: A boolean indicating whether to print with extra formatting.
    ///
    /// # Example
    ///
    /// ```
    /// use kn_o3::board::Chessboard;
    /// Chessboard::new().print(true);
    /// ```
    ///
    /// The function prints the current state of the chessboard to the console. If `pretty` is
    /// set to `true`, it adds extra formatting, including colors for pieces and backgrounds.
    /// Otherwise, it prints a simple representation of the board with piece characters.
    pub fn print(&self, pretty: bool) {
        let ranks = [8, 7, 6, 5, 4, 3, 2, 1];
        let files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

        for rank in ranks.iter() {
            print!("{rank} ");
            for (f_index, file) in files.iter().enumerate() {
                let square = position::rank_file_to_square(*rank, *file).unwrap();
                let piece = self.piece_at_position(square).unwrap_or('.');
                if !pretty {
                    print!("{piece} ");
                    continue;
                }

                let fg = self.find_fg(piece);
                let frmt_piece = format!("{:^3}", piece);
                let bk = self.find_bkgnd(*rank, f_index as u8);
                let _ = execute!(
                    stdout(),
                    SetForegroundColor(fg),
                    SetBackgroundColor(bk),
                    Print(frmt_piece),
                    ResetColor
                );
            }
            println!();
        }

        print!("  ");
        for file in files.iter() {
            if pretty {
                print!(" {file} ")
            } else {
                print!("{file} ");
            }
        }
        println!();
    }

    /// Determines the foreground color based on the chess piece color.
    ///
    /// # Arguments
    ///
    /// - `p`: The character representation of a chess piece.
    ///
    /// # Returns
    ///
    /// The `Color` of the chess piece. If the piece is uppercase (representing a white piece),
    /// it returns `Color::White`; otherwise, it returns `Color::Black`.
    #[rustfmt::skip]
    fn find_fg(&self, p: char) -> Color {
        if p.is_uppercase() { Color::White }
        else                { Color::Black }
    }

    /// Determines the background color of the chessboard at a specific position.
    ///
    /// # Arguments
    ///
    /// - `rank`: The rank of the square (1-indexed).
    /// - `file`: The file (A=0) of the square (0-indexed).
    ///
    /// # Returns
    ///
    /// The `Color` of the board at the specified position. The color is represented by an RGB
    /// value. Light squares are represented by (r: 190, g: 140, b: 170), and dark squares are
    /// represented by (r: 255, g: 206, b: 158).
    #[rustfmt::skip]
    fn find_bkgnd(&self, rank: u8, file: u8) -> Color {
        let lght = Color::Rgb { r: 190, g: 140, b: 170 };
        let dark = Color::Rgb { r: 255, g: 206, b: 158 };
        if (rank + file) % 2 == 0 { dark }
        else                      { lght }
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print(true);
        let fen = self.to_string();
        write!(f, "FEN: {fen}")
    }
}

//use crate::board::movement;
impl fmt::Debug for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (piece_char, positions) in self.get_pieces() {
            let color = if piece_char.is_uppercase() {
                "White"
            } else {
                "Black"
            };
            let p_type = match piece_char.to_ascii_lowercase() {
                'p' => "Pawn",
                'n' => "Knight",
                'b' => "Bishop",
                'k' => "King",
                'q' => "Queen",
                'r' => "Rook",
                _ => "Unknown",
            };

            let squares = position::bitboard_squares(positions);
            let coords: Vec<String> = squares
                .iter()
                .map(|&square| position::square_to_rank_file(square))
                .map(|(rank, file)| format!("{file}{rank}"))
                .collect();

            writeln!(f, "{:<5} {:<7}: {:?}", color, p_type, coords)?;
        }
        write!(f, "{}", self.to_string())
    }
}
