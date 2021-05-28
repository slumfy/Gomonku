use crate::algorithms::algo_utils::update_max_depth;
use crate::algorithms::algo_utils::update_node_checked_count;
use crate::algorithms::algo_utils::update_pruning_count;
use crate::algorithms::transpotable;
use crate::data_struct::State;
use crate::data_struct::Transpotablenode;
use crate::global_var;
use crate::heuristic_ratios;
use crate::print::print_heuristic_table;
use crate::print::print_pos_in_human_format;
use crate::state::create_child;
use crate::state::state_is_terminated;
use std::cmp::Reverse;

pub fn negamax(mut state: &mut State, depth: i32, mut alpha: i64, beta: i64) -> i64 {
    update_node_checked_count();
    update_max_depth(depth);
    if depth == 0 || state_is_terminated(state) == true {
        // transpotable::tt_insert(state,depth);
        // transpotable::tt_search(state,depth);
        return state.heuristic;
    }
    if depth == 1 {
        state.available_move = create_child(&mut state);
    } else {
        state.available_move = create_child(&mut state);
    }
    state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    if state.available_move.len() < 1 {
        return state.heuristic;
    }
    let mut value: i64 = heuristic_ratios::HEURISTIC_MIN_VALUE;
    for child_index in 0..state.available_move.len() {
        let negamax_value;
        negamax_value = negamax(
            &mut state.available_move[child_index],
            depth - 1,
            -beta,
            -alpha,
        );

        value = std::cmp::max(value, negamax_value);
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            update_pruning_count();
            break;
        }
    }
    if (state.heuristic as i128 - (value / 5) as i128)
        > heuristic_ratios::HEURISTIC_MAX_VALUE as i128
    {
        state.heuristic = heuristic_ratios::HEURISTIC_MAX_VALUE;
    } else if (state.heuristic as i128 - (value / 5) as i128)
        < heuristic_ratios::HEURISTIC_MIN_VALUE as i128
    {
        state.heuristic = heuristic_ratios::HEURISTIC_MIN_VALUE;
    } else {
        state.heuristic = state.heuristic - value / 5;
    }
    return state.heuristic;
}
