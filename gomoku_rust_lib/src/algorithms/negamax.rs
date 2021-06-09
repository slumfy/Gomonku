  
use crate::algorithms::algo_utils::update_max_depth;
use crate::algorithms::algo_utils::update_node_checked_count;
use crate::algorithms::algo_utils::update_pruning_count;
use crate::algorithms::transpotable;
use crate::data_struct::State;
use crate::data_struct::Transpotablenode;
use crate::data_struct::Flag;
use crate::global_var;
use crate::heuristic_ratios;
use crate::print::print_heuristic_table;
use crate::print::print_pos_in_human_format;
use crate::state::create_child;
use crate::state::state_is_terminated;
use std::cmp::Reverse;

pub fn negamax(mut state: &mut State, depth: i32, mut alpha: i64, mut beta: i64) -> i64 {
	let mut alphaorigin = alpha;
    update_node_checked_count();
    update_max_depth(depth);
	let tt_entry = transpotable::tt_search(state);
	if tt_entry != None {
		let tt_unwrap = tt_entry.unwrap();
		if tt_unwrap.depth >= depth {
			// println!("TTentry {:?}",tt_unwrap);
			if tt_unwrap.flag == Flag::EXACT {
				return(tt_unwrap.value);
			}
			else if tt_unwrap.flag == Flag::LOWERBOUND {
				alpha = std::cmp::max(alpha, tt_unwrap.value);
			}
			else if tt_unwrap.flag == Flag::UPPERBOUND {
				beta = std::cmp::min(beta, tt_unwrap.value);
			}
			if alpha >= beta {
				return(tt_unwrap.value);
			}
		}
	}
    if depth == 0 || state_is_terminated(state) == true {
        return state.heuristic;
    }
    state.available_move = create_child(&mut state);
    state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    if state.available_move.len() < 1 {
        return state.heuristic;
    }
    let mut value: i64 = heuristic_ratios::MIN_VALUE;
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
	//TT STORING
	if state.heuristic <= alphaorigin {
		transpotable::tt_insert(state,depth,Flag::UPPERBOUND);
	}
	else if state.heuristic >= beta {
        transpotable::tt_insert(state,depth,Flag::LOWERBOUND);
	}
	else {
        transpotable::tt_insert(state,depth,Flag::EXACT);
	}
	state.heuristic = value;
    return state.heuristic;
}