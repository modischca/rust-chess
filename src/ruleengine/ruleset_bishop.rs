use std::cmp::PartialEq;
use crate::errors::{GameErr, GameResult};
use crate::game::{Color, Game, Piece};
use crate::ruleengine;
use crate::ruleengine::get_piece_at_pos;

#[derive(PartialEq)]
enum Direction {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}
pub fn check(board: [Option<Piece>; 64], from: (char, i32), to: (char, i32), current_player: Color) -> GameResult<i32> {
    let index = ruleengine::get_index_based_on_pos(from) as i32;
    let target_index = ruleengine::get_index_based_on_pos(to) as i32;

    let mut allowed_positions = step(index, Direction::UpRight);
    for &i in allowed_positions.iter() {
        if (&i < &target_index) {
            if &i != &target_index  && ruleengine::get_piece_at_index(&board, i as usize).is_some()  {
                return Err(GameErr::IllegalBishopMove);
            }
        }
    }

    allowed_positions.extend(step(index, Direction::UpLeft));
    for &i in allowed_positions.iter() {
        if (&i < &target_index) {
            if &i != &target_index  && ruleengine::get_piece_at_index(&board, i as usize).is_some()  {
                return Err(GameErr::IllegalBishopMove);
            }
        }
    }
    allowed_positions.extend(step(index, Direction::DownRight));
    for &i in allowed_positions.iter() {
        if (&i > &target_index) {
            if &i != &target_index  && ruleengine::get_piece_at_index(&board, i as usize).is_some()  {
                return Err(GameErr::IllegalBishopMove);
            }
        }
    }
    allowed_positions.extend(step(index, Direction::DownLeft));
    for &i in allowed_positions.iter() {
        if (&i > &target_index) {
            if &i != &target_index  && ruleengine::get_piece_at_index(&board, i as usize).is_some()  {
                return Err(GameErr::IllegalBishopMove);
            }
        }
    }

    if let Some(piece_at_pos) = get_piece_at_pos(&board, to) {
        if piece_at_pos.color == current_player {
            return Err(GameErr::IllegalBishopMove);
        }
        return Ok(piece_at_pos.get_points());
    }
    Ok(0)

}



fn step(index: i32, direction: Direction) -> Vec<i32> {
    let mut index = index;
    let mut to_return = Vec::new();
    let add = match direction {
        Direction::UpRight => 9,
        Direction::UpLeft => 7,
        Direction::DownRight => -7,
        Direction::DownLeft => -9,
    };
    loop {
        let file = index % 8;     // 0 = 'a', 7 = 'h'
        let rank = index / 8;     // 0 = rank 1, 7 = rank 8
        // For up-left
        if file == 0 || rank == 7{
            // can't move further up-left
            return to_return;
        }

        index += add;

        if index > 0{
            to_return.push(index);
        }

    }
}
