use crate::game::errors::GameErr;
use crate::game::Game;

#[test]
fn king_non_adjacent_illegal_requires_redo_same_turn() {
    let mut g = Game::new();

    // 1. White king tries a non-adjacent, knight-like move — illegal.
    //    From e1 to f3 is more than one square away.
    assert_eq!(
        g.move_piece(('e', 1), ('f', 3)),
        Err(GameErr::IllegalKingMove)
    );

    // Still White's turn — redo with a legal White move.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // Black responds to confirm turn flow.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B
}

#[test]
fn king_cannot_move_onto_own_piece_requires_redo_same_turn() {
    let mut g = Game::new();

    // 1. White king tries to move from e1 to e2.
    //    e2 is occupied by a white pawn in the initial position.
    assert_eq!(
        g.move_piece(('e', 1), ('e', 2)),
        Err(GameErr::PositionOccupied)
    );

    // Still White's turn after the illegal move.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // Black responds to confirm normal turn alternation.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B
}

#[test]
fn king_can_move_one_square_legally() {
    let mut g = Game::new();

    // 1. Free a square for the king to move into.
    //    Move the pawn from e2 to e4, vacating e2.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // 2. Black makes a normal move.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B

    // 3. King moves one square forward from e1 to e2 (now empty) — legal king move.
    g.move_piece(('e', 1), ('e', 2)).unwrap(); // W

    // 4. Black responds to confirm continued turn alternation.
    g.move_piece(('g', 8), ('f', 6)).unwrap(); // B
}

#[test]
fn king_can_move_one_square_diagonally_when_target_is_free() {
    let mut g = Game::new();

    // 1. Free the diagonal target square f2 by moving that pawn.
    g.move_piece(('f', 2), ('f', 4)).unwrap(); // W

    // 2. Black makes a normal move.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B

    // 3. King moves one square diagonally from e1 to f2 — legal.
    g.move_piece(('e', 1), ('f', 2)).unwrap(); // W

    // 4. Black responds to confirm turn flow.
    g.move_piece(('g', 8), ('f', 6)).unwrap(); // B
}

#[test]
fn king_illegal_two_square_vertical_to_empty_square_requires_redo_same_turn() {
    let mut g = Game::new();

    // In the initial position, e3 is empty.
    // But a king cannot move two squares vertically from e1 to e3.
    assert_eq!(
        g.move_piece(('e', 1), ('e', 3)),
        Err(GameErr::IllegalKingMove)
    );

    // Still White's turn after the illegal move.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // Black responds to confirm normal turn alternation.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B
}

#[test]
fn king_illegal_two_square_diagonal_to_empty_square_requires_redo_same_turn() {
    let mut g = Game::new();

    // In the initial position, c3 is empty.
    // From e1 to c3 is a two-square diagonal move, which is illegal for a king.
    assert_eq!(
        g.move_piece(('e', 1), ('c', 3)),
        Err(GameErr::IllegalKingMove)
    );

    // Still White's turn after the illegal move.
    g.move_piece(('d', 2), ('d', 4)).unwrap(); // W

    // Black responds to confirm normal turn alternation.
    g.move_piece(('d', 7), ('d', 5)).unwrap(); // B
}
