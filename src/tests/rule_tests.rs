use std::cmp::PartialEq;
use crate::errors::GameErr;
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
fn illegal_pawn_step_to_the_left() {
    let mut g = Game::new();
    let _ = &g.move_piece(('a', 2), ('a', 4)).unwrap(); // W
    let _ = &g.move_piece(('b', 7), ('b', 5)).unwrap(); // B
    let _ = &g.move_piece(('a', 4), ('a', 5)).unwrap(); // W
    assert_eq!(
        g.move_piece(('b', 5), ('a', 4)), // B
        Err(GameErr::IllegalMove("Illegal Pawn move".into()))
    );
}

#[test]
fn pawn_white_take_right() {
    let mut g = Game::new();
    let _ = &g.move_piece(('a', 2), ('a', 4)).unwrap(); // W
    let _ = &g.move_piece(('b', 7), ('b', 5)).unwrap(); // B
    let _ = &g.move_piece(('a', 4), ('b', 5)).unwrap(); // W
}

#[test]
fn pawn_black_take_right() {
    let mut g = Game::new();
    let _ = &g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    let _ = &g.move_piece(('d', 7), ('d', 5)).unwrap(); // B
    let _ = &g.move_piece(('d', 2), ('d', 4)).unwrap(); // W
    let _ = &g.move_piece(('d', 5), ('e', 4)).unwrap(); // B
}

#[test]
fn pawn_black_illegal_step_into_white() {
    let mut g = Game::new();
    let _ = &g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    let _ = &g.move_piece(('d', 7), ('d', 5)).unwrap(); // B
    let _ = &g.move_piece(('d', 2), ('d', 4)).unwrap(); // W
    assert_eq!(
        g.move_piece(('d', 5), ('d', 4)), // B
        Err(GameErr::IllegalMove("Illegal Pawn move".into()))
    );
}
#[test]
fn pawn_black_illegal_jump_white() {
    let mut g = Game::new();
    let _ = &g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    let _ = &g.move_piece(('d', 7), ('d', 5)).unwrap(); // B
    let _ = &g.move_piece(('d', 2), ('d', 4)).unwrap(); // W
    assert_eq!(
        g.move_piece(('d', 5), ('d', 3)), // B
        Err(GameErr::IllegalMove("Illegal Pawn move".into()))
    );
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
fn rook_test_horizontal() {
    let mut g = Game::new();
    let _ = &g.move_piece(('a', 2), ('a', 4)).unwrap(); // W
    let _ = &g.move_piece(('b', 7), ('b', 5)).unwrap(); // B
    let _ = &g.move_piece(('a', 4), ('a', 5)).unwrap(); // W

    let _ = &g.move_piece(('b', 5), ('b', 4)).unwrap(); // B
    assert_eq!(
        g.move_piece(('a', 1), ('a', 5)),
        Err(GameErr::IllegalMove("Position is occupied.".into())) //W
    );
    let _ = &g.move_piece(('a', 1), ('a', 4)).unwrap(); // W
    let _ = &g.move_piece(('d', 7), ('d', 5)).unwrap(); // B
    assert_eq!(
        g.move_piece(('a', 4), ('d', 4)), // W
        Err(GameErr::IllegalMove("Illegal slide".into()))
    );
    let _ = &g.move_piece(('a', 4), ('b', 4)).unwrap(); // W
    assert_eq!(g.score_white, 1);
    let _ = &g.move_piece(('c', 7), ('c', 5)).unwrap(); // B
    let _ = &g.move_piece(('b', 4), ('d', 4)).unwrap(); // W
    let _ = &g.move_piece(('c', 5), ('c', 4)).unwrap(); // B

    assert_eq!(
        g.move_piece(('d', 4), ('b', 4)), // W
        Err(GameErr::IllegalMove("Illegal slide".into()))
    );
    let _ = &g.move_piece(('d', 4), ('c', 4)).unwrap(); // W
    assert_eq!(g.score_white, 2);
    let _ = &g.move_piece(('f', 7), ('f', 5)).unwrap(); // B
    let _ = &g.move_piece(('c', 4), ('c', 7)).unwrap(); // W
    let _ = &g.move_piece(('f', 5), ('f', 4)).unwrap(); // B
    assert_eq!(
        g.move_piece(('c', 7), ('g', 7)), // W
        Err(GameErr::IllegalMove("Illegal slide".into()))
    );
    let _ = &g.move_piece(('c', 7), ('a', 7)).unwrap(); // W
    assert_eq!(g.score_white, 3);
    let _ = &g.move_piece(('d', 5), ('d', 4)).unwrap(); // B
    assert_eq!(
        g.move_piece(('a', 7), ('a', 5)), // W
        Err(GameErr::IllegalMove("Position is occupied.".into()))
    );

    let _ = &g.move_piece(('a', 7), ('a', 8)).unwrap(); // W
    assert_eq!(g.score_white, 8);

}

#[test]
fn rook_test_vertical() {
    let mut g = Game::new();
    let _ = &g.move_piece(('a', 2), ('a', 4)).unwrap(); // W
    let _ = &g.move_piece(('b', 7), ('b', 5)).unwrap(); // B
    let _ = &g.move_piece(('a', 4), ('b', 5)).unwrap(); // W
    assert_eq!(g.score_white, 1);
    let _ = &g.move_piece(('a', 7), ('a', 6)).unwrap(); // B
    assert_eq!(
        g.move_piece(('a', 1), ('a', 7)), // W
        Err(GameErr::IllegalMove("Illegal slide".into()))
    );
    let _ = &g.move_piece(('a', 1), ('a', 6)).unwrap(); // W
    assert_eq!(g.score_white, 2);
    let _ = &g.move_piece(('c', 7), ('c', 6)).unwrap(); // B
    let _ = &g.move_piece(('a', 6), ('a', 8)).unwrap(); // W
    assert_eq!(g.score_white, 7);
    let _ = &g.move_piece(('b', 8), ('a', 6)).unwrap(); // B
    assert_eq!(
        g.move_piece(('a', 8), ('a', 5)), // W
        Err(GameErr::IllegalMove("Illegal slide".into()))
    );
    let _ = &g.move_piece(('a', 8), ('a', 6)).unwrap(); // W
    assert_eq!(g.score_white, 10);

}
#[test]
fn move_pawn_only_one_square_is_legal() {
    let mut g = Game::new();
    assert!(&g.move_piece(('e', 2), ('e', 5)).is_err());
}
