use std::ops::Index;
use crate::game::{Color, Piece};

#[derive(Debug)]
pub struct Game {
    pub board: Vec<Option<Piece>>
}

impl Game {
    pub fn new() -> Self {
        let mut b = Vec::new();
        for row  in 0..8 {
            for cell in 0..8 {
                match row {
                    0 => {
                        match cell {
                            0 => b.push(Some(Piece::Rook(0, Color::White))),
                            1 => b.push(Some(Piece::Knight(0, Color::White))),
                            2 => b.push(Some(Piece::Bishop(0, Color::White))),
                            3 => b.push(Some(Piece::Queen(0, Color::White))),
                            4 => b.push(Some(Piece::King(0, Color::White))),
                            5 => b.push(Some(Piece::Bishop(0, Color::White))),
                            6 => b.push(Some(Piece::Knight(0, Color::White))),
                            7 => b.push(Some(Piece::Rook(0, Color::White))),
                            _ => b.push(None)
                        }
                    },
                    1 => {
                        b.push(Some(Piece::Pawn(0, Color::White)))
                    }
                    6 => {
                        b.push(Some(Piece::Pawn(0, Color::Black)))
                    },
                    7 => {
                        match cell {
                            0 => b.push(Some(Piece::Rook(0, Color::Black))),
                            1 => b.push(Some(Piece::Knight(0, Color::Black))),
                            2 => b.push(Some(Piece::Bishop(0, Color::Black))),
                            3 => b.push(Some(Piece::Queen(0, Color::Black))),
                            4 => b.push(Some(Piece::King(0, Color::Black))),
                            5 => b.push(Some(Piece::Bishop(0, Color::Black))),
                            6 => b.push(Some(Piece::Knight(0, Color::Black))),
                            7 => b.push(Some(Piece::Rook(0, Color::Black))),
                            _ => b.push(None)
                        }
                    },                    
                    _ => b.push(None)
                }
            }

        }
        Self {
            board: b
        }
    }

    pub fn get_pos(&self, pos: (char, i32)) -> Option<&Piece> {
        let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let char_index = chars.iter().position(|f| f.eq(&pos.0)).expect("Invalid char");
        let row = (8 * pos.1 - 8) as usize;
        let board_index = char_index + row as usize;

        let to_return = self.board.get(board_index).and_then(|f|{f.as_ref()});
        to_return
    }
}
