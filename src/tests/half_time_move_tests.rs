use crate::{Game};
use crate::game::errors::GameErr;

#[test]
fn halfmove_clock_starts_at_zero() {
    let g = Game::new();
    assert_eq!(g.half_time_moves, 0);
}

#[test]
fn halfmove_clock_increments_on_quiet_non_pawn_moves() {
    let mut g = Game::new();

    g.move_piece(('g', 1), ('f', 3)).unwrap(); // Nf3
    assert_eq!(g.half_time_moves, 1);

    g.move_piece(('b', 8), ('c', 6)).unwrap(); // Nc6
    assert_eq!(g.half_time_moves, 2);

    g.move_piece(('e', 2), ('e', 4)).unwrap(); // e4 frees the bishop, resets to 0
    assert_eq!(g.half_time_moves, 0);

    g.move_piece(('g', 8), ('f', 6)).unwrap(); // Nf6 (quiet move)
    assert_eq!(g.half_time_moves, 1);

    g.move_piece(('f', 1), ('b', 5)).unwrap(); // Bb5 (quiet move, now possible)
    assert_eq!(g.half_time_moves, 2);
}


#[test]
fn halfmove_clock_resets_on_pawn_move() {
    let mut g = Game::new();

    // 1. Nf3 (quiet move, non-pawn)
    g.move_piece(('g', 1), ('f', 3)).unwrap();
    assert_eq!(g.half_time_moves, 1);

    // ... Nc6 (quiet move, non-pawn)
    g.move_piece(('b', 8), ('c', 6)).unwrap();
    assert_eq!(g.half_time_moves, 2);

    // 2. d4 (pawn move, should reset clock to 0)
    g.move_piece(('d', 2), ('d', 4)).unwrap();
    assert_eq!(g.half_time_moves, 0);

    // ... d6 (another pawn move, stays at 0 because it resets again)
    g.move_piece(('d', 7), ('d', 6)).unwrap();
    assert_eq!(g.half_time_moves, 0);
}

#[test]
fn halfmove_clock_resets_on_capture() {
    let mut g = Game::new();

    // 1. e4 (pawn move, reset to 0)
    g.move_piece(('e', 2), ('e', 4)).unwrap();
    assert_eq!(g.half_time_moves, 0);

    // ... Nf6 (quiet non-pawn move, increments to 1)
    g.move_piece(('g', 8), ('f', 6)).unwrap();
    assert_eq!(g.half_time_moves, 1);

    // 2. e5 (pawn move, resets to 0)
    g.move_piece(('e', 4), ('e', 5)).unwrap();
    assert_eq!(g.half_time_moves, 0);

    // ... Nd5 (quiet non-pawn move, increments to 1)
    g.move_piece(('f', 6), ('d', 5)).unwrap();
    assert_eq!(g.half_time_moves, 1);

    // 3. c4 (pawn move, resets to 0)
    g.move_piece(('c', 2), ('c', 4)).unwrap();
    assert_eq!(g.half_time_moves, 0);

    // ... Nb6 (quiet non-pawn move, increments to 1)
    g.move_piece(('d', 5), ('b', 6)).unwrap();
    assert_eq!(g.half_time_moves, 1);

    // 4. c5 (pawn move, resets to 0)
    g.move_piece(('c', 4), ('c', 5)).unwrap();
    assert_eq!(g.half_time_moves, 0);

    // ... Nd5 (quiet non-pawn move, increments to 1)
    g.move_piece(('b', 6), ('d', 5)).unwrap();
    assert_eq!(g.half_time_moves, 1);

    // 5. Nc3 (quiet non-pawn move, increments to 2)
    g.move_piece(('b', 1), ('c', 3)).unwrap();
    assert_eq!(g.half_time_moves, 2);

    // ... Nxc3 (capture, should reset to 0)
    g.move_piece(('d', 5), ('c', 3)).unwrap();
    assert_eq!(g.half_time_moves, 0);
}

#[test]
fn halfmove_clock_unchanged_on_illegal_or_blocked_move() {
    let mut g = Game::new();

    // Initial value.
    assert_eq!(g.half_time_moves, 0);

    // Bishop on c1 tries to go to f4 but is blocked by the pawn on d2.
    let res = g.move_piece(('c', 1), ('f', 4));
    assert_eq!(res, Err(GameErr::PathIsBlocked));

    // Clock must not change on a failed / rejected move.
    assert_eq!(g.half_time_moves, 0);

    // Now make a normal pawn move; this should reset to 0 (it already is 0).
    g.move_piece(('e', 2), ('e', 4)).unwrap();
    assert_eq!(g.half_time_moves, 0);
}

#[test]
fn halfmove_clock_mixed_sequence_example() {
    let mut g = Game::new();

    // 1. Nf3 (quiet non-pawn) -> 1
    g.move_piece(('g', 1), ('f', 3)).unwrap();
    assert_eq!(g.half_time_moves, 1);

    // ... d5 (pawn move) -> reset to 0
    g.move_piece(('d', 7), ('d', 5)).unwrap();
    assert_eq!(g.half_time_moves, 0);

    // 2. g3 (pawn move) -> reset to 0
    g.move_piece(('g', 2), ('g', 3)).unwrap();
    assert_eq!(g.half_time_moves, 0);

    // ... Nf6 (quiet non-pawn) -> 1
    g.move_piece(('g', 8), ('f', 6)).unwrap();
    assert_eq!(g.half_time_moves, 1);

    // 3. Bg2 (quiet non-pawn) -> 2
    g.move_piece(('f', 1), ('g', 2)).unwrap();
    assert_eq!(g.half_time_moves, 2);

    // ... Bh3 (quiet non-pawn) -> 3
    g.move_piece(('c', 8), ('h', 3)).unwrap();
    assert_eq!(g.half_time_moves, 3);

    // 4. Bxh3 (capture) -> reset to 0
    g.move_piece(('g', 2), ('h', 3)).unwrap();
    assert_eq!(g.half_time_moves, 0);
}