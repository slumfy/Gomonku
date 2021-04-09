use crate::global_var;
use crate::state::State;

pub fn heuristic(state: &mut State) -> i32 {
    let value: i32;
    value = state.bit_current_move_pos as i32 + global_var::DEEP;

    return value;
}
