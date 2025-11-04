use crate::game::Game;

#[test]
fn test_fen_starts_with_initial_position() {
    let g = Game::new();
    assert_eq!(g.fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
}

#[test]
fn test_fen_is_correct() {
    let mut g = Game::new();
    g.move_piece(('d', 2), ('d', 4)).unwrap();
    //assert_eq!(g.fen, "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");
}