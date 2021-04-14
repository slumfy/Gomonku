//! Heuristic of our AI.

use crate::check_pos_still_win;
use crate::checking_and_apply_bits_move;
use crate::global_var;
use crate::state::State;
#[derive(Debug)]
pub struct BoardStateInfo {
    pub is_wrong_move: i8,
    pub stone_captured: i8,
    pub flank: (i8,i8),
    pub pattern_value: i32,
    pub is_winning: (usize, i8),
}

pub fn heuristic(state: &mut State) -> i32 {
    let mut value: i32 = 0;
    let board_state_info: BoardStateInfo = checking_and_apply_bits_move(state);
    if board_state_info.is_wrong_move != global_var::VALID_MOVE {
        state.is_playable = board_state_info.is_wrong_move;
        return value;
    }
    if state.win_state.1 != 0 {
        if check_pos_still_win(state.bitboards, state.win_state.0, state.win_state.1) == true {
            if state.current_player == state.win_state.1 {
                value = global_var::HEURISTIC_MAX_VALUE;
            }
			else {value = global_var::HEURISTIC_MIN_VALUE;}
            return value;
        } else {
            state.win_state = (0, 0);
        }
    }
    if board_state_info.is_winning.1 != 0 {
        state.win_state = board_state_info.is_winning;
    }
	if board_state_info.pattern_value == global_var::HEURISTIC_MAX_VALUE {
		value = global_var::HEURISTIC_MAX_VALUE;
		return value;
	}
    value += board_state_info.pattern_value as i32;
    if board_state_info.flank.0 == 1 {
        value -= 100;
    }
	if board_state_info.flank.1 == 1 {
        value += 100;
    }
    value += board_state_info.stone_captured as i32 * 100;
    return value;
}
