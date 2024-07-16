/// A square is a 0-63 number

pub fn square_to_rank(square: u8) -> u8 {
    (square / 8) + 1
}

pub fn square_to_file(square: u8) -> char {
    ((square % 8) + b'A') as char
}

pub fn square_to_rank_file(square: u8) -> (u8, char) {
    let rank = square_to_rank(square);
    let file = square_to_file(square);
    (rank, file)
}

pub fn rank_file_to_square(rank: u8, file: char) -> Result<u8, String> {
    if !(1..=8).contains(&rank)     { return Err(format!("Invalid rank {rank}")); }
    if !('A'..='H').contains(&file) { return Err(format!("Invalid file {file}")); }
    Ok((rank - 1) * 8 + (file as u8 - b'A'))
}

pub fn square_to_string(square: u8) -> String {
    let (rank, file) = square_to_rank_file(square);
    format!("{}{}", file, rank)
}

/// Coordinate string `A1` - `H8`
pub fn string_to_square(coord: &str) -> Result<u8, String> {
    let chars: Vec<char> = coord.chars().collect();
    if chars.len() != 2 { return Err(format!("Invalid coordinate input: {coord}")); }

    let rank = chars[1].to_digit(10).expect("Invalid rank") as u8;
    let file = chars[0];
    Ok(rank_file_to_square(rank, file)?)
}

/// Find the squares turned on in this bitboard
pub fn active_squares(mut bitboard: i64) -> Vec<u8> {
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
