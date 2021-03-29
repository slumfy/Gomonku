#[path = "check.rs"]
mod check;
use crate::state::State;
use crate::check::check_alignment_for_given_pos;

pub fn heuristic(state: &mut State) -> i32 {
    let mut value = 0i32;
    let board_check = check::checking_move_biggest_alignment_and_stone_captured(state);// current alignement and current eat_value
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
	if state.win_move.len() > 0 {
		for winmove in 0..state.win_move.len() {
			if check_alignment_for_given_pos(&state,state.win_move[winmove].0.0,state.win_move[winmove].0.1,state.win_move[winmove].1) == true {
				if state.win_move[winmove].1 == state.player_to_play {
					state.win_state += 1;
					value += 1000;
				}
				else {
					state.win_state += -1;
					value -= 1000;
				}
				break ;
			}
	}
	}
    value += board_check["biggest_alignment"] as i32;
	if board_check["biggest_alignment"] == 5 {
		state.win_move.push((state.current_move,state.player_to_play));
	}

    return value;
}