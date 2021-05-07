//! Heuristic of our AI.

use crate::bitboard_operations::apply_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::check_move::check_is_wrong_move;
use crate::check_move::check_move_is_capturing_stone;
use crate::check_move::check_potential_winning_alignment;
use crate::check_pos_still_win;
use crate::checking_and_apply_bits_move;
use crate::data_struct::BoardStateInfo;
use crate::data_struct::State;
use crate::global_var;
use crate::global_var::PATTERN;
use crate::heuristic_ratios;

pub fn heuristic(state: &mut State) -> i32 {
    let mut value: i32 = 0;
    let win_state: i32;
    let board_state_info: BoardStateInfo = checking_and_apply_bits_move(state);

    if !is_playable_move(state, &board_state_info) {
        state.is_playable = -1;
        return heuristic_ratios::HEURISTIC_MIN_VALUE;
    }
    state.board_info = board_state_info.clone();

    // Instant return move
    // Check if win by capturing stone
    let stone_captured = check_move_is_capturing_stone(&state.axes[0], &state.axes[1]);
    unsafe {
        if stone_captured != 0
            && state.current_player == global_var::PLAYER_WHITE_NB
            && global_var::WHITE_CAPTURED_STONE + stone_captured >= 10
        {
            return heuristic_ratios::HEURISTIC_CAPTURE_TEN_STONE;
        } else if stone_captured != 0
            && state.current_player == global_var::PLAYER_BLACK_NB
            && global_var::BLACK_CAPTURED_STONE + stone_captured >= 10
        {
            return heuristic_ratios::HEURISTIC_CAPTURE_TEN_STONE;
        }
    }

    let axes_development_value = check_potential_winning_alignment(state);
    println!("axes_development_value {:?}", axes_development_value);
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
    let player_capture_count: i8;
    let opponent_capture_count: i8;
    if state.current_player == 1 {
        player_capture_count = state.white_captured_stone;
        opponent_capture_count = state.black_captured_stone;
    } else {
        player_capture_count = state.black_captured_stone;
        opponent_capture_count = state.white_captured_stone;
    }
    if player_capture_count >= 10 {
        return heuristic_ratios::HEURISTIC_MAX_VALUE;
    } else if opponent_capture_count >= 10 {
        return heuristic_ratios::HEURISTIC_MIN_VALUE;
    }
    if state.win_state.1 != 0 {
        if check_pos_still_win(state.bitboards, state.win_state.0, state.win_state.1) == true {
            if state.current_player == state.win_state.1 {
                value = heuristic_ratios::HEURISTIC_MAX_VALUE;
            } else {
                value = heuristic_ratios::HEURISTIC_MIN_VALUE;
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

// fn assign_pattern_value_to_state(state: &mut State, board_state_info: &BoardStateInfo) -> i32 {
//     let mut value: i32 = 0;
//     if board_state_info.pattern_value == heuristic_ratios::HEURISTIC_MAX_VALUE {
//         return heuristic_ratios::HEURISTIC_MAX_VALUE;
//     }
//     let mut opponent_move_to_win: i8;
//     let mut pattern_multiplier: i32 = 1;
//     let mut blocker_multiplier: i32 = 1;
//     if state.current_player == 1 {
//         opponent_move_to_win = state.black_move_to_win;
//     } else {
//         opponent_move_to_win = state.white_move_to_win;
//     }
//     if board_state_info.nb_move_to_win < opponent_move_to_win {
//         pattern_multiplier = heuristic_ratios::PATTERN_MULTIPLIER;
//     } else {
//         blocker_multiplier = heuristic_ratios::BLOCKER_MULTIPLIER;
//     }
//     value += board_state_info.blocker_value * blocker_multiplier;
//     value += board_state_info.pattern_value * pattern_multiplier;
//     return value;
// }

// fn assign_capturing_pos_value_to_state(board_state_info: &BoardStateInfo) -> i32 {
//     let mut value: i32 = 0;
//     if board_state_info.capturable {
//         value -= heuristic_ratios::CAPTURABLE_POS_SCORE;
//     }
//     if board_state_info.capturing {
//         value += heuristic_ratios::CAPTURING_POS_SCORE;
//     }
//     return value;
// }
// fn assign_capture_value_to_state(state: &mut State, board_state_info: &BoardStateInfo) -> i32 {
//     let mut value: i32 = 0;
//     let capture_count: i8;
//     if state.current_player == 1 {
//         state.white_captured_stone += board_state_info.stone_captured;
//     } else {
//         state.black_captured_stone += board_state_info.stone_captured;
//     }
//     if state.current_player == 1 {
//         capture_count = state.white_captured_stone;
//     } else {
//         capture_count = state.black_captured_stone;
//     }
//     value += board_state_info.stone_captured as i32
//         * capture_count as i32
//         * heuristic_ratios::CAPTURING_COUNT_RATIO_MULTIPLIER;
//     return value;
// }
