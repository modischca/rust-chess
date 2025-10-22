use log::debug;
use crate::errors::{GameErr, GameResult};
use crate::game::{Color, Piece};
use crate::rule_engine;

pub fn check(board: [Option<Piece>; 64], from: (char, i32), to: (char, i32), current_player: Color) -> GameResult<i32> {
    let index = rule_engine::get_index_based_on_pos(from) as i32;
    let legal_pos: [i32; 8] = [
        step(index, 2, true, false),
        step(index, 2, true, true),
        step(index, 2, false, false),
        step(index, 2, false, true),
        step(index, 1, true, false),
        step(index, 1, true, true),
        step(index, 1, false, false),
        step(index, 1, false, true),
    ];
    let target = rule_engine::get_index_based_on_pos(to) as i32;
    let is_valid = legal_pos.contains(&target);
    if !is_valid {
        return Err(GameErr::IllegalKnightMove);
    }
    if let Some(at_target) = rule_engine::get_piece_at_pos(&board, to) {
        if at_target.color != current_player {
            return Ok(at_target.get_points());
        }
    }
    Ok(0)
}
fn two_plus_one(index: i32, direction_up: bool, direction_left: bool) -> i32{
    step(index, 2, direction_up, direction_left)
}

fn one_plus_two(index: i32, direction_up: bool, direction_left: bool) -> i32{
    step(index, 1, direction_up, direction_left)
}

fn step(index: i32, step: i32, direction_up: bool, direction_left: bool) -> i32{
    let steps = step;
    let index_moves = 8 * steps;
    let start_pos = index ;
    let start_row =  (index / 8) + 1;
    let mut last = if direction_left { - 1} else {1};

    if steps == 1 {
        last = if direction_left { - 2} else {2};
    }
    let direction_val = if direction_up {  index_moves} else {-index_moves};
    let end_pos = start_pos + (direction_val) + (last);
    let end_row = (end_pos / 8) + 1;

    if (end_pos < 0) || (end_pos > 63) {
        return -1;
    }

    if step == 2 {
        if direction_up && end_row - start_row != 2 {
            return -1;
        }

        if !direction_up && start_row - end_row != 2 {
            return -1;
        }
    } else {
        if direction_up && end_row - start_row != 1 {
            return -1;
        }

        if !direction_up && start_row - end_row != 1 {
            return -1;
        }
    }

    end_pos
}


#[test]
fn test_two_plus_one() {

    // UP 2
    let index = rule_engine::get_index_based_on_pos(('b', 1)) as i32;
    assert_ne!(two_plus_one(index, true, false), -1);

    let index = rule_engine::get_index_based_on_pos(('a', 3)) as i32;
    assert_ne!(two_plus_one(index, true, false), -1);

    let index = rule_engine::get_index_based_on_pos(('b', 1)) as i32;
    assert_ne!(two_plus_one(index, true, true), -1);

    let index = rule_engine::get_index_based_on_pos(('a', 3)) as i32;
    assert_eq!(two_plus_one(index, true, true), -1);

    let index = rule_engine::get_index_based_on_pos(('g', 1)) as i32;
    assert_ne!(two_plus_one(index, true, false), -1);

    let index = rule_engine::get_index_based_on_pos(('h', 3)) as i32;
    assert_eq!(two_plus_one(index, true, false), -1);

    // DOWN 2

    let index = rule_engine::get_index_based_on_pos(('b', 1)) as i32;
    assert_eq!(two_plus_one(index, false, false), -1);

    let index = rule_engine::get_index_based_on_pos(('b', 1)) as i32;
    assert_eq!(two_plus_one(index, false, false), -1);

    let index = rule_engine::get_index_based_on_pos(('b', 8)) as i32;
    assert_ne!(two_plus_one(index, false, false), -1);

    let index = rule_engine::get_index_based_on_pos(('b', 8)) as i32;
    assert_ne!(two_plus_one(index, false, true), -1);

    let index = rule_engine::get_index_based_on_pos(('b', 8)) as i32;
    assert_eq!(two_plus_one(index, true, false), -1);

    let index = rule_engine::get_index_based_on_pos(('b', 8)) as i32;
    assert_eq!(two_plus_one(index, true, true), -1);


    let index = rule_engine::get_index_based_on_pos(('c', 3)) as i32;
    assert_ne!(one_plus_two(index, true, false), -1);


    let index = rule_engine::get_index_based_on_pos(('a', 2)) as i32;
    assert_eq!(one_plus_two(index, true, true), -1);

}
