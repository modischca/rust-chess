
use crate::game::{Color, Game, PieceType};
use crate::rule_engine;
use super::*;
#[test]
fn get_pos_a_1_is_white_rook() {
    let g = Game::new();
    let piece_at_pos = rule_engine::get_piece_at_pos(&g.board,('a', 1));
    assert_eq!(piece_at_pos.unwrap().color, Color::White);
    assert_eq!(piece_at_pos.unwrap().piece_type, PieceType::Rook);
}

#[test]
fn get_pos_a_2_is_white_pawn() {
    let g = Game::new();
    let piece_at_pos = rule_engine::get_piece_at_pos(&g.board,('a', 2));
    assert_eq!(piece_at_pos.unwrap().color, Color::White);
    assert_eq!(piece_at_pos.unwrap().piece_type, PieceType::Pawn);
}

#[test]
fn get_pos_a_7_is_black_pawn() {
    let g = Game::new();
    let piece_at_pos = rule_engine::get_piece_at_pos(&g.board,('a', 7));
    assert_eq!(piece_at_pos.unwrap().color, Color::Black);
    assert_eq!(piece_at_pos.unwrap().piece_type, PieceType::Pawn);
}

#[test]
fn get_pos_a_6_is_empty() {
    let g = Game::new();
    let piece_at_pos = rule_engine::get_piece_at_pos(&g.board,('a', 6));
    assert!(piece_at_pos.is_none());
}

#[test]
fn get_pos_b_3_is_empty() {
    let g = Game::new();
    let piece_at_pos = rule_engine::get_piece_at_pos(&g.board,('b', 3));
    assert!(piece_at_pos.is_none());
}



