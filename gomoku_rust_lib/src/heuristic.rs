use crate::check_bits::check_is_wrong_move;
use crate::global_var;
use crate::state::State;
use crate::HashMap;
use crate::checking_and_apply_bits_move;

pub fn heuristic(state: &mut State) -> i32 {
    let value: i32;
	let board_check: HashMap<String, i8> = checking_and_apply_bits_move(state);
    if board_check["is_wrong_move"] == global_var::VALID_MOVE {
        value = 0;
    } else {
        value = -1000;
    }
    return value;
}
