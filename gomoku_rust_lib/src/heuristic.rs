//! Heuristic of our AI.

use crate::check_move::check_potential_winning_alignment;
use crate::check_pos_still_win;
use crate::checking_and_apply_bits_move;
use crate::data_struct::BoardStateInfo;
use crate::data_struct::State;
use crate::global_var;
// use crate::global_var::PATTERN;
use crate::heuristic_ratios;

pub fn heuristic(state: &mut State) -> i32 {
    let mut value: i32 = 0;
    let board_state_info: BoardStateInfo = checking_and_apply_bits_move(state);

    if !is_playable_move(state, &board_state_info) {
        state.is_playable = -1;
        return heuristic_ratios::HEURISTIC_MIN_VALUE;
    }
    state.board_info = board_state_info.clone();
    let ret = is_in_winning_pos(state);
    if ret == heuristic_ratios::HEURISTIC_MAX_VALUE || ret == heuristic_ratios::HEURISTIC_MIN_VALUE
    {
        return ret;
    }
    let stone_captured = state.board_info.stone_captured;

    // Instant return move
    // Check if win by capturing stone
    unsafe {
        if stone_captured != 0
            && ((state.current_player == global_var::PLAYER_WHITE_NB
                && global_var::WHITE_CAPTURED_STONE + stone_captured >= 10)
                || (state.current_player == global_var::PLAYER_BLACK_NB
                    && global_var::BLACK_CAPTURED_STONE + stone_captured >= 10))
        {
            return heuristic_ratios::HEURISTIC_CAPTURE_TEN_STONE;
        }
    }
    // Checking if undefeatable 5, every pattern_axe should be (0 , 5)
    if board_state_info.pattern_axe[0].1 == 5 {
        return heuristic_ratios::HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW;
    }

    // Add stone captured value
    if stone_captured != 0 {
        value += heuristic_ratios::HEURISTIC_CAPTURING_ONE_STONE * stone_captured as i32;
    }

    for axe_index in 0..4 {
        // Check potential_winning_alignment
        let potential_winning_alignment = check_potential_winning_alignment(state);
        if potential_winning_alignment[axe_index] {
            value += heuristic_ratios::HEURISTIC_POSSIBLE_AXE_DEVELOPMENT;
        }

        // Check pattern on axe and add the value
        let found_pattern_on_axe = board_state_info.pattern_axe[axe_index].0;
        let numbers_of_blocker_on_pattern = board_state_info.pattern_axe[axe_index].1;
        // Blocker value of 3 means no pattern found
        if numbers_of_blocker_on_pattern != 3 {
            value += heuristic_ratios::HEURISTIC_PATTERN[found_pattern_on_axe]
                [numbers_of_blocker_on_pattern];
        }

        let found_blocker_pattern_on_axe = board_state_info.blocker_axe[axe_index].0;
        let numbers_of_blocker_on_blocked_pattern = board_state_info.blocker_axe[axe_index].1;
        // Blocker value of 3 means no pattern found
        if numbers_of_blocker_on_blocked_pattern != 3 {
            value += heuristic_ratios::HEURISTIC_BLOCKER[found_blocker_pattern_on_axe]
                [numbers_of_blocker_on_blocked_pattern];
        }
    }
    return value;
}

fn is_playable_move(state: &mut State, board_state_info: &BoardStateInfo) -> bool {
    if board_state_info.is_wrong_move != global_var::VALID_MOVE {
        state.is_playable = board_state_info.is_wrong_move;
        return false;
    }
    return true;
}

fn is_in_winning_pos(state: &mut State) -> i32 {
    if state.win_state.1 != 0 {
        if check_pos_still_win(state.bitboards, state.win_state.0, state.win_state.1) == true {
            if state.current_player == state.win_state.1 {
                return heuristic_ratios::HEURISTIC_MAX_VALUE;
            } else {
                return heuristic_ratios::HEURISTIC_MIN_VALUE;
            }
        } else {
            state.win_state = (0, 0);
        }
    }
    if state.board_info.is_winning.1 != 0 {
        state.win_state = state.board_info.is_winning;
    }
    return 0;
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
