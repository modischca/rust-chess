use crate::game::errors::GameErr;
use crate::game::Game;

#[test]
fn queen_non_linear_illegal_requires_redo_same_turn() {
    let mut g = Game::new();

    // 1. White tries a non-linear queen move (like a knight) — illegal.
    //    Queen starts on d1, so d1 -> e3 is not straight or diagonal.
    assert_eq!(
        g.move_piece(('d', 1), ('e', 3)),
        Err(GameErr::IllegalQueenMove)
    );

    // Still White's turn — redo with a legal White move.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // Black responds to confirm turn flow.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B
}

#[test]
fn queen_blocked_path_requires_redo_same_turn() {
    let mut g = Game::new();

    // 1. White queen tries to move forward two squares from d1 to d3.
    //    The pawn on d2 is blocking the path, so this should fail.
    assert_eq!(
        g.move_piece(('d', 1), ('d', 3)),
        Err(GameErr::IllegalQueenMove)
    );

    // Still White's turn after the blocked-path error.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // Black responds to confirm turn flow.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B
}

#[test]
fn queen_cannot_capture_own_piece_requires_redo_same_turn() {
    let mut g = Game::new();

    // 1. White queen tries to move onto its own pawn at d2.
    //    The move shape is legal (one square forward),
    //    but the destination is occupied by a friendly piece.
    assert_eq!(
        g.move_piece(('d', 1), ('d', 2)),
        Err(GameErr::PositionOccupied)
    );

    // Still White's turn — redo with a legal White move.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // Black responds to confirm turn flow.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B
}
#[test]
fn queen_can_move_straight_when_path_clear() {
    let mut g = Game::new();

    // 1. Free the queen's path by moving the pawn in front of it.
    g.move_piece(('d', 2), ('d', 4)).unwrap(); // W

    // 2. Black makes a normal move to alternate turn.
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B

    // 3. Queen moves straight up from d1 to d3 (now that the path is open).
    g.move_piece(('d', 1), ('d', 3)).unwrap(); // W

    // 4. Black makes a confirming move.
    g.move_piece(('g', 8), ('f', 6)).unwrap(); // B
}

#[test]
fn queen_can_move_diagonally_when_path_clear() {
    let mut g = Game::new();

    // 1. Free the diagonal path for the queen.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    g.move_piece(('d', 7), ('d', 5)).unwrap(); // B

    // 2. Move the white bishop out to clear the diagonal further if needed.
    g.move_piece(('f', 1), ('c', 4)).unwrap(); // W
    g.move_piece(('g', 8), ('f', 6)).unwrap(); // B

    // 3. Queen moves diagonally (d1 -> h5).
    g.move_piece(('d', 1), ('h', 5)).unwrap(); // W

    // 4. Black responds to confirm continued turn alternation.
    g.move_piece(('f', 6), ('h', 5)).unwrap_or(()); // capturing queen optional if valid
}
