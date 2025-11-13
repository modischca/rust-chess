mod ruleset_pawn;
mod ruleset_rook;
mod ruleset_knight;
mod ruleset_bishop;
mod ruleset_king;

use crate::game::errors::{GameErr, GameResult};
use crate::game::*;
use crate::ruleengine;
pub fn get_piece_at_pos(board: &[Option<Piece>; 64], pos: (char, i32)) -> Option<&Piece> {
    let board_index = get_index_based_on_pos(pos);
    let to_return = board.get(board_index).and_then(|f| { f.as_ref() });
    to_return
}

pub fn get_piece_at_index(board: &[Option<Piece>; 64], index: usize) -> Option<&Piece> {
    let to_return = board.get(index).and_then(|f|{f.as_ref()});
    to_return
}

pub fn get_index_based_on_pos(pos: (char, i32)) -> usize {
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let char_index = chars.iter().position(|f| f.eq(&pos.0)).expect("Invalid char");
    let row = (8 * pos.1 - 8) as usize;
    char_index + row as usize
}

pub fn is_allowed_move(game: &Game, from: (char, i32), to: (char, i32), current_player: Color) -> GameResult<i32> {

    let piece_from = ruleengine::get_piece_at_pos(&game.board, from).expect("No piece at position.");

    let piece_to = ruleengine::get_piece_at_pos(&game.board, to);

    // Wrong color is moving
    if piece_from.color != current_player {
        Err(GameErr::IllegalMoveOnOtherPlayer)?
    }

    // No move registered. Start and end positions are equal.
    if from == to {
        return Err(GameErr::NoMoveRegistered);
    }

    // Check that the position is empty, or that it is not occupied by the same color.
    if let Some(piece_to) = piece_to {
        if (piece_from.color ==  piece_to.color) {
            return Err(GameErr::PositionOccupied);
        }
    }
    
    match piece_from.piece_type {
        PieceType::Pawn => {
            ruleset_pawn::check(&game.board, from, to, current_player)
        },
        PieceType::Rook => {
            ruleset_rook::check(&game.board, from, to, current_player)
        },
        PieceType::Knight => {
            ruleset_knight::check(&game.board, from ,to, current_player)
        },
        PieceType::Bishop => {
            ruleset_bishop::check(&game.board, from ,to, current_player)
        }
        PieceType::Queen => {
            ruleset_rook::check(&game.board, from, to, current_player)
                .or_else(|_| {
                    ruleset_bishop::check(&game.board, from, to, current_player)
                        .map_err(|e| match e {
                            GameErr::IllegalBishopMove => GameErr::IllegalQueenMove,
                            _ => e,
                        })
                })
        }
        PieceType::King => {
            ruleset_king::check(&game.board, from, to, current_player)
        }
    }

}

pub fn is_castling_move(game: &Game, from: (char, i32), to: (char, i32), current_player: Color) -> &str {
    if (from.0 == 'e' && to.0 == 'g' && (from.1 == 1 && to.1 == from.1) && game.white_can_castle.contains("K") && current_player == Color::White) {
        return "K".as_ref()
    }
    if (from.0 == 'e' && to.0 == 'g' && (from.1 == 1 && to.1 == from.1) && game.white_can_castle.contains("Q") && current_player == Color::White) {
        return "Q".as_ref()
    }
    if (from.0 == 'e' && to.0 == 'g' && (from.1 == 8 && to.1 == from.1) && game.black_can_castle.contains("k") && current_player == Color::Black) {
        return "k".as_ref()
    }
    if (from.0 == 'e' && to.0 == 'g' && (from.1 == 8 && to.1 == from.1) && game.black_can_castle.contains("q") && current_player == Color::Black) {
        return "q".as_ref()
    }

    "-"
}