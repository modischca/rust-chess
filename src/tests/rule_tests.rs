use std::cmp::PartialEq;
use crate::errors::GameErr;
use crate::game::{Game, Piece, PieceType};



// 1) Single step forward is legal.
#[test]
fn pawn_single_step_forward_is_legal() {
    let mut g = Game::new();
    g.move_piece(('e', 2), ('e', 3)).unwrap(); // W
}

// 2) Double step from starting position is legal.
#[test]
fn pawn_double_step_from_start_is_legal() {
    let mut g = Game::new();
    g.move_piece(('d', 2), ('d', 4)).unwrap(); // W
}

// 3) Triple step forward is illegal.
#[test]
fn pawn_triple_step_is_illegal() {
    let mut g = Game::new();
    assert_eq!(
        g.move_piece(('e', 2), ('e', 5)),
        Err(GameErr::IllegalPawnMove)
    );
}

// 4) Backward move is illegal.
#[test]
fn pawn_backward_move_is_illegal() {
    let mut g = Game::new();
    // First, move forward legally so we can attempt backward movement
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    g.move_piece(('a', 7), ('a', 6)).unwrap(); // B

    assert_eq!(
        g.move_piece(('e', 4), ('e', 3)),
        Err(GameErr::IllegalPawnMove)
    );
}

// 5) Attempting to capture forward (no diagonal) is illegal.
#[test]
fn pawn_cannot_capture_forward() {
    let mut g = Game::new();
    // White pawn forward
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    // Black pawn moves in front
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B

    // White tries to capture forward — not allowed
    assert_eq!(
        g.move_piece(('e', 4), ('e', 5)),
        Err(GameErr::IllegalPawnMove)
    );
}

// 6) Diagonal capture is legal.
#[test]
fn pawn_diagonal_capture_is_legal() {
    let mut g = Game::new();
    // Setup a capture on d5
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    g.move_piece(('d', 7), ('d', 5)).unwrap(); // B

    // White captures diagonally: e4 → d5
    g.move_piece(('e', 4), ('d', 5)).unwrap(); // W
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
        Err(GameErr::PathIsBlocked)
    );
    let _ = &g.move_piece(('a', 1), ('a', 6)).unwrap(); // W
    assert_eq!(g.score_white, 2);
    let _ = &g.move_piece(('c', 7), ('c', 6)).unwrap(); // B
    let _ = &g.move_piece(('a', 6), ('a', 8)).unwrap(); // W
    assert_eq!(g.score_white, 7);
    let _ = &g.move_piece(('b', 8), ('a', 6)).unwrap(); // B
    assert_eq!(
        g.move_piece(('a', 8), ('a', 5)), // W
        Err(GameErr::PathIsBlocked)
    );
    let _ = &g.move_piece(('a', 8), ('a', 6)).unwrap(); // W
    assert_eq!(g.score_white, 10);

}
#[test]
fn move_pawn_only_one_square_is_legal() {
    let mut g = Game::new();
    assert!(&g.move_piece(('e', 2), ('e', 5)).is_err());
}


#[test]
fn illegal_move_does_not_change_turn() {
    let mut g = Game::new();

    // 1. White makes a legal move.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W

    // 1... Black attempts an illegal pawn move (diagonal without capture).
    assert_eq!(
        g.move_piece(('d', 7), ('c', 6)), // B
        Err(GameErr::IllegalPawnMove)
    );

    // Because the move was illegal, it's STILL Black's turn.
    // This legal Black move must now succeed (test will fail if turn incorrectly switched).
    g.move_piece(('d', 7), ('d', 5)).unwrap(); // B

    // Now the turn goes back to White after a legal Black move.
    g.move_piece(('e', 4), ('e', 5)).unwrap(); // W
}

#[test]
fn illegal_rook_move_does_not_change_turn() {
    let mut g = Game::new();

    // 1. White moves pawn to free the rook.
    g.move_piece(('a', 2), ('a', 4)).unwrap(); // W

    // 1... Black makes a legal move.
    g.move_piece(('h', 7), ('h', 5)).unwrap(); // B

    // 2. White tries an illegal rook diagonal move.
    assert_eq!(
        g.move_piece(('a', 1), ('b', 2)), // diagonal → illegal
        Err(GameErr::PositionOccupied)
    );

    // Because that was illegal, it's STILL White's turn.
    // This legal vertical rook move must now succeed.
    g.move_piece(('a', 1), ('a', 3)).unwrap(); // W

    // 2... Black moves again to verify turn flow.
    g.move_piece(('h', 5), ('h', 4)).unwrap(); // B
}

