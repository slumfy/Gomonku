use crate::data_struct::State;
use crate::data_struct::Transpotablenode;
use crate::global_var;
use crate::heuristic_ratios;
use crate::print::print_heuristic_table;
use crate::state::create_child;
use crate::state::state_is_terminated;
use crate::algorithms::transpotable::transposition_table_push;
use crate::algorithms::transpotable::transposition_table_search;
use crate::algorithms::transpotable::TRANSPOTABLENEGA;
use crate::algorithms::transpotable::TRANSPOTABLESCOUT;
use crate::algorithms::algo_utils::update_node_checked_count;
use crate::algorithms::algo_utils::update_max_depth;
use std::cmp::Reverse;


pub fn negamax(mut state: &mut State, depth: i32, mut alpha: i64, beta: i64, color: i8) -> i64 {
    update_node_checked_count();
    update_max_depth(depth);
    if depth == 0 || state_is_terminated(state) == true {
        return state.heuristic * color as i64;
    }
    if depth != 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    if state.available_move.len() <= 1 {
        return state.heuristic * color as i64;
    }
    let mut value: i64 = heuristic_ratios::HEURISTIC_MIN_VALUE;
    for child_index in 0..state.available_move.len() {
        let negamax_value;
        negamax_value = -negamax(
            &mut state.available_move[child_index],
            depth - 1,
            -beta,
            -alpha,
            -color,
        );

        value = std::cmp::max(value, negamax_value);
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            break;
        }
    }
    state.heuristic = value;
    return value;
}