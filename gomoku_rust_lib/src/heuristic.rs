//! Heuristic of our AI.

use crate::check_move::check_potential_winning_alignment;
use crate::check_pos_still_win;
use crate::checking_and_apply_bits_move;
use crate::data_struct::BoardStateInfo;
use crate::data_struct::State;
use crate::global_var;
use crate::heuristic_ratios;

pub fn heuristic(state: &mut State) -> i64 {
    let mut value: i64 = 0;
    let board_state_info: BoardStateInfo = checking_and_apply_bits_move(state);

    if !is_playable_move(state, &board_state_info) {
        state.is_playable = -1;
        return heuristic_ratios::HEURISTIC_MIN_VALUE;
    }
    state.board_info = board_state_info.clone();
    value += board_state_info.captured_pattern_blocking_value;
    let ret = is_in_winning_pos(state);
    if ret == heuristic_ratios::HEURISTIC_MAX_VALUE || ret == heuristic_ratios::HEURISTIC_MIN_VALUE
    {
        return ret;
    }
    let move_stone_captured = state.board_info.stone_captured;
    let player_total_stone_captured;
    let opponent_total_stone_captured;
    if state.current_player == global_var::VALID_MOVE {
        opponent_total_stone_captured = state.black_captured_stone;
        player_total_stone_captured = state.white_captured_stone;
    } else {
        opponent_total_stone_captured = state.white_captured_stone;
        player_total_stone_captured = state.black_captured_stone;
    }

    // Instant return move
    // Check if win by capturing stone
    unsafe {
        if move_stone_captured != 0
            && ((state.current_player == global_var::PLAYER_WHITE_NB
                && global_var::WHITE_CAPTURED_STONE + move_stone_captured >= 10)
                || (state.current_player == global_var::PLAYER_BLACK_NB
                    && global_var::BLACK_CAPTURED_STONE + move_stone_captured >= 10))
        {
            return heuristic_ratios::HEURISTIC_CAPTURE_TEN_STONE;
        }
    }
    // Checking if undefeatable 5, every pattern_axe should be (0 , 5)
    if board_state_info.pattern_axe[0].1 == 5 {
        return heuristic_ratios::HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW;
    }

    // Add stone captured value
    if move_stone_captured != 0 {
        value += heuristic_ratios::exponential_heuristic_prevent_capture_stone_calculator(
            player_total_stone_captured,
        );
    }

    let mut count_simple_blocking_triple = 0;
    let mut count_simple_blocking_two = 0;
    let current_player_axe;
    let opponent_axe;
    if state.current_player == global_var::PLAYER_WHITE_NB {
        current_player_axe = 0;
        opponent_axe = 1;
    } else {
        current_player_axe = 1;
        opponent_axe = 0;
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
        // Blocker value of 3 means no pattern found, so no value to add on this axe.
        if numbers_of_blocker_on_pattern != 3 {
            // Checking if creating a triple that will prevent a double to be captured.
            if numbers_of_blocker_on_pattern > 0
                && found_pattern_on_axe > 4
                && found_pattern_on_axe < 6
            {
                value += checking_if_pattern_is_blocking_a_capture_and_return_value(
                    state.axes[current_player_axe][axe_index],
                    state.axes[opponent_axe][axe_index],
                    numbers_of_blocker_on_pattern,
                    opponent_total_stone_captured,
                );
            } else {
                // Check if it's creating a pattern two with one blocker and the blocker is actually opponent stone, so it's a capturable move.
                // Except this one, add pattern heuristic ratio to value.
                if !(found_pattern_on_axe == 9
                    && numbers_of_blocker_on_pattern == 1
                    && !pattern_two_is_capturable(
                        state.axes[current_player_axe][axe_index],
                        state.axes[opponent_axe][axe_index],
                    ))
                {
                    // Add pattern values
                    value += heuristic_ratios::HEURISTIC_PATTERN[found_pattern_on_axe]
                        [numbers_of_blocker_on_pattern];
                }
            }
        }

        // Check blocker pattern on axe and add the value
        let found_blocker_pattern_on_axe = board_state_info.blocker_axe[axe_index].0;
        let numbers_of_blocker_on_blocked_pattern = board_state_info.blocker_axe[axe_index].1;
        // Blocker value of 3 means no pattern found
        if numbers_of_blocker_on_blocked_pattern != 3 {
            count_simple_blocking_triple +=
                is_simple_blocking_three_pattern(found_blocker_pattern_on_axe);
            count_simple_blocking_two += is_simple_blocking_two_pattern(
                found_blocker_pattern_on_axe,
                state.axes[opponent_axe][axe_index],
                state.axes[current_player_axe][axe_index],
            );
            value += heuristic_ratios::HEURISTIC_BLOCKER[found_blocker_pattern_on_axe]
                [numbers_of_blocker_on_blocked_pattern];
        }

        // Checking if AI try to block a double triple and prevent it
        if count_simple_blocking_two >= 2 {
            value += heuristic_ratios::HEURISTIC_BLOCK_A_DOUBLE_THREE;
        }

        // Force AI to block combination of free tree and free two
        if count_simple_blocking_two >= 1 && count_simple_blocking_triple >= 1 {
            value += heuristic_ratios::HEURISTIC_SIMPLE_BLOCK_THREE_AND_TWO;
        }
    }
    return value;
}

