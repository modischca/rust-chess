use crate::game::errors::{GameErr, GameResult};
use crate::game::Color;
use crate::ruleengine;
use crate::game::Piece;
pub fn check(board: &[Option<Piece>; 64], from: (char, i32), to: (char, i32), current_player: Color) -> GameResult<i32> {
    let piece_from = ruleengine::get_piece_at_pos(&board, from);
    let piece_to = ruleengine::get_piece_at_pos(&board, to);

    if to.1 - from.1 == 1 && current_player == Color::White && piece_to.is_none() {
        if from.0 == to.0 {
            return Ok(0)
        }
    }
    if to.1 - from.1 == -1 && current_player == Color::Black && piece_to.is_none() {
        if from.0 == to.0 {
            return Ok(0)
        }
    }
    if (from.1 == 2 && to.1 == 4) || (from.1 == 7 && to.1 == 5) {
        return Ok(0)
    }
    if let Some(target) = piece_to
        && (to.1 - from.1).abs() == 1
        && (to.0 as i32 - from.0 as i32 == 1 || to.0 as i32 - from.0 as i32 == -1)
    {
        if let Some(piece_from) = piece_from {
            if (target.color != piece_from.color) {
                return Ok(target.get_points());
            }
        }
    }
    Err(GameErr::IllegalPawnMove)
}