#[cfg(test)]
mod tests {
    use crate::game::{Color, Game, Piece};
    use super::*;
    #[test]
    fn get_pos_a_1_is_rook() {
        let g = Game::new();
        let piece_at_pos = g.get_pos(('a', 1));
        if let Some(p) = piece_at_pos {
            assert_eq!(p.get_char_code(), '\u{2656}');
        } else {
            panic!("No piece at pos");
        }
    }
    #[test]
    fn get_pos_b_2_is_pawn() {
        let g = Game::new();
        let piece_at_pos = g.get_pos(('b', 2));
        if let Some(p) = piece_at_pos {
            assert_eq!(p.get_char_code(), '\u{2659}');
        } else {
            panic!("No piece at pos");
        }
    }

    #[test]
    fn get_pos_e_1_is_king() {
        let g = Game::new();
        let piece_at_pos = g.get_pos(('e', 1));
        if let Some(p) = piece_at_pos {
            assert_eq!(p.get_char_code(), '\u{2654}');
        } else {
            panic!("No piece at pos");
        }
    }

    #[test]
    fn get_pos_d_1_is_queen() {
        let g = Game::new();
        let piece_at_pos = g.get_pos(('d', 1));
        if let Some(p) = piece_at_pos {
            assert_eq!(p.get_char_code(), '\u{2655}');
        } else {
            panic!("No piece at pos");
        }
    }

    #[test]
    fn get_pos_b_3_is_empty() {
        let g = Game::new();
        let piece_at_pos = g.get_pos(('b', 3));
        assert!(piece_at_pos.is_none());
    }
}


