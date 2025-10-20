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

    let piece_from = rule_engine::get_piece_at_pos(&board,from);
    let piece_to = rule_engine::get_piece_at_pos(&board,to);
    if (from.0 == to.0) && (from.1 == to.1) {
        return Err(GameErr::IllegalMove("No move registered.".into()));
    }
    if let Some(piece_from) = piece_from {
        // Check that the user is moving his piece.
        if piece_from.color != current_player {
            Err(GameErr::IllegalMove("You can only play your own pieces.".into()))?
        }
        // Check that the position is empty, or that it is not occupied by the same color.
        if let Some(piece_to) = piece_to {
            if (piece_from.color ==  piece_to.color) {
                return Err(GameErr::IllegalMove("Position is occupied.".into()));
            }
        }
        // PAWN CHECK
        if piece_from.piece_type == PieceType::Pawn {
            if to.1 - from.1 == 1 && current_player == Color::White && piece_to.is_none() {
                return Ok(0)
            }
            if to.1 - from.1 == -1 && current_player == Color::Black && piece_to.is_none() {
                return Ok(0)
            }
            if (from.1 == 2 && to.1 == 4) || (from.1 == 7 && to.1 == 5) {
                return Ok(0)
            }
            if let Some(target) =
                rule_engine::get_piece_at_pos(&board, (to.0, to.1))
                && (to.1 - from.1).abs() == 1
                && (to.0 as i32 - from.0 as i32 == 1 || to.0 as i32 - from.0 as i32 == -1)
            {
                if (target.color != piece_from.color) {
                    return Ok(target.get_points());
                }
            }
            return Err(GameErr::IllegalMove("Illegal Pawn move".into()));
        }
        // Knight Check
        if (piece_from.piece_type == PieceType::Rook) {

            if (from.0 != to.0 && from.1 != to.1) {
                return Err(GameErr::IllegalMove("Illegal Rook move".into()));
            }
            let board_ = board.clone();

            // Move on same column
            if from.0 == to.0 {
                // Sliding forward
                if (to.1 - from.1).abs() != 1 && to.1 > from.1 {
                    for i in (from.1 + 1)..to.1 {
                        if get_piece_at_pos(&board_, (from.0, i)).is_some() {
                            return Err(GameErr::IllegalMove("Can not jump over other pieces 1".into()));
                        }
                    }
                }
                // Sliding backward
                if (from.1 - to.1).abs() != 1 && from.1 < to.1 {
                    for i in (to.1..from.1).rev() {
                        if get_piece_at_pos(&board_, (from.0, i)).is_some() {
                            return Err(GameErr::IllegalMove("Can not jump over other pieces 2".into()));
                        }
                    }
                }
            }

            // Move on same row

            // Sliding Right

            // Sliding Left


            if let Some(piece_to) = get_piece_at_pos(&board_, to) {
                return Ok(piece_to.get_points());
            } else {
                return Ok(0);
            }
            return Err(GameErr::IllegalMove("Illegal Rook move".into()));
            // TODO: Implement Knight Check
        }
    }
    Ok(0)
}