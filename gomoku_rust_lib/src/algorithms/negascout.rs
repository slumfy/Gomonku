use crate::algorithms::algo_utils::update_max_depth;
use crate::algorithms::algo_utils::update_node_checked_count;
use crate::algorithms::algo_utils::update_pruning_count;
// use crate::algorithms::transpotable;
use crate::data_struct::State;
use crate::state::create_child;
use crate::state::state_is_terminated;
use std::cmp::Reverse;

pub fn negascout(mut state: &mut State, depth: i32, mut alpha: i64, beta: i64, color: i8) -> i64 {
    update_node_checked_count();
    update_max_depth(depth);
    if depth != 0 && state.available_move.len() == 0 {
        if depth == 1 {
            state.available_move = create_child(&mut state);
        } else {
            state.available_move = create_child(&mut state);
        }
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    // println!("current state: {:?} player to play {} current heuristic {} depth {}", state.current_move, state.player_to_play, state.heuristic, depth);
    if depth == 0 || state.available_move.len() == 0 || state_is_terminated(state) == true {
        state.heuristic = state.heuristic * color as i64;
        return state.heuristic * color as i64;
    }
    let mut value: i64;
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
            update_pruning_count();
            break;
        }
    }
    // println!("alpha {}  beta {}", alpha, beta);
    state.heuristic = alpha;
    return alpha;
}
