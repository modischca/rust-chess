use crate::game::Game;

/// At start, both sides can castle both sides.
#[test]
fn both_sides_start_with_full_castling_rights() {
    let g = Game::new();
    assert_eq!(g.white_can_castle, "KQ");
    assert_eq!(g.black_can_castle, "kq");
}

/// Moving the white king (from e1) removes both K and Q for White.
#[test]
fn white_king_move_removes_both_rights() {
    let mut g = Game::new();

    // Free e2 so the king can step to e2; also maintain turn order.
    g.move_piece(('e', 2), ('e', 3)).unwrap(); // White
    g.move_piece(('a', 7), ('a', 6)).unwrap(); // Black (any legal)
    g.move_piece(('e', 1), ('e', 2)).unwrap(); // White king moves

    assert_eq!(g.white_can_castle, "");
    assert_eq!(g.black_can_castle, "kq");
}

/// Moving the white rook from a1 removes the queenside ('Q') right only.
#[test]
fn white_rook_a1_move_removes_queenside_castling_right() {
    let mut g = Game::new();

    // Clear a2 and respect alternating turns.
    g.move_piece(('a', 2), ('a', 3)).unwrap(); // White: free a2
    g.move_piece(('h', 7), ('h', 6)).unwrap(); // Black: any legal
    g.move_piece(('a', 1), ('a', 2)).unwrap(); // White: rook a1→a2

    assert_eq!(g.white_can_castle, "K"); // lost Q-side
    assert_eq!(g.black_can_castle, "kq");
}

/// Moving the white rook from h1 removes the kingside ('K') right only.
#[test]
fn white_rook_h1_move_removes_kingside_castling_right() {
    let mut g = Game::new();

    // Clear h2 and respect alternating turns.
    g.move_piece(('h', 2), ('h', 3)).unwrap(); // White: free h2
    g.move_piece(('a', 7), ('a', 6)).unwrap(); // Black: any legal
    g.move_piece(('h', 1), ('h', 2)).unwrap(); // White: rook h1→h2

    assert_eq!(g.white_can_castle, "Q"); // lost K-side
    assert_eq!(g.black_can_castle, "kq");
}

/// Moving the black king (from e8) removes both k and q for Black.
#[test]
fn black_king_move_removes_both_rights() {
    let mut g = Game::new();

    // Let Black move the pawn on e7 so the king can step to e7.
    g.move_piece(('a', 2), ('a', 3)).unwrap(); // White: any legal
    g.move_piece(('e', 7), ('e', 6)).unwrap(); // Black: free e7
    g.move_piece(('h', 2), ('h', 3)).unwrap(); // White: any legal
    g.move_piece(('e', 8), ('e', 7)).unwrap(); // Black king moves

    assert_eq!(g.white_can_castle, "KQ");
    assert_eq!(g.black_can_castle, "");
}

/// Moving the black rook from a8 removes the queenside ('q') right only.
#[test]
fn black_rook_a8_move_removes_queenside_castling_right() {
    let mut g = Game::new();

    // Clear a7 and alternate turns.
    g.move_piece(('h', 2), ('h', 3)).unwrap(); // White: any legal
    g.move_piece(('a', 7), ('a', 6)).unwrap(); // Black: free a7
    g.move_piece(('g', 2), ('g', 3)).unwrap(); // White: any legal
    g.move_piece(('a', 8), ('a', 7)).unwrap(); // Black: rook a8→a7

    assert_eq!(g.black_can_castle, "k"); // lost q-side
    assert_eq!(g.white_can_castle, "KQ");
}

/// Moving the black rook from h8 removes the kingside ('k') right only.
#[test]
fn black_rook_h8_move_removes_kingside_castling_right() {
    let mut g = Game::new();

    // Clear h7 and alternate turns.
    g.move_piece(('a', 2), ('a', 3)).unwrap(); // White: any legal
    g.move_piece(('h', 7), ('h', 6)).unwrap(); // Black: free h7
    g.move_piece(('b', 2), ('b', 3)).unwrap(); // White: any legal
    g.move_piece(('h', 8), ('h', 7)).unwrap(); // Black: rook h8→h7

    assert_eq!(g.black_can_castle, "q"); // lost k-side
    assert_eq!(g.white_can_castle, "KQ");
}

/// Castling rights never "come back" even if a rook returns to its original square.
#[test]
fn rights_do_not_return_if_rook_moves_back() {
    let mut g = Game::new();

    // Clear a2 and move rook out, then back.
    g.move_piece(('a', 2), ('a', 3)).unwrap(); // White
    g.move_piece(('h', 7), ('h', 6)).unwrap(); // Black
    g.move_piece(('a', 1), ('a', 2)).unwrap(); // White: rook leaves a1 (lose 'Q')
    g.move_piece(('a', 7), ('a', 5)).unwrap(); // Black: any legal
    g.move_piece(('a', 2), ('a', 1)).unwrap(); // White: rook returns

    assert_eq!(g.white_can_castle, "K"); // still no 'Q'
    assert_eq!(g.black_can_castle, "kq");
}
