
#[derive(Debug,Clone, Copy,PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn display(&self) -> String {
        match self {
            Color::White => "WHITE".into(),
            Color::Black => "BLACK".into(),
        }
    }
}

