use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;
use crate::errors::{GameErr, GameResult};
use crate::game::{Color, PieceType};
use crate::ruleengine;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType
}

impl Piece {
    pub fn get_char_code(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => 'P',
            (PieceType::Knight, Color::White) => 'N',
            (PieceType::Bishop, Color::White) => 'B',
            (PieceType::Rook, Color::White) => 'R',
            (PieceType::Queen, Color::White) => 'Q',
            (PieceType::King, Color::White) => 'K',

            (PieceType::Pawn, Color::Black) => 'p',
            (PieceType::Knight, Color::Black) => 'n',
            (PieceType::Bishop, Color::Black) => 'b',
            (PieceType::Rook, Color::Black) => 'r',
            (PieceType::Queen, Color::Black) => 'q',
            (PieceType::King, Color::Black) => 'k',
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
    pub fen: String,
    pub current_player: Color,
    pub score_white: i32,
    pub score_black: i32,
    pub white_can_castle: String,
    pub black_can_castle: String,
    moves: u8
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
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into(),
            current_player: Color::White,
            score_white: 0,
            score_black: 0,
            white_can_castle: "KQ".into(),
            black_can_castle: "kq".into(),
            moves: 1,
        }
    }


    pub fn move_piece(&mut self, from: (char, i32), to: (char, i32)) -> GameResult<()> {
        let piece = match ruleengine::get_piece_at_pos(&self.board, from) {
            Some(p) => p.clone(),                // requires Piece: Clone
            None => return Err(GameErr::NoPieceAtPosition),
        };

        // Check if the move is allowed
        // Will cast GameErr if not allowed.
        let points = ruleengine::is_allowed_move(&self.board, from, to, self.current_player)?;

        // Clear square
        let from_idx = ruleengine::get_index_based_on_pos(from);
        self.board[from_idx] = None;

        // Move piece to new square
        let to_idx = ruleengine::get_index_based_on_pos(to);
        self.board[to_idx] = Some(piece);

        // Can player still castle? Has King or Rook moved?
        if piece.piece_type == PieceType::King {
            if self.current_player == Color::Black {
                self.black_can_castle = "".to_string()
            } else {
                self.white_can_castle = "".to_string()
            }
        }
        if piece.piece_type == PieceType::Rook {
            if from.0 < 'e' {
                if self.current_player == Color::White {
                    self.white_can_castle = self.white_can_castle.replace("Q", "");
                } else {
                    self.black_can_castle = self.black_can_castle.replace("q", "");
                }
            } else {
                if self.current_player == Color::White {
                    self.white_can_castle = self.white_can_castle.replace("K", "");
                } else {
                    self.black_can_castle = self.black_can_castle.replace("k", "");
                }
            }
        }

        // Set current player
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

        // Increase move counter
        self.moves += 1;

        // Update FEN
        self.fen = get_fen(self);
        // Return Ok
        Ok(())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str = String::from("");
        let mut board_rows: [[char; 8]; 8] = [[' '; 8];8];
        let mut row_index = 0;
        let mut col_index = 0;
        for i in (0..self.board.len()).rev(){

            if let Some(p) = self.board[i] {
                board_rows[row_index][col_index] = p.get_char_code();

            } else {
                board_rows[row_index][col_index] = '\u{25A1}';
            }
            if col_index == 7 {
                board_rows[row_index].reverse();
                row_index += 1;
                col_index = 0;
            } else {
                col_index += 1;
            }

        }
        str.push_str("\n\r");
        str.push_str(&format!("Score WHITE {} \n", self.score_white));
        str.push_str(&format!("Score BLACK {} \n", self.score_black));
        str.push_str(&format!("FEN {} \n", self.fen));
        str.push_str("\n\r");
        for (z, row) in board_rows.iter().enumerate() {
            for (i, col) in row.iter().enumerate() {
                if i == 0 {
                    str.push_str(&format!("{}    {} " ,(8 - z), col.to_string().as_str()));
                } else {
                    str.push_str(&format!(" {} " ,col.to_string().as_str()));
                }

            }
            str.push('\n');
        }
        str.push_str("\n\r");
        str.push_str("     a  b  c  d  e  f  g  h \n");

        write!(f, "{}", str)
    }
}

fn get_fen(game: &Game) -> String{
    let mut fen = String::from("");
    for rank in (0..8).rev() { // 8→1 top-down
        for file in 0..8 {
            let index = rank * 8 + file;
            let empty_squares = 1;
            match game.board[index] {
                Some(piece) => fen.push(piece.get_char_code()),
                None => {
                    fen.push_str(("1"))
                }
            }
        }
        fen.push_str("/");
    }
    fen.remove(fen.len() - 1);
    fen = compress_ones(fen);
    let mut color = 'w';
    if (game.current_player == Color::Black) {
        color = 'b';
    }
    let mut castle_rights = String::from("");
    castle_rights.push_str(game.white_can_castle.as_str());
    castle_rights.push_str(game.black_can_castle.as_str());

    if (castle_rights.len() == 0) {
        castle_rights = "-".to_string();
    }
    fen.push(' ');
    fen.push(color.into());
    fen.push(' ');
    fen.push_str(castle_rights.as_str());
    fen.push(' ');
    fen.push('-');
    fen.push(' ');
    fen.push('0');
    fen.push(' ');
    fen.push_str(game.moves.to_string().as_str()); // Number of black moves
    fen
}


pub fn compress_ones(f: String) -> String {
    let mut result = String::new();
    let mut count = 0;

    for c in f.chars() {
        if c == '1' {
            count += 1;
        } else {
            if count > 0 {
                result.push_str(&count.to_string());
                count = 0;
            }
            result.push(c);
        }
    }

    // flush last run of 1s
    if count > 0 {
        result.push_str(&count.to_string());
    }

    result
}
