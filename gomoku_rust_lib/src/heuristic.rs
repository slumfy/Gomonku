#[path = "check.rs"]
mod check;
use crate::state::State;

pub fn heuristic(board: &mut Vec<Vec<i8>>, state: &State) -> isize {
    let mut value = 0isize;
    let board_check = check::checking_move_biggest_alignment_and_stone_captured(state);
    // println!("boardcheck {:?}", board_check);
    value += (board_check["stone_captured"] as isize) * 100;
    value += (board_check["biggest_alignment"] as isize) * 10;
    //current alignement
    value += count_diff_piece(board, state) * 10;
    value += count_default_value(state.current_move);

    return value;
}

fn count_diff_piece(board: &mut Vec<Vec<i8>>, state: &State) -> isize {
    let mut player = 0i8;
    let mut advers = 0i8;
    for x in 0..board.len() {
        for y in 0..board.len() {
            if board[x as usize][y as usize] == state.player_to_play {
                player += 1;
            } else if board[x as usize][y as usize] != 0 {
                advers += 1;
            }
        }
    }
    return (player - advers) as isize;
}

fn count_default_value(current_move: (isize, isize)) -> isize {
    let default_table: [[isize; 19]; 19] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
        [0, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 6, 6, 6, 6, 6, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 6, 6, 6, 6, 6, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 1, 0],
        [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    return default_table[current_move.0 as usize][current_move.1 as usize];
}
