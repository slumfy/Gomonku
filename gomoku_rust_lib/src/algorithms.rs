//! Four algorithms use by the AI.

use crate::data_struct::State;
use crate::data_struct::Transpotablenode;
use crate::global_var;
use crate::heuristic_ratios;
use crate::print::print_heuristic_table;
use crate::state::create_child;
use crate::state::state_is_terminated;
use crate::transpotable::transposition_table_push;
use crate::transpotable::transposition_table_search;
use crate::transpotable::TRANSPOTABLENEGA;
use crate::transpotable::TRANSPOTABLESCOUT;
use std::cmp::Reverse;

pub fn negamax(mut state: &mut State, depth: i32, mut alpha: i32, beta: i32, color: i8) -> i32 {
    update_node_checked_count();
    update_max_depth(depth);
    if depth == 0 || state_is_terminated(state) == true {
        return state.heuristic * color as i32;
    }
    if depth != 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    // println!("NEGAMAX player {}  color {} state: {}", state.current_player, color, state.bit_current_move_pos);
    // if depth == global_var::DEPTH {
    //     for child in 0..state.available_move.len() {
    //         println!(
    //             "child {} heuristic {} pos {}",
    //             child,
    //             state.available_move[child].heuristic,
    //             state.available_move[child].bit_current_move_pos
    //         );
    //     }
    // }
    let mut value: i32 = heuristic_ratios::HEURISTIC_MIN_VALUE;
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

fn update_max_depth(depth: i32) {
    unsafe {
        if global_var::DEPTH - depth > global_var::MAX_DEPTH_REACH {
            global_var::MAX_DEPTH_REACH = global_var::DEPTH - depth;
        }
    }
}

fn update_node_checked_count() {
    unsafe {
        global_var::NODE_CHECKED_COUNT += 1;
    }
}

fn reset_node_checked_count() {
    unsafe {
        global_var::NODE_CHECKED_COUNT = 0;
    }
}

fn update_pruning_count() {
    unsafe {
        global_var::PRUNING_COUNT += 1;
    }
}

fn reset_pruning_count() {
    unsafe {
        global_var::PRUNING_COUNT = 0;
    }
}

fn update_tt_count() {
    unsafe {
        global_var::TT_COUNT += 1;
    }
}

fn reset_tt_count() {
    unsafe {
        global_var::TT_COUNT = 0;
    }
}

pub fn negamax_with_transpotable(
    mut state: &mut State,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    color: i8,
) -> i32 {
    update_node_checked_count();
    update_max_depth(depth);
    let tt_search: (bool, i32, i32);
    unsafe { tt_search = transposition_table_search(state, &TRANSPOTABLENEGA) };
    if tt_search.0 == true && tt_search.1 >= depth {
        update_tt_count();
        return tt_search.2;
    }
    if depth != 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    // println!("current state: {:?} player to play {} current heuristic {} depth {}", state.current_move, state.player_to_play, state.heuristic, depth);
    if depth == 0 || state.available_move.len() == 0 || state_is_terminated(state) == true {
        return state.heuristic * color as i32;
    }
    let mut value: i32 = heuristic_ratios::HEURISTIC_MIN_VALUE;
    let len = state.available_move.len();
    for child in 0..len {
        let negamax = -negamax(
            &mut state.available_move[child],
            depth - 1,
            -beta,
            -alpha,
            -color,
        );
        value = std::cmp::max(value, negamax);
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            update_pruning_count();
            // println!("pruning");
            break;
        }
    }
    unsafe {
        transposition_table_push(state, depth, &mut TRANSPOTABLENEGA);
    }
    // println!("alpha {}  beta {}", alpha, beta);
    state.heuristic = value;
    return value;
}

pub fn negascout(mut state: &mut State, depth: i32, mut alpha: i32, beta: i32, color: i8) -> i32 {
    if depth != 0 && state.available_move.len() == 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    // println!("current state: {:?} player to play {} current heuristic {} depth {}", state.current_move, state.player_to_play, state.heuristic, depth);
    if depth == 0 || state.available_move.len() == 0 || state_is_terminated(state) == true {
        state.heuristic = state.heuristic * color as i32;
        return state.heuristic * color as i32;
    }
    let mut value: i32;
    let len = state.available_move.len();
    for child in 0..len {
        if child == 0 {
            value = -negascout(
                &mut state.available_move[child],
                depth - 1,
                -beta,
                -alpha,
                -color,
            );
        } else {
            value = -negascout(
                &mut state.available_move[child],
                depth - 1,
                -alpha - 1,
                -alpha,
                -color,
            );

            if alpha < value && value < beta {
                value = -negascout(
                    &mut state.available_move[child],
                    depth - 1,
                    -beta,
                    -value,
                    -color,
                );
            }
        }
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            // println!("pruning");
            break;
        }
    }
    // println!("alpha {}  beta {}", alpha, beta);
    state.heuristic = alpha;
    return alpha;
}

pub fn negascout_with_transpotable(
    mut state: &mut State,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    color: i8,
) -> i32 {
    let tt_search: (bool, i32, i32);
    unsafe { tt_search = transposition_table_search(state, &TRANSPOTABLESCOUT) };
    if tt_search.0 == true && tt_search.1 >= depth {
        return tt_search.2;
    }
    if depth != 0 && state.available_move.len() == 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    // println!("current state: {:?} player to play {} current heuristic {} depth {}", state.current_move, state.player_to_play, state.heuristic, depth);
    if depth == 0 || state.available_move.len() == 0 || state_is_terminated(state) == true {
        state.heuristic = state.heuristic * color as i32;
        return state.heuristic * color as i32;
    }
    let mut value: i32;
    let len = state.available_move.len();
    for child in 0..len {
        if child == 0 {
            value = -negascout(
                &mut state.available_move[child],
                depth - 1,
                -beta,
                -alpha,
                -color,
            );
        } else {
            value = -negascout(
                &mut state.available_move[child],
                depth - 1,
                -alpha - 1,
                -alpha,
                -color,
            );

            if alpha < value && value < beta {
                value = -negascout(
                    &mut state.available_move[child],
                    depth - 1,
                    -beta,
                    -value,
                    -color,
                );
            }
        }
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            // println!("pruning");
            break;
        }
    }
    unsafe {
        transposition_table_push(state, depth, &mut TRANSPOTABLESCOUT);
    }
    // println!("alpha {}  beta {}", alpha, beta);
    state.heuristic = alpha;
    return alpha;
}

pub fn return_move(state: &mut State) -> (usize, i32) {
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
