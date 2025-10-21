mod ruleset_pawn;
mod ruleset_rook;

use crate::errors::{GameErr, GameResult};
use crate::game::*;
use crate::rule_engine;

pub fn get_piece_at_pos(board: &Vec<Option<Piece>>, pos: (char, i32)) -> Option<&Piece> {
    let board_index = get_index_based_on_pos(pos);
    let to_return = board.get(board_index).and_then(|f|{f.as_ref()});
    to_return
}

pub fn get_index_based_on_pos(pos: (char, i32)) -> usize {
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let char_index = chars.iter().position(|f| f.eq(&pos.0)).expect("Invalid char");
    let row = (8 * pos.1 - 8) as usize;
    char_index + row as usize
}


pub fn is_allowed_move(board: &Vec<Option<Piece>>, from: (char, i32), to: (char, i32), current_player: Color) -> GameResult<i32> {

    let piece_from = rule_engine::get_piece_at_pos(&board,from).expect("No piece at position.");

    let piece_to = rule_engine::get_piece_at_pos(&board,to);

    // Check that the position is empty, or that it is not occupied by the same color.
    if let Some(piece_to) = piece_to {
        if (piece_from.color ==  piece_to.color) {
            return Err(GameErr::IllegalMove("Position is occupied.".into()));
        }
    }
    
    match piece_from.piece_type {
        PieceType::Pawn => {
            ruleset_pawn::check(board.clone(), from, to, current_player)
        },
        PieceType::Rook => {
            ruleset_rook::check(board.clone(), from, to, current_player)
        },
        _ => {
            Ok(0)
        }
    }

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
