pub mod algo_utils;
mod minimax;
mod negamax;
mod negascout;
mod transpotable;
use crate::data_struct::State;

pub fn negamax(state: &mut State, depth: i32, alpha: i64, beta: i64) -> i64 {
    return negamax::negamax(state, depth, alpha, beta);
}

pub fn minimax(
    state: &mut State,
    depth: i32,
    alpha: i64,
    beta: i64,
    maximizingplayer: bool,
) -> i64 {
    return minimax::minimax(state, depth, alpha, beta, maximizingplayer);
}

pub fn negascout(state: &mut State, depth: i32, alpha: i64, beta: i64, color: i8) -> i64 {
    return negascout::negascout(state, depth, alpha, beta, color);
}

pub fn return_move(state: &mut State) -> (usize, i64) {
    return algo_utils::return_move(state);
}

pub fn reset_tt_table() {
	transpotable::clear_tt_table();
}