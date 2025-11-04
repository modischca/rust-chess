use crate::errors::{GameErr, GameResult};
use crate::game::{Color, Piece};
use crate::ruleengine;

pub fn check(board: &[Option<Piece>; 64], from: (char, i32), to: (char, i32), current_player: Color) -> GameResult<i32> {
    let current_pos = ruleengine::get_index_based_on_pos(from);
    let end_pos = ruleengine::get_index_based_on_pos(to);

    let legal_moves: Vec<i8> = vec![
        7,8,9,
        -1,1
        -7,-8,-9
    ];

    if legal_moves.iter().find(|&i| current_pos as i8 + i == end_pos as i8).is_some() {
        if let Some(piece_at_pos) = ruleengine::get_piece_at_pos(&board, to) {
            if piece_at_pos.color != current_player {
                return Ok(piece_at_pos.get_points())
            }
        }
        return Ok(0);
    }
    Err(GameErr::IllegalKingMove)

}