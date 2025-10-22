use std::cmp::PartialEq;
use std::ops::Index;
use crate::errors::{GameErr, GameResult};
use crate::game::{Color, PieceType};
use crate::rule_engine;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType
}


impl Piece {
    pub fn get_char_code(&self) -> char {
        match self.piece_type {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }

    pub fn get_points(&self) -> i32 {
        match self.piece_type {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 100,
        }
    }
}
#[derive(Debug)]
pub struct Game {
    pub board: [Option<Piece>; 64],
    pub current_player: Color,
    pub score_white: i32,
    pub score_black: i32,
}
impl Game {
    pub fn new() -> Self {
        let board :[Option<Piece>; 64] = std::array::from_fn(|i| {
            match i {
                0 => Some(Piece { color: Color::White, piece_type: PieceType::Rook }),
                1 => Some(Piece { color: Color::White, piece_type: PieceType::Knight }),
                2 => Some(Piece { color: Color::White, piece_type: PieceType::Bishop }),
                3 => Some(Piece { color: Color::White, piece_type: PieceType::Queen }),
                4 => Some(Piece { color: Color::White, piece_type: PieceType::King }),
                5 => Some(Piece { color: Color::White, piece_type: PieceType::Bishop }),
                6 => Some(Piece { color: Color::White, piece_type: PieceType::Knight }),
                7 => Some(Piece { color: Color::White, piece_type: PieceType::Rook }),
                8..=15 => Some(Piece { color: Color::White, piece_type: PieceType::Pawn }),
                48..=55 => Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }),
                56 => Some(Piece { color: Color::Black, piece_type: PieceType::Rook }),
                57 => Some(Piece { color: Color::Black, piece_type: PieceType::Knight }),
                58 => Some(Piece { color: Color::Black, piece_type: PieceType::Bishop }),
                59 => Some(Piece { color: Color::Black, piece_type: PieceType::Queen }),
                60 => Some(Piece { color: Color::Black, piece_type: PieceType::King }),
                61 => Some(Piece { color: Color::Black, piece_type: PieceType::Bishop }),
                62 => Some(Piece { color: Color::Black, piece_type: PieceType::Knight }),
                63 => Some(Piece { color: Color::Black, piece_type: PieceType::Rook }),
                _ => None,
            }
        });
        Self {
            board,
            current_player: Color::White,
            score_white: 0,
            score_black: 0,
        }
    }

    pub fn move_piece(&mut self, from: (char, i32), to: (char, i32)) -> GameResult<()> {

        let piece = match rule_engine::get_piece_at_pos(&self.board,from) {
            Some(p) => p.clone(),                // requires Piece: Clone
            None => return Err(GameErr::NoPieceAtPosition),
        };

        if piece.color != self.current_player {
            Err(GameErr::IllegalMoveOnOtherPlayer)?
        }

        if from == to {
            return Err(GameErr::NoMoveRegistered);
        }

        let points = rule_engine::is_allowed_move(&self.board, from, to, self.current_player)?;

        let from_idx = rule_engine::get_index_based_on_pos(from);
        self.board[from_idx] = None;

        let to_idx = rule_engine::get_index_based_on_pos(to);
        self.board[to_idx] = Some(piece);

        self.current_player = match self.current_player {
            Color::White => {
                self.score_white += points;
                Color::Black
            },
            Color::Black => {
                self.score_black += points;
                Color::White
            },
        };
        Ok(())
    }


}
