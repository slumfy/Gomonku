#[path = "check.rs"] mod check;
use crate::state::State;

pub fn heuristic(board: &mut Vec<Vec<i32>>, state: &State) -> i32 {
	let mut value = 0i32;
	value += check::check_win_position(board,state.player_to_play,state.current_move.0,state.current_move.1); //current alignement
	value += check::check_three_position(board,state.player_to_play,state.current_move.0,state.current_move.1);
	value += count_diff_piece(board, state) * 10;
	value += count_default_value(state.current_move);

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

fn count_default_value(current_move: (i32,i32)) -> i32 {
let default_table: [[i32; 19]; 19] = [[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
							[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
							[0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0],
							[0,1,2,3,3,3,3,3,3,3,3,3,3,3,3,3,2,1,0],
							[0,1,2,3,4,4,4,4,4,4,4,4,4,4,4,3,2,1,0],
							[0,1,2,3,4,5,5,5,5,5,5,5,5,5,4,3,2,1,0],
							[0,1,2,3,4,5,6,6,6,6,6,6,6,5,4,3,2,1,0],
							[0,1,2,3,4,5,6,7,7,7,7,7,6,5,4,3,2,1,0],
							[0,1,2,3,4,5,6,7,8,8,8,7,6,5,4,3,2,1,0],
							[0,1,2,3,4,5,6,7,8,9,8,7,6,5,4,3,2,1,0],
							[0,1,2,3,4,5,6,7,8,8,8,7,6,5,4,3,2,1,0],
							[0,1,2,3,4,5,6,7,7,7,7,7,6,5,4,3,2,1,0],
							[0,1,2,3,4,5,6,6,6,6,6,6,6,5,4,3,2,1,0],
							[0,1,2,3,4,5,5,5,5,5,5,5,5,5,4,3,2,1,0],
							[0,1,2,3,4,4,4,4,4,4,4,4,4,4,4,3,2,1,0],
							[0,1,2,3,3,3,3,3,3,3,3,3,3,3,3,3,2,1,0],
							[0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0],
							[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
							[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]];
	return default_table[current_move.0 as usize][current_move.1 as usize];
}