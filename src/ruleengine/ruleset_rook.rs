use crate::errors::{GameErr, GameResult};
use crate::game::{Color, Piece};
use crate::ruleengine::{get_piece_at_pos};

pub fn check(board: &[Option<Piece>; 64], from: (char, i32), to: (char, i32), current_player: Color) -> GameResult<i32> {

    if from.1 != to.1 && from.0 != to.0 {
        return Err(GameErr::IllegalRookMove)
    }
    if is_sliding_move(&from, &to) {
        let is_vertical_slide = from.1 != to.1 && from.0 == to.0;

        let mut next_check_pos = from.0 as i32 + 1;
        let mut stop_position = to.0 as i32;
        if from.0 as i32 > to.0 as i32 {
            stop_position = from.0 as i32;
            next_check_pos = to.0 as i32 + 1;
        }

        if is_vertical_slide {
            next_check_pos = from.1 + 1;
            stop_position = to.1 ;
            if from.1 > to.1 {
                stop_position = from.1;
                next_check_pos = to.1 + 1;
            }
        }
        while next_check_pos < stop_position {
            let mut c = from.0;
            let mut row = next_check_pos;
            if !is_vertical_slide {
                c = char::from_u32(next_check_pos as u32).expect("Invalid char");
                row = from.1;
            }
            if get_piece_at_pos(&board, (c, row)) != None {
                return Err(GameErr::PathIsBlocked)
            }
            next_check_pos = next_check_pos + 1;
        }
    }
    if let Some(piece_at_target) = get_piece_at_pos(&board, to) {
        if piece_at_target.color != current_player {
            return Ok(piece_at_target.get_points());
        }
    }
    Ok(0)
}

pub fn is_sliding_move(from: &(char, i32), to: &(char, i32)) -> bool {
    if (to.0 as i32 - from.0 as i32).abs() > 1 && to.1 == from.1 {
        return true;
    }
    if (from.0 as i32 - to.0 as i32).abs() > 1 && to.1 == from.1 {
        return true;
    }

    if (from.1 - to.1).abs() > 1 && to.0 == from.0 {
        return true;
    }

    false
}
