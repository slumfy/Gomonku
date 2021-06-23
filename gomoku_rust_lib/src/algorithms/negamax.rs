use crate::algorithms::algo_utils::update_max_depth;
use crate::algorithms::algo_utils::update_node_checked_count;
use crate::algorithms::algo_utils::update_pruning_count;
// use crate::algorithms::algo_utils::update_tt_count;
// use crate::algorithms::transpotable;
// use crate::data_struct::Flag;
use crate::data_struct::State;
// use crate::global_var;
use crate::heuristic_ratios;
use crate::state::create_child;
use crate::state::state_is_terminated;
use std::cmp::Reverse;

pub fn negamax(mut state: &mut State, depth: i32) -> i64 {
    update_node_checked_count();
    update_max_depth(depth);
	let terminated_value = state_is_terminated(state);
    if depth == 0 || terminated_value != 0 {
        if terminated_value == 1 {
            state.heuristic = heuristic_ratios::MAX_VALUE;
        } else if terminated_value == -1 {
			state.heuristic = heuristic_ratios::MIN_VALUE;
		}
            return state.heuristic;
    }
    state.available_move = create_child(&mut state);
    state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    if state.available_move.len() < 1 {
        return state.heuristic;
    }
    let mut value: i64 = heuristic_ratios::MIN_VALUE;
    let mut alpha: i128 = heuristic_ratios::MIN_VALUE as i128;
    for child_index in 0..state.available_move.len() {
        let negamax_value;
        negamax_value = negamax(&mut state.available_move[child_index], depth - 1);

        value = std::cmp::max(value, negamax_value);

        // Alpha pruning
        if (state.heuristic as i128
            - value as i128 / heuristic_ratios::HEURISTIC_MULTIPLIER as i128)
            <= alpha
            && alpha != heuristic_ratios::MIN_VALUE as i128
        {
            update_pruning_count();
            break;
        } else {
            alpha = state.heuristic as i128
                - value as i128 / heuristic_ratios::HEURISTIC_MULTIPLIER as i128;
        }
    }

    // Terminated state on depth, return opposite value.
    if value == heuristic_ratios::MAX_VALUE {
        state.heuristic = heuristic_ratios::MIN_VALUE;
        return state.heuristic;
    }
	if value == heuristic_ratios::MIN_VALUE {
        state.heuristic = heuristic_ratios::MAX_VALUE;
        return state.heuristic;
    }

    state.heuristic = substract_depth_value_to_heuristic(state.heuristic, value);
    return state.heuristic;
}

fn substract_depth_value_to_heuristic(current_heuristic: i64, in_depth_heuristic: i64) -> i64 {
    // Check for underflow or overflow
    if check_i64_substraction_overflow(
        current_heuristic,
        in_depth_heuristic / heuristic_ratios::HEURISTIC_MULTIPLIER,
    ) {
        return heuristic_ratios::MAX_VALUE;
    } else if check_i64_substraction_underflow(
        current_heuristic,
        in_depth_heuristic / heuristic_ratios::HEURISTIC_MULTIPLIER,
    ) {
        return heuristic_ratios::MIN_VALUE;
    } else {
        return current_heuristic - in_depth_heuristic / heuristic_ratios::HEURISTIC_MULTIPLIER;
    }
}

fn check_i64_substraction_overflow(value_one: i64, value_two: i64) -> bool {
    if (value_one as i128 - value_two as i128) > (heuristic_ratios::MAX_VALUE as i128) {
        return true;
    }
    return false;
}

fn check_i64_substraction_underflow(value_one: i64, value_two: i64) -> bool {
    if (value_one as i128 - value_two as i128) < (heuristic_ratios::MIN_VALUE as i128) {
        return true;
    }
    return false;
}
