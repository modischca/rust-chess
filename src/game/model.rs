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
    pub board: Vec<Option<Piece>>,
    pub current_player: Color,
    pub score_white: i32,
    pub score_black: i32,
}
impl Game {
    pub fn new() -> Self {
        let mut b = Vec::new();
        for row  in 0..8 {
            for col in 0..8 {
                match (row, col) {
                    (0,0) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::Rook })),
                    (0,1) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::Knight })),
                    (0,2) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::Bishop })),
                    (0,3) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::Queen })),
                    (0,4) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::King })),
                    (0,5) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::Bishop })),
                    (0,6) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::Knight })),
                    (0,7) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::Rook })),
                    (1, 0..8) => b.push(Some(Piece { color: Color::White, piece_type: PieceType::Pawn })),
                    (6, 0..8) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::Pawn })), // kept color as in your original
                    (7,0) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::Rook })),
                    (7,1) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::Knight })),
                    (7,2) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::Bishop })),
                    (7,3) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::Queen })),
                    (7,4) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::King })),
                    (7,5) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::Bishop })),
                    (7,6) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::Knight })),
                    (7,7) => b.push(Some(Piece { color: Color::Black, piece_type: PieceType::Rook })),
                    _ => b.push(None),
                }
            }

        }
        Self {
            board: b,
            current_player: Color::White,
            score_white: 0,
            score_black: 0,
        }
    }

    pub fn move_piece(&mut self, from: (char, i32), to: (char, i32)) -> GameResult<()> {

        let piece = match rule_engine::get_piece_at_pos(&self.board,from) {
            Some(p) => p.clone(),                // requires Piece: Clone
            None => return Err(GameErr::IllegalMove("No piece at position.".into())),
        };

        if piece.color != self.current_player {
            Err(GameErr::IllegalMove("You can only play your own pieces.".into()))?
        }

        if from == to {
            return Err(GameErr::IllegalMove("No move registered.".into()));
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
