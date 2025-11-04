use crate::errors::GameErr;
use crate::game::{Color, Game, PieceType};
use crate::ruleengine;
#[test]
fn bishop_non_diagonal_illegal_requires_redo_same_turn() {
    let mut g = Game::new();

    // 1. White tries a non-diagonal bishop move (illegal).
    assert_eq!(
        g.move_piece(('f', 1), ('f', 3)),
        Err(GameErr::IllegalBishopMove)
    );

    // Still White's turn — redo with a legal White move.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // Black responds to confirm turn flow.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B
}

#[test]
fn bishop_destination_occupied_errors_then_succeeds_after_freeing_square() {
    let mut g = Game::new();

    // 1. f1 → e2 is onto White's own pawn at e2 → destination occupied.
    assert_eq!(
        g.move_piece(('f', 1), ('e', 2)),
        Err(GameErr::PositionOccupied)
    );

    // Still White's turn — redo: free the destination square first.
    g.move_piece(('e', 2), ('e', 3)).unwrap(); // W

    // Black reply to keep turn order consistent.
    g.move_piece(('e', 7), ('e', 6)).unwrap(); // B

    // Now the bishop move is legal.
    g.move_piece(('f', 1), ('e', 2)).unwrap(); // W
}

#[test]
fn bishop_path_blocked_errors_then_works_after_pawn_moves() {
    let mut g = Game::new();

    // 1. c1 → g5 is blocked by the pawn on d2.
    assert_eq!(
        g.move_piece(('c', 1), ('g', 5)),
        Err(GameErr::PathIsBlocked)
    );

    // Still White's turn — redo: clear the diagonal first (d2 → d3).
    g.move_piece(('d', 2), ('d', 3)).unwrap(); // W

    // Black reply.
    g.move_piece(('a', 7), ('a', 6)).unwrap(); // B

    // Path c1–d2–e3–f4–g5 is now clear → legal move.
    g.move_piece(('c', 1), ('g', 5)).unwrap(); // W
}

#[test]
fn bishop_up_left_blocked_by_b2_then_legal_after_clearing() {
    let mut g = Game::new();

    // 1. c1 → a3 is blocked by pawn on b2.
    assert_eq!(
        g.move_piece(('c', 1), ('a', 3)),
        Err(GameErr::PathIsBlocked)
    );

    // Still White's turn — redo: move the blocker.
    g.move_piece(('b', 2), ('b', 3)).unwrap(); // W

    // Black reply.
    g.move_piece(('h', 7), ('h', 6)).unwrap(); // B

    // Now diagonal is open.
    g.move_piece(('c', 1), ('a', 3)).unwrap(); // W
}
#[test]
fn bishop_blocked_path_requires_redo_same_turn() {
    let mut g = Game::new();

    // 1. White bishop on c1 tries to move diagonally to f4.
    //    However, the pawn on d2 blocks its path.
    assert_eq!(
        g.move_piece(('c', 1), ('f', 4)),
        Err(GameErr::PathIsBlocked)
    );

    // Still White's turn after the blocked-path error.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // Black responds to confirm normal turn flow.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B
}
