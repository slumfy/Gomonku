use crate::global_var::BOARD_PATTERN_STATE;
use crate::data_struct::State;
use crate::global_var;

pub fn update_board_pattern_state(state: &State) {
	let player: i8 = state.current_player;
    let pattern_axe: [(usize, usize); 4] = state.board_info.pattern_axe;
    let blocker_axe: [(usize, usize); 4] = state.board_info.blocker_axe;
	println!("PATERN {:?}", pattern_axe);
	println!("PATERN {:?}", blocker_axe);
	unsafe {
		if player == global_var::PLAYER_WHITE_NB {
			for pattern in 0..4 {
				if pattern_axe[pattern].1 != 3 && pattern_axe[pattern].1 != 5 {
					BOARD_PATTERN_STATE.white_patterns[pattern_axe[pattern].0][pattern_axe[pattern].1] += 1;
					if pattern_axe[pattern].0 == 5 || pattern_axe[pattern].0 == 5 || pattern_axe[pattern].0 == 5 {
						BOARD_PATTERN_STATE.white_patterns[8][pattern_axe[pattern].1] -= 1;
					}
					if pattern_axe[pattern].0 == 1 || pattern_axe[pattern].0 == 2 || pattern_axe[pattern].0 == 3 {
						BOARD_PATTERN_STATE.white_patterns[5][pattern_axe[pattern].1] -= 1;
					}
					if pattern_axe[pattern].0 == 4 {
						BOARD_PATTERN_STATE.white_patterns[2][pattern_axe[pattern].1] -= 2;
					}
				}
			}
			for blocker in 0..4 {
				if blocker_axe[blocker].1 != 3 {
					BOARD_PATTERN_STATE.black_patterns[blocker_axe[blocker].0][blocker_axe[blocker].1] += 1;
					BOARD_PATTERN_STATE.black_patterns[blocker_axe[blocker].0][blocker_axe[blocker].1 - 1] -= 1;
				}
			}
		}
		else {
			for pattern in 0..4 {
				if pattern_axe[pattern].1 != 3 && pattern_axe[pattern].1 != 5 {
					BOARD_PATTERN_STATE.black_patterns[pattern_axe[pattern].0][pattern_axe[pattern].1] += 1;
					if pattern_axe[pattern].0 == 5 || pattern_axe[pattern].0 == 5 || pattern_axe[pattern].0 == 5 {
						BOARD_PATTERN_STATE.black_patterns[8][pattern_axe[pattern].1] -= 1;
					}
					if pattern_axe[pattern].0 == 1 || pattern_axe[pattern].0 == 2 || pattern_axe[pattern].0 == 3 {
						BOARD_PATTERN_STATE.black_patterns[5][pattern_axe[pattern].1] -= 1;
					}
					if pattern_axe[pattern].0 == 4 {
						BOARD_PATTERN_STATE.black_patterns[2][pattern_axe[pattern].1] -= 2;
					}
				}
			}
			for blocker in 0..4 {
				if blocker_axe[blocker].1 != 3 {
					BOARD_PATTERN_STATE.white_patterns[blocker_axe[blocker].0][blocker_axe[blocker].1] += 1;
					BOARD_PATTERN_STATE.white_patterns[blocker_axe[blocker].0][blocker_axe[blocker].1 - 1] -= 1;
				}
			}
		}
	}
}

pub fn print_board_pattern_state() {
	unsafe {
		println!("BOARD PATTERN STATE");
		println!("WHITE_PATTERNS {:?}", BOARD_PATTERN_STATE.white_patterns);
		println!("BLACK_PATTERNS {:?}", BOARD_PATTERN_STATE.black_patterns);
	}
}