use crate::data_struct::State;
use crate::data_struct::Transpotablenode;
use crate::global_var;
use crate::heuristic_ratios;
use crate::print::print_heuristic_table;
use crate::state::create_child;
use crate::state::state_is_terminated;
use crate::algorithms::algo_utils::update_node_checked_count;
use crate::algorithms::algo_utils::update_max_depth;
use crate::algorithms::algo_utils::update_pruning_count;
use crate::algorithms::transpotable;
use std::cmp::Reverse;

pub fn minimax(mut state: &mut State, depth: i32, mut alpha: i64, mut beta: i64, maximizingplayer: bool) -> i64 {
	update_node_checked_count();
    update_max_depth(depth);
    if depth == 0 || state_is_terminated(state) == true {
        return state.heuristic;
    }
	if depth != 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
	let mut value: i64;
	if maximizingplayer == true {
		value = heuristic_ratios::HEURISTIC_MIN_VALUE;
		for child_index in 0..state.available_move.len() {
			let minimax_value;
			minimax_value = minimax(
				&mut state.available_move[child_index],
				depth - 1,
				beta,
				alpha,
				false,
			);
			value = std::cmp::max(value, minimax_value);
			alpha = std::cmp::max(alpha, value);
			if alpha >= beta {
				update_pruning_count();
				break;
			}
		}
	}
	else {
		value = heuristic_ratios::HEURISTIC_MAX_VALUE;
		for child_index in 0..state.available_move.len() {
			let minimax_value;
			minimax_value = minimax(
				&mut state.available_move[child_index],
				depth - 1,
				beta,
				alpha,
				true,
			);
			value = std::cmp::min(value, minimax_value);
			beta = std::cmp::min(alpha, value);
			if alpha >= beta {
				update_pruning_count();
				break;
			}
		}
	}
	state.heuristic = value;
    return value;
}