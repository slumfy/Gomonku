use crate::data_struct::State;
use crate::global_var;
use crate::print::print_heuristic_table;
use std::cmp::Reverse;

pub fn return_move(state: &mut State) -> (usize, i64) {
    print_heuristic_table(state);
    unsafe {
        println!("MAX DEPTH: {}", global_var::MAX_DEPTH_REACH);
        println!("nb of node checked: {:?}", global_var::NODE_CHECKED_COUNT);
        println!("pruning count: {:?}", global_var::PRUNING_COUNT);
        println!("TT cut count: {:?}", global_var::TT_COUNT);
    }
    reset_node_checked_count();
    reset_pruning_count();
    reset_tt_count();
    // println!("NB OF MOVE: {}",state.available_move.len());
    // for child in 0..state.available_move.len() {
    //     println!(
    //         "child {} heuristic {} pos {}",
    //         child,
    //         state.available_move[child].heuristic,
    //         state.available_move[child].bit_current_move_pos
    //     );
    // }
    state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    return (
        (state.available_move[0].bit_current_move_pos),
        state.available_move[0].heuristic,
    );
}

pub fn update_max_depth(depth: i32) {
    unsafe {
        if global_var::DEPTH - depth > global_var::MAX_DEPTH_REACH {
            global_var::MAX_DEPTH_REACH = global_var::DEPTH - depth;
        }
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
