use std::cmp::PartialEq;
use crate::errors::{GameErr, GameResult};
use crate::game::{Color, Game, Piece};
use crate::ruleengine;
use crate::ruleengine::{get_piece_at_index, get_piece_at_pos};

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

        let up_right_positions = step(index, Direction::UpRight);
        let found_up_right = up_right_positions.iter().find(|&i| i.eq(&target_index));
        if found_up_right.is_some() && !is_path_blocked_up(board, up_right_positions, target_index)? {
            return check_score_and_return(board, to, current_player);
        }

        let up_left_positions = step(index, Direction::UpLeft);
        let found_up_left = up_left_positions.iter().find(|&i| i.eq(&target_index));
        if found_up_left.is_some() && !is_path_blocked_up(board, up_left_positions, target_index)? {
            return check_score_and_return(board, to, current_player);
        }

        let down_right_positions = step(index, Direction::DownRight);
        let found_down_right = down_right_positions.iter().find(|&i| i.eq(&target_index));
        if found_down_right.is_some() && !is_path_blocked_down(board, down_right_positions, target_index)? {
            return check_score_and_return(board, to, current_player);
        }

        let down_left_positions = step(index, Direction::DownRight);
        let found_down_left = down_left_positions.iter().find(|&i| i.eq(&target_index));
        if found_down_left.is_some() && !is_path_blocked_down(board, down_left_positions, target_index)? {
            return check_score_and_return(board, to, current_player);
        }

        Err(GameErr::IllegalBishopMove)
}
fn is_path_blocked_up(board: [Option<Piece>; 64], selection: Vec<i32>, target_index: i32) -> Result<bool, GameErr> {
    if selection.iter().find(|&i| ruleengine::get_piece_at_index(&board, i.clone() as usize).is_some() && i < &target_index).is_some() {
        return Err(GameErr::PathIsBlocked);
    }
    return Ok(false);
}
fn is_path_blocked_down(board: [Option<Piece>; 64], selection: Vec<i32>, target_index: i32) -> Result<bool, GameErr> {
    if selection.iter().find(|&i| ruleengine::get_piece_at_index(&board, i.clone() as usize).is_some() && i > &target_index).is_some() {
        return Err(GameErr::PathIsBlocked);
    }
    return Ok(false);
}

fn check_score_and_return (board: [Option<Piece>; 64], to: (char, i32), current_player: Color) -> GameResult<i32>{
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
