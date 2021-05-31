use crate::algorithms::algo_utils::update_max_depth;
use crate::algorithms::algo_utils::update_node_checked_count;
use crate::algorithms::algo_utils::update_pruning_count;
use crate::algorithms::algo_utils::update_tt_count;
use crate::algorithms::transpotable;
use crate::data_struct::Flag;
use crate::data_struct::State;
use crate::global_var;
use crate::heuristic_ratios;
use crate::state::create_child;
use crate::state::state_is_terminated;
use std::cmp::Reverse;

pub fn negamax(mut state: &mut State, depth: i32, mut alpha: i64, mut beta: i64) -> i64 {
    let alphaorigin = alpha;
    update_node_checked_count();
    update_max_depth(depth);
    if unsafe { depth != global_var::DEPTH } {
        let tt_entry = transpotable::tt_search(state);
        if tt_entry != None {
            let tt_unwrap = tt_entry.unwrap();
            if tt_unwrap.depth >= depth {
                // println!("TTentry {:?}",tt_unwrap);
                if tt_unwrap.flag == Flag::EXACT {
                    update_tt_count();
                    return tt_unwrap.value;
                } else if tt_unwrap.flag == Flag::LOWERBOUND {
                    alpha = std::cmp::max(alpha, tt_unwrap.value);
                } else if tt_unwrap.flag == Flag::UPPERBOUND {
                    beta = std::cmp::min(beta, tt_unwrap.value);
                }
                if alpha >= beta {
                    update_tt_count();
                    return tt_unwrap.value;
                }
            }
        }
    }
    if depth == 0 || state_is_terminated(state) == true {
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
    //TT STORING
    if unsafe { depth != global_var::DEPTH } {
        if state.heuristic <= alphaorigin {
            transpotable::tt_insert(state, depth, Flag::UPPERBOUND);
        } else if state.heuristic >= beta {
            transpotable::tt_insert(state, depth, Flag::LOWERBOUND);
        } else {
            transpotable::tt_insert(state, depth, Flag::EXACT);
        }
    }
    return state.heuristic;
}