#[test]
fn illegal_rook_move_then_legal_backward_move() {
    let mut g = Game::new();

    // 1. White clears the rook’s path.
    g.move_piece(('a', 2), ('a', 4)).unwrap(); // W

    // 1... Black makes a legal move.
    g.move_piece(('h', 7), ('h', 5)).unwrap(); // B

    // 2. White tries an illegal diagonal rook move.
    assert_eq!(
        g.move_piece(('a', 1), ('b', 2)), // diagonal → illegal
        Err(GameErr::PositionOccupied)
    );

    // Because that was illegal, it's STILL White's turn.
    // First, make a legal forward rook move.
    g.move_piece(('a', 1), ('a', 3)).unwrap(); // W

    // 2... Black moves again.
    g.move_piece(('h', 5), ('h', 4)).unwrap(); // B

    // 3. Now White moves the rook *backwards*, which is legal.
    g.move_piece(('a', 3), ('a', 1)).unwrap(); // W

    // 3... Black makes another move to confirm turn order is fine.
    g.move_piece(('h', 4), ('h', 3)).unwrap(); // B
}


// 1) Rook: diagonal to an EMPTY square → "Illegal diagonal rook move."
#[test]
fn rook_diagonal_to_empty_square_is_illegal() {
    let mut g = Game::new();

    // Clear a2 and b2 so that a1→b2 is empty and path for later a1→a3 is clear.
    g.move_piece(('a', 2), ('a', 4)).unwrap(); // W
    g.move_piece(('h', 7), ('h', 5)).unwrap(); // B
    g.move_piece(('b', 2), ('b', 4)).unwrap(); // W
    g.move_piece(('h', 5), ('h', 4)).unwrap(); // B

    // White tries a diagonal rook move into empty b2 → should be illegal diagonal.
    assert_eq!(
        g.move_piece(('a', 1), ('b', 2)),
        Err(GameErr::IllegalRookMove)
    );

    // Turn should NOT change; White can still move now.
    g.move_piece(('a', 1), ('a', 3)).unwrap(); // legal vertical
}

// 2) Rook: diagonal where the target is occupied by a FRIENDLY piece → "Position is occupied."
#[test]
fn rook_diagonal_to_friendly_occupied_square_reports_occupied() {
    let mut g = Game::new();

    // Keep b2 occupied by the white pawn; do a quick Black move to pass the turn back.
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    g.move_piece(('e', 7), ('e', 5)).unwrap(); // B

    // White tries a1→b2 (diagonal) but b2 has a white pawn → occupied
    assert_eq!(
        g.move_piece(('a', 1), ('b', 2)),
        Err(GameErr::PositionOccupied)
    );

    // Still White's turn after the illegal move; confirm with a legal move.
    // First clear a2 so rook can go up.
    g.move_piece(('a', 2), ('a', 4)).unwrap(); // W (legal)
}

// 3) Pawn: diagonal WITHOUT capture → "Illegal pawn move."
#[test]
fn pawn_illegal_diagonal_without_capture_is_rejected() {
    let mut g = Game::new();

    // White tries to move c2→d3 with no capture available → illegal pawn move.
    assert_eq!(
        g.move_piece(('c', 2), ('d', 3)),
        Err(GameErr::IllegalPawnMove)
    );

    // Still White's turn after the illegal move; a legal move should work.
    g.move_piece(('c', 2), ('c', 4)).unwrap(); // W
}

// 4) Pawn: diagonal WITH capture (contrast) → should be OK (no error).
#[test]
fn pawn_diagonal_with_capture_is_legal() {
    let mut g = Game::new();

    // Create a capture on d5: e2→e4, d7→d5, e4xd5
    g.move_piece(('e', 2), ('e', 4)).unwrap(); // W
    g.move_piece(('d', 7), ('d', 5)).unwrap(); // B

    // White captures diagonally: e4→d5 (legal)
    g.move_piece(('e', 4), ('d', 5)).unwrap(); // W
}

#[test]
fn white_rook_cannot_capture_white_piece() {
    let mut g = Game::new();

    // 1. White clears rook's path by moving the pawn in front of it forward one step.
    g.move_piece(('a', 2), ('a', 3)).unwrap(); // W

    // 1... Black makes a legal move to pass the turn.
    g.move_piece(('h', 7), ('h', 5)).unwrap(); // B

    // 2. White tries to capture its own pawn with the rook.
    assert_eq!(
        g.move_piece(('a', 1), ('a', 3)), // White rook tries to capture white pawn
        Err(GameErr::PositionOccupied)
    );

    // Still White's turn, so a legal move should now succeed (rook moves elsewhere).
    g.move_piece(('a', 1), ('a', 2)).unwrap(); // W (legal move onto now-empty square)
}
