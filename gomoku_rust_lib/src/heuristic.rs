#[path = "check.rs"] mod check;
use crate::state::State;

pub fn heuristic(board: &mut Vec<Vec<i32>>, state: &State) -> i32 {
	let mut value: i32;
	value = check::check_win_position(board,state.player_to_play,state.current_move.0,state.current_move.1); //current alignement
	// state.heuristic += check_5_void_axe();
	// score += _free_flank()
	println!("value x {}", state.current_move.0);
	println!("value y {}", state.current_move.1);
	println!("value heuri {}", value);
	return value;
}