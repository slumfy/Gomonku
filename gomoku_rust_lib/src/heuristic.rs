#[path = "check.rs"]
mod check;
use crate::state::State;

pub fn heuristic(state: &mut State) -> i32 {
    let mut value = 0i32;
    let board_check = check::checking_move_biggest_alignment_and_stone_captured(state);
	if state.player_to_play == 1 {
		state.white_eat_value += board_check["stone_captured"];
		if state.white_eat_value >= 10 {
			value += 1000
		}
		else {
		value += state.white_eat_value as i32 * state.white_eat_value as i32;
	}
	}
	else {
		state.black_eat_value += board_check["stone_captured"];
		if state.black_eat_value >= 10 {
			value += 1000
		}
		else {
		value += state.black_eat_value as i32 * state.black_eat_value as i32;
		}
	}
	// println!("boardcheck {:?}", board_check);
    value += board_check["biggest_alignment"] as i32;
	if board_check["biggest_alignment"] == 5 {
		state.win_state.push((state.current_move,state.player_to_play));
	}

    return value;
}