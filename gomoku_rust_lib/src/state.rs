//! ##This current crate possess the state struct and methods to creat state!

use crate::data_struct::Bitboards;
use crate::data_struct::State;
use crate::heuristic::heuristic;
use crate::BoardStateInfo;

use crate::search_space::get_search_box_bitboard;

pub fn create_new_state(
    bitboards: &mut Bitboards,
    player: i8,
    bit_current_move_pos: usize,
    white_captured_stone: i8,
    black_captured_stone: i8,
    win_state: (usize, i8),
	nb_move_to_win: i8
) -> State {
    let mut new_state = State {
        bitboards: bitboards.clone(),
        current_player: player,
        white_captured_stone: white_captured_stone,
        black_captured_stone: black_captured_stone,
        white_move_to_win: 5,
        black_move_to_win: 5,
        available_move: vec![],
        heuristic: 0,
        is_playable: 0,
        win_state: win_state,
        bit_current_move_pos: bit_current_move_pos,
		board_info: BoardStateInfo {
			player: player,
			is_wrong_move: 0,
			stone_captured: 0,
			capturable: false,
			capturing: false,
			pattern_value: 0,
			blocker_value: 0,
			is_winning: (0, 0),
			nb_move_to_win: 5,
			pattern_axe: [(0, 3), (0, 3), (0, 3), (0, 3)],
			blocker_axe: [(0, 3), (0, 3), (0, 3), (0, 3)],
		}
    };
	if player == 1 {new_state.black_move_to_win = nb_move_to_win;}
	else {new_state.white_move_to_win = nb_move_to_win;}
    return new_state;
}

pub fn create_child(state: &mut State) -> Vec<State> {
    let mut copy_bitboards: Bitboards;
    let mut childs_list: Vec<State>;
    let index_box: Vec<usize> = get_search_box_bitboard(&state.bitboards);
    let len = index_box.len();
    childs_list = Vec::new();
    for pos in 0..len {
        copy_bitboards = state.bitboards.clone();
        let bit_current_move_pos: usize = index_box[pos];
		let nb_move_to_win: i8 = if -state.current_player == 1 {state.black_move_to_win}
		else {state.white_move_to_win};
        let mut child = create_new_state(
            &mut copy_bitboards,
            -state.current_player,
            bit_current_move_pos,
            state.white_captured_stone,
            state.black_captured_stone,
            state.win_state,
			nb_move_to_win
        );
        child.heuristic = heuristic(&mut child);
        if child.is_playable == 0 {
            childs_list.push(child);
        }
    }
    return childs_list;
}

pub fn state_is_terminated(state: &mut State) -> bool {
    if state.white_captured_stone >= 10 || state.black_captured_stone >= 10 {
        return true;
    }
    if state.heuristic == 100000 {
        return true;
    }
    return false;
}
