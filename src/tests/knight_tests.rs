use crate::errors::GameErr;
use crate::game::Game;

#[test]
fn illegal_knight_move_does_not_change_turn() {
    let mut g = Game::new();

    // 1. White tries an illegal knight move (not an L-shape).
    assert_eq!(
        g.move_piece(('g', 1), ('g', 3)), // vertical two → illegal
        Err(GameErr::IllegalKnightMove)
    );

    // Still White's turn after the illegal move.
    g.move_piece(('g', 1), ('f', 3)).unwrap(); // W (legal knight L)

    // 1... Black responds to confirm turn flow.
    g.move_piece(('h', 7), ('h', 6)).unwrap(); // B
}