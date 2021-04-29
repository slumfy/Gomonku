//! Four algorithms use by the AI.

use crate::data_struct::State;
use crate::global_var;
use crate::heuristic_ratios;
use crate::print::print_heuristic_table;
use crate::state::create_child;
use crate::state::state_is_terminated;
use std::cmp::Reverse;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

static mut TRANSPOTABLENEGA: Vec<Transpotablenode> = vec![];
static mut TRANSPOTABLESCOUT: Vec<Transpotablenode> = vec![];

pub fn negamax(mut state: &mut State, depth: i32, mut alpha: i32, beta: i32, color: i8) -> i32 {
    update_max_depth(depth);
    if depth == 0 || state_is_terminated(state) == true {
        return state.heuristic * color as i32;
    }
    if depth != 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    // println!("NEGAMAX player {}  color {} state: {}", state.current_player, color, state.bit_current_move_pos);
    if depth == global_var::DEPTH {
        for child in 0..state.available_move.len() {
            println!(
                "child {} heuristic {} pos {}",
                child,
                state.available_move[child].heuristic,
                state.available_move[child].bit_current_move_pos
            );
        }
    }
    let mut value: i32 = heuristic_ratios::HEURISTIC_MIN_VALUE;
    for child_index in 0..state.available_move.len() {
        let negamax_value = -negamax(
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

pub fn negamax_with_transpotable(
    mut state: &mut State,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    color: i8,
) -> i32 {
    let tt_search: (bool, i32, i32);
    unsafe { tt_search = transposition_table_search(state, &TRANSPOTABLENEGA) };
    if tt_search.0 == true && tt_search.1 >= depth {
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
            // println!("pruning");
            break;
        }
    }
    unsafe {
        transposition_table_push(state, depth, &mut TRANSPOTABLENEGA);
    }
    // // println!("alpha {}  beta {}", alpha, beta);
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
    // // println!("alpha {}  beta {}", alpha, beta);
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
    // // println!("alpha {}  beta {}", alpha, beta);
    state.heuristic = alpha;
    return alpha;
}

pub fn return_move(state: &mut State) -> (usize, i32) {
    print_heuristic_table(state);
    unsafe {
        println!("MAX DEPTH: {}", global_var::MAX_DEPTH_REACH);
    }
    for child in 0..state.available_move.len() {
        println!(
            "child {} heuristic {} pos {}",
            child,
            state.available_move[child].heuristic,
            state.available_move[child].bit_current_move_pos
        );
    }
    state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    println!(
        "boardstate of returning move {} : {:?}",
        state.available_move[0].bit_current_move_pos, state.available_move[0].board_info
    );
    return (
        (state.available_move[0].bit_current_move_pos),
        state.available_move[0].heuristic,
    );
}

struct Transpotablenode {
    hash: u64,
    depth: i32,
    value: i32,
}

fn transposition_table_push(state: &State, depth: i32, transpo_table: &mut Vec<Transpotablenode>) {
    let mut hash = DefaultHasher::new();
    state.bitboards.hash(&mut hash);
    let state_hash: u64 = hash.finish();
    let new_table_node = Transpotablenode {
        hash: state_hash,
        depth: depth,
        value: state.heuristic,
    };
    transpo_table.push(new_table_node);
}

unsafe fn transposition_table_search(
    state: &State,
    transpo_table: &Vec<Transpotablenode>,
) -> (bool, i32, i32) {
    let mut hash = DefaultHasher::new();
    state.bitboards.hash(&mut hash);
    let state_hash: u64 = hash.finish();
    let len = transpo_table.len();
    for node in 0..len {
        if transpo_table[node].hash == state_hash {
            return (true, transpo_table[node].depth, transpo_table[node].value);
        }
    }
    return (false, 0, 0);
}
