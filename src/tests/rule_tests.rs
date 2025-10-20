use std::cmp::PartialEq;
use crate::game::{Game, Piece, PieceType};
use crate::rule_engine;
#[test]
fn move_pawn_e2_to_e4_is_legal() {
    let mut g = Game::new();
    let _ = &g.move_piece(('e', 2), ('e', 4)).unwrap();
    assert!(rule_engine::get_piece_at_pos(&g.board, ('e', 4)).is_some());
    assert!(rule_engine::get_piece_at_pos(&g.board,('e', 2)).is_none());
    assert_eq!(rule_engine::get_piece_at_pos(&g.board,('e', 4)).unwrap().piece_type.clone(), PieceType::Pawn);
}

#[test]
fn move_pawn_e2_to_e3_is_legal() {
    let mut g = Game::new();
    let _ = &g.move_piece(('e', 2), ('e', 3)).unwrap();
    assert!(rule_engine::get_piece_at_pos(&g.board,('e', 3)).is_some());
    assert!(rule_engine::get_piece_at_pos(&g.board,('e', 2)).is_none());
    assert_eq!(rule_engine::get_piece_at_pos(&g.board,('e', 3)).unwrap().piece_type.clone(), PieceType::Pawn);
}


#[test]
fn move_pawn_only_one_square_is_legal() {
    let mut g = Game::new();
    assert!(&g.move_piece(('e', 2), ('e', 5)).is_err());
}
