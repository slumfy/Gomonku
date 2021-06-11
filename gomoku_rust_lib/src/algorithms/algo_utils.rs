use crate::data_struct::State;
use crate::global_var;
use crate::print::print_heuristic_table;
use std::cmp::Reverse;
use crate::algorithms::transpotable::clear_tt_table;

pub fn return_move(state: &mut State) -> (usize, i64) {
    print_heuristic_table(state);
    unsafe {
        println!("MAX DEPTH: {}", global_var::MAX_DEPTH_REACH);
        println!("nb of node checked: {:?}", global_var::NODE_CHECKED_COUNT);
        println!("pruning count: {:?}", global_var::PRUNING_COUNT);
        println!("TT cut count: {:?}", global_var::TT_COUNT);
    }
	reset_max_depth();
    reset_node_checked_count();
    reset_pruning_count();
    reset_tt_count();
	clear_tt_table();
	// state.available_move.sort_by_key(|d| d.heuristic);
    // for child in 0..state.available_move.len() {
	// 	if state.available_move[child].heuristic == -state.heuristic {
	// 		return(state.available_move[child].current_move_pos,
	// 			state.available_move[child].heuristic);
	// 	}
	// }
	for child in 0..state.available_move.len() {
		println!("player {},  heuristic {}",state.available_move[child].current_player, state.available_move[child].heuristic);
	}
	state.available_move.sort_by_key(|d| Reverse(d.heuristic));
	print_tree_path(state);
	println!("return move {:?} {:?}",state.available_move[0].current_move_pos,
	state.available_move[0].heuristic);
    return (
        (state.available_move[0].current_move_pos),
        state.available_move[0].heuristic,
    );
}

fn print_tree_path(state: &mut State) {
	let mut node = state;
	while true {
		for child in 0..node.available_move.len() {
			if node.available_move[child].heuristic == node.heuristic {
				println!("PLAYER {} MOVE {} previous heuristic {} saved heuristic {} heuristic {}", node.available_move[child].current_player,node.available_move[child].current_move_pos,node.available_move[child].previous_heuristic,node.available_move[child].saved_heuristic,node.available_move[child].heuristic);
				node = &mut node.available_move[child];
				break ;
			}
		}
		if node.available_move.len() == 0 {
			break;
		}
	}
}

pub fn update_max_depth(depth: i32) {
    unsafe {
		// println!("maxdepth {} DEPTH {} , depth {}",global_var::MAX_DEPTH_REACH,global_var::DEPTH,depth);
        if global_var::DEPTH - depth > global_var::MAX_DEPTH_REACH {
            global_var::MAX_DEPTH_REACH = global_var::DEPTH - depth;
        }
    }
}

pub fn reset_max_depth() {
    unsafe {
        global_var::MAX_DEPTH_REACH = 0;
    }
}

pub fn update_node_checked_count() {
    unsafe {
        global_var::NODE_CHECKED_COUNT += 1;
    }
}

pub fn reset_node_checked_count() {
    unsafe {
        global_var::NODE_CHECKED_COUNT = 0;
    }
}

pub fn update_pruning_count() {
    unsafe {
        global_var::PRUNING_COUNT += 1;
    }
}

pub fn reset_pruning_count() {
    unsafe {
        global_var::PRUNING_COUNT = 0;
    }
}

pub fn update_tt_count() {
    unsafe {
        global_var::TT_COUNT += 1;
    }
}

pub fn reset_tt_count() {
    unsafe {
        global_var::TT_COUNT = 0;
    }
}
