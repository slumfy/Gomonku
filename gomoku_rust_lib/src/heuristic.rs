//! Heuristic of our AI.

use crate::check_move::check_free_development;
use crate::check_pos_still_win;
use crate::checking_and_apply_bits_move;
use crate::global_var;
use crate::state::State;

#[derive(Debug)]
pub struct BoardStateInfo {
    pub is_wrong_move: i8,
    pub stone_captured: i8,
    pub capturable: bool,
    pub capturing: bool,
    pub pattern_value: i32,
    pub is_winning: (usize, i8),
    pub nb_move_to_win: i8,
}

pub fn heuristic(state: &mut State) -> i32 {
    let mut value: i32 = 0;
    let board_state_info: BoardStateInfo = checking_and_apply_bits_move(state);
    if is_playable_move(state, &board_state_info) == false {
        return value;
    }
    value = is_in_winning_pos(state, &board_state_info);
    if value == global_var::HEURISTIC_MAX_VALUE || value == global_var::HEURISTIC_MIN_VALUE {
        return value;
    }
    value += assign_pattern_value_to_state(&board_state_info);
    if value == global_var::HEURISTIC_MAX_VALUE {
        return value;
    }
    value += assign_capturing_pos_value_to_state(&board_state_info);
    value += check_free_development(state);
    value += assign_capture_value_to_state(state, &board_state_info);
    return value;
}

fn is_playable_move(state: &mut State, board_state_info: &BoardStateInfo) -> bool {
    if board_state_info.is_wrong_move != global_var::VALID_MOVE {
        state.is_playable = board_state_info.is_wrong_move;
        return false;
    }
    return true;
}

fn is_in_winning_pos(state: &mut State, board_state_info: &BoardStateInfo) -> i32 {
    let mut value: i32 = 0;
    if state.win_state.1 != 0 {
        if check_pos_still_win(state.bitboards, state.win_state.0, state.win_state.1) == true {
            if state.current_player == state.win_state.1 {
                value = global_var::HEURISTIC_MAX_VALUE;
            } else {
                value = global_var::HEURISTIC_MIN_VALUE;
            }
            return value;
        } else {
            state.win_state = (0, 0);
        }
    }
    if board_state_info.is_winning.1 != 0 {
        state.win_state = board_state_info.is_winning;
    }
    return value;
}

fn assign_pattern_value_to_state(board_state_info: &BoardStateInfo) -> i32 {
    let mut value: i32 = 0;
    if board_state_info.pattern_value == global_var::HEURISTIC_MAX_VALUE {
        value = global_var::HEURISTIC_MAX_VALUE;
        return value;
    }
    value += board_state_info.pattern_value as i32;
    return value;
}

fn assign_capturing_pos_value_to_state(board_state_info: &BoardStateInfo) -> i32 {
    let mut value: i32 = 0;
    if board_state_info.capturable {
        value -= 100;
    }
    if board_state_info.capturing {
        value += 100;
    }
    return value;
}
fn assign_capture_value_to_state(state: &mut State, board_state_info: &BoardStateInfo) -> i32 {
    let mut value: i32 = 0;
    let capture_count: i8;
    if state.current_player == 1 {
        state.white_captured_stone += board_state_info.stone_captured;
    } else {
        state.black_captured_stone += board_state_info.stone_captured;
    }
    if state.current_player == 1 {
        capture_count = state.white_captured_stone;
    } else {
        capture_count = state.black_captured_stone;
    }
    if capture_count >= 10 {
        value += global_var::HEURISTIC_MAX_VALUE;
    } else {
        value += board_state_info.stone_captured as i32 * capture_count as i32 * 50;
    }
    return value;
}
