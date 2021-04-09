use crate::check_bits::check_is_wrong_move;
use crate::global_var;
use crate::state::State;

pub fn heuristic(state: &mut State) -> i32 {
    let value: i32;
    if check_is_wrong_move(state) == global_var::VALID_MOVE {
        value = 0;
    } else {
        value = -1000;
    }
    return value;
}