fn pattern_two_is_capturable(current_player_axe: u16, opponent_axe: u16) -> bool {
    if ((current_player_axe & (1 << 7) == 1 << 7)
        && (opponent_axe & (1 << 8) == 1 << 8 || opponent_axe & (1 << 6) == 1 << 6))
        || ((current_player_axe & (1 << 9) == 1 << 9)
            && (opponent_axe & (1 << 10) == 1 << 10 || opponent_axe & (1 << 7) == 1 << 7))
    {
        return true;
    }
    return false;
}

fn checking_if_pattern_is_blocking_a_capture_and_return_value(
    current_player_axe: u16,
    opponent_axe: u16,
    numbers_of_blocker_on_pattern: usize,
    opponent_stone_captured: i8,
) -> i64 {
    let mut value = 0;
    // Checking if new placed stone is not in center of pattern, otherwise it will
    // not prevent a two to be captureted (ex : -0X-X- before move, -0XXX- after)
    // Using bitwise to know that, current move is at position 8 on axe so we
    // check position 7 and 9 respectively to know if move is in center of the pattern.
    // also checking that enemy position can actually capture the two pattern.

    // println!("current_player_axe = {:016b}", current_player_axe);
    // println!("opponent_axe = {:016b}", opponent_axe);
    if (current_player_axe & (1 << 7) == 1 << 7) ^ (current_player_axe & (1 << 9) == 1 << 9)
        && !(opponent_axe & (1 << 9) == 1 << 9)
        && !(opponent_axe & (1 << 7) == 1 << 7)
    {
        // println!("blocking a capture");
        // Only one blocker, good move
        if numbers_of_blocker_on_pattern == 1 {
            value += heuristic_ratios::exponential_heuristic_prevent_capture_stone_calculator(
                opponent_stone_captured,
            );
        } else {
            value += heuristic_ratios::exponential_heuristic_prevent_capture_stone_calculator(
                opponent_stone_captured,
            ) / 3;
        }
    } else if numbers_of_blocker_on_pattern == 1 {
        value += heuristic_ratios::HEURISTIC_THREE_IN_A_ROW_ONE_BLOCKER;
    }
    return value;
}

fn is_simple_blocking_two_pattern(
    found_blocker_pattern_on_axe: usize,
    opponent_axe: u16,
    current_player_axe: u16,
) -> i16 {
    // Check blocker table in heuristic_ratios.rs
    if found_blocker_pattern_on_axe == 9
        && ((opponent_axe & (1 << 7) == 1 << 7
            && opponent_axe & (1 << 6) == 1 << 6
            && current_player_axe & (1 << 5) != 1 << 5)
            || (opponent_axe & (1 << 9) == 1 << 9
                && opponent_axe & (1 << 10) == 1 << 10
                && current_player_axe & (1 << 11) != 1 << 11))
    {
        return 1;
    }
    return 0;
}

fn is_simple_blocking_three_pattern(found_blocker_pattern_on_axe: usize) -> i16 {
    // Check blocker table in heuristic_ratios.rs
    if found_blocker_pattern_on_axe > 4 && found_blocker_pattern_on_axe < 6 {
        return 1;
    }
    return 0;
}

fn is_playable_move(state: &mut State, board_state_info: &BoardStateInfo) -> bool {
    if board_state_info.is_wrong_move != global_var::VALID_MOVE {
        state.is_playable = board_state_info.is_wrong_move;
        return false;
    }
    return true;
}

fn is_in_winning_pos(state: &mut State) -> i64 {
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
