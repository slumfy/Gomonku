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

    // Add new stone captured value to current state to save for childrens and add the current value to heuristic value.
    let opponent_total_stone_captured;
    if move_stone_captured != 0 {
        if state.current_player == global_var::PLAYER_WHITE_NB {
            state.total_white_captured_stone += move_stone_captured;
            opponent_total_stone_captured = state.total_black_captured_stone;
            // Winning by capture, instant return.
            if state.total_white_captured_stone >= 10 {
                return heuristic_ratios::HEURISTIC_MAX_VALUE;
            }
            state.all_depth_white_captured_stone_value =
                heuristic_ratios::exponential_heuristic_prevent_capture_stone_calculator(
                    state.total_white_captured_stone,
                );
        } else {
            state.total_black_captured_stone += move_stone_captured;
            opponent_total_stone_captured = state.total_white_captured_stone;
            if state.total_black_captured_stone >= 10 {
                return heuristic_ratios::HEURISTIC_MAX_VALUE;
            }
            state.all_depth_black_captured_stone_value =
                heuristic_ratios::exponential_heuristic_prevent_capture_stone_calculator(
                    state.total_black_captured_stone,
                );
        }
    } else if state.current_player == global_var::PLAYER_WHITE_NB {
        opponent_total_stone_captured = state.total_black_captured_stone;
    } else {
        opponent_total_stone_captured = state.total_white_captured_stone;
    }
    // Add all past stone capture value and substract past opponnent capture value.
    if state.current_player == global_var::PLAYER_WHITE_NB {
        value += state.all_depth_white_captured_stone_value;
        value -= state.all_depth_black_captured_stone_value;
    } else {
        value += state.all_depth_black_captured_stone_value;
        value -= state.all_depth_white_captured_stone_value;
    }

    // Checking if undefeatable 5, every pattern_axe should be (0 , 5)
    if board_state_info.pattern_axe[0].1 == 5 {
        return heuristic_ratios::HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW;
    }

    let mut count_blocking_triple = 0;
    let mut count_simple_blocking_two = 0;
    let mut count_blocking_two = 0;
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
                && found_pattern_on_axe == 5
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
                if !(found_pattern_on_axe == 8
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
            count_blocking_triple += is_blocking_three_pattern(found_blocker_pattern_on_axe);
            count_simple_blocking_two += is_simple_blocking_two_pattern(
                found_blocker_pattern_on_axe,
                state.axes[opponent_axe][axe_index],
                state.axes[current_player_axe][axe_index],
            );
            count_blocking_two += is_blocking_two_pattern(found_blocker_pattern_on_axe);
            value += heuristic_ratios::HEURISTIC_BLOCKER[found_blocker_pattern_on_axe]
                [numbers_of_blocker_on_blocked_pattern];
        }

        // Checking if AI try to block a double triple and prevent it
        if count_simple_blocking_two >= 2 {
            // Adding a negative value 2 times bigger than a free three.
            value += heuristic_ratios::HEURISTIC_BLOCK_A_DOUBLE_THREE;
        }

        // Force AI to block combination of free tree and free two
        if count_blocking_two >= 1 && count_blocking_triple >= 1 {
            value += heuristic_ratios::HEURISTIC_SIMPLE_BLOCK_THREE_AND_TWO;
        }
    }
    return value;
}

fn pattern_two_is_capturable(current_player_axe: u16, opponent_axe: u16) -> bool {
    if (current_player_axe & (1 << 7) == (1 << 7)
        && (opponent_axe & (1 << 9) == (1 << 9) || opponent_axe & (1 << 6) == (1 << 6)))
        || (current_player_axe & (1 << 9) == 1 << 9
            && (opponent_axe & (1 << 10) == (1 << 10) || opponent_axe & (1 << 7) == (1 << 7)))
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

    if (current_player_axe & (1 << 7) == 1 << 7) ^ (current_player_axe & (1 << 9) == 1 << 9)
        && !(opponent_axe & (1 << 9) == 1 << 9)
        && !(opponent_axe & (1 << 7) == 1 << 7)
    {
        // Only one blocker, good move
        if numbers_of_blocker_on_pattern == 1 {
            value += heuristic_ratios::exponential_heuristic_prevent_capture_stone_calculator(
                opponent_stone_captured,
            );
        }
        // Two blocker, not really good move because we will create a two blocker three, unplayable.
        else {
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
    if found_blocker_pattern_on_axe == 8
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

fn is_blocking_two_pattern(found_blocker_pattern_on_axe: usize) -> i16 {
    // Check blocker table in heuristic_ratios.rs
    if found_blocker_pattern_on_axe == 8 {
        return 1;
    }
    return 0;
}

fn is_blocking_three_pattern(found_blocker_pattern_on_axe: usize) -> i16 {
    // Check blocker table in heuristic_ratios.rs
    if found_blocker_pattern_on_axe == 5 {
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
