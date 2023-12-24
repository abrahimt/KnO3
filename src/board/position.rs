// rank can be 1-8
pub fn square_to_rank(square: i64) -> u8 {
    ((square / 8) + 1) as u8
}

// Char can be A-H
pub fn square_to_file(square: i64) -> char {
    ((square % 8) as u8 + b'A') as char
}

/// Converts a square index (0-63) to its corresponding chess rank and file.
///
/// # Arguments
///
/// * `square` - The square index (0-63) on the chessboard.
///
/// # Returns
///
/// A tuple containing the corresponding file and rank for the given square.
///
/// # Example
///
/// ```
/// use kn_o3::board::position::square_to_rank_file;
/// let (file, rank) = square_to_rank_file(35);
/// println!("File: {}, Rank: {}", file, rank);
/// // Output: File: 'D', Rank: 5
/// ```
pub fn square_to_rank_file(square: i64) -> (char, usize) {
    let rank = square_to_rank(square);
    let file = square_to_file(square);
    (file, rank as usize)
}

/// Converts a chess rank and file to its corresponding square index (0-63).
///
/// # Arguments
///
/// * `rank` - The rank of the chessboard (1-8).
/// * `file` - The file of the chessboard (character 'A' to 'H').
///
/// # Returns
///
/// The result of the square index (0-63) corresponding to the given rank and file.
///
/// # Example
///
/// ```
/// use kn_o3::board::position::rank_file_to_square;
/// let square = rank_file_to_square(5, 'D').unwrap();
/// println!("Square: {square}");
/// // Output: Square: 35
/// ```
pub fn rank_file_to_square(rank: u8, file: char) -> Result<i64, String> {
    if !(1..=8).contains(&rank) {
        return Err("Invalid rank".to_string());
    }
    if !('A'..='H').contains(&file) {
        return Err("Invalid file".to_string());
    }
    Ok((rank - 1) as i64 * 8 + (file as u8 - b'A') as i64)
}

/// Converts a chess rank and file coordinate to its corresponding square index (0-63).
/// # Arguments
/// * `str` - The coordinate string (`A1` - `H8`)
/// # Returns
/// The result of the square index (0-63) corresponding to the given rank and file.
pub fn string_to_square(str: &str) -> Result<i64, String> {
    let chars: Vec<char> = str.chars().collect();
    if chars.len() != 2 {
        return Err("Invalid coordinate input".to_string());
    }

    let rank = chars[1].to_digit(10).unwrap_or(9);
    let file = chars[0];

    rank_file_to_square(rank as u8, file)
}

/// Find the squares turned on in this bitboard
///
/// # Arguments
///
/// * `bitboard` - The bitboard representing positions of a piece
///
/// # Returns
///
/// A vector containing the square indicies of where the bits are set
///
/// # Example
///
/// ```
/// use kn_o3::board::{Chessboard, position::bitboard_squares};
/// let cb = Chessboard::new();
/// let bitboard = cb.white_pawns;
/// let squares = bitboard_squares(bitboard);
/// ```
pub fn bitboard_squares(mut bitboard: u64) -> Vec<i64> {
    let mut squares = Vec::new();
    let mut index = 0;

    while bitboard != 0 {
        if bitboard & 1 != 0 {
            squares.push(index);
        }
        index += 1;
        bitboard >>= 1;
    }

    squares
}
