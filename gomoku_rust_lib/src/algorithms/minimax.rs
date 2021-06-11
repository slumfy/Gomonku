use crate::algorithms::algo_utils::update_max_depth;
use crate::algorithms::algo_utils::update_node_checked_count;
use crate::algorithms::algo_utils::update_pruning_count;
// use crate::algorithms::algo_utils::update_pruning_count;
// use crate::algorithms::transpotable;
use crate::data_struct::State;
use crate::heuristic_ratios;
use crate::state::create_child;
use crate::state::state_is_terminated;
use std::cmp::Reverse;
use crate::global_var;

pub fn minimax(
    mut state: &mut State,
    depth: i32,
    mut alpha: i64,
    mut beta: i64,
    maximizingplayer: bool,
) -> i64 {
    update_node_checked_count();
    update_max_depth(depth);
    if depth == 0 || state_is_terminated(state) == true {
		if state_is_terminated(state) == true {
		// println!("heuristic {} winstate {:?} player {}",state.heuristic, state.win_state, state.current_player);
		}
        return state.heuristic;
    }
    state.available_move = create_child(&mut state);
    state.available_move.sort_by_key(|d| Reverse(d.heuristic));
	unsafe {
	if depth == global_var::DEPTH {
		for child in 0..state.available_move.len() {
			println!("player {},  heuristic {}",state.available_move[child].current_player, state.available_move[child].heuristic);
		}
	}
}
    let mut value: i64;
    if maximizingplayer == true {
		state.available_move.sort_by_key(|d| Reverse(d.heuristic));
        value = heuristic_ratios::MIN_VALUE;
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
            if value >= beta {
				update_pruning_count();
                break;
            }
			alpha = std::cmp::max(alpha, value);
        }
    } else {
		state.available_move.sort_by_key(|d| d.heuristic);
        value = heuristic_ratios::MAX_VALUE;
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
            if value <= alpha {
				update_pruning_count();
                break;
            }
			beta = std::cmp::min(alpha, value);
        }
    }
    state.heuristic = value;
    return value;
}
