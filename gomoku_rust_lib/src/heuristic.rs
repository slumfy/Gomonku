#[path = "check.rs"] mod check;
use crate::state::State;

pub fn heuristic(board: &mut Vec<Vec<i32>>, state: &State) -> i32 {
	let mut value = 0i32;
	value += check::check_win_position(board,state.player_to_play,state.current_move.0,state.current_move.1); //current alignement
	value += count_diff_piece(board, state);

	// println!("value x {}", state.current_move.0);
	// println!("value y {}", state.current_move.1);
	// println!("value heuri {}", value);
	return value;
}

fn count_diff_piece(board: &mut Vec<Vec<i32>>,state: &State) -> i32 {
	let mut player = 0i32;
	let mut advers = 0i32;
	for x in 0..board.len() {
		for y in 0..board.len() {
			if board[x as usize][y as usize] == state.player_to_play {
				player += 1;
			}
			else if board[x as usize][y as usize] != 0 {
				advers += 1;
			}
		}
	}
	return player - advers
}