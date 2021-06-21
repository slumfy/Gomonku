pub mod algo_utils;
mod negamax;
// mod transpotable;
use crate::data_struct::State;

pub fn negamax(state: &mut State, depth: i32) -> i64 {
    return negamax::negamax(state, depth);
}

pub fn return_move(state: &mut State) -> (usize, i64) {
    return algo_utils::return_move(state);
}

// pub fn reset_tt_table() {
//     transpotable::clear_tt_table();
// }
