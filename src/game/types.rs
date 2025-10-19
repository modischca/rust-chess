
#[derive(Debug,Clone, Copy)]
pub enum Piece {
    Pawn(i32, Color),
    Knight(i32, Color),
    Bishop(i32, Color),
    Rook(i32, Color),
    Queen(i32, Color),
    King(i32, Color),
}
#[allow(unused)]

impl Piece {
    pub fn get_char_code(&self) -> char {
        match self {
            Piece::Pawn(x, y) => '\u{2659}',
            Piece::Knight(x, y) => '\u{2658}',
            Piece::Bishop(x, y) => '\u{2657}',
            Piece::Rook(x, y) => '\u{2656}',
            Piece::Queen(x, y) => '\u{2655}',
            Piece::King(x, y) => '\u{2654}',
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

