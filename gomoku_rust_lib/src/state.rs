//! ##This current crate possess the state struct and methods to creat state!

use crate::data_struct::Bitboards;
use crate::data_struct::State;
use crate::global_var;
use crate::heuristic::heuristic;
use crate::heuristic_ratios;
use crate::BoardStateInfo;
use std::cmp::Reverse;

use crate::search_space::get_search_box_bitboard;

pub fn create_new_state(
    bitboards: &mut Bitboards,
    player: i8,
	color: i8,
    current_move_pos: usize,
    total_white_captured_stone: i8,
    total_black_captured_stone: i8,
    all_depth_white_captured_stone_value: i64,
    all_depth_black_captured_stone_value: i64,
    win_state: (usize, i8),
	previous_heuristic: i64
) -> State {
    let new_state = State {
        bitboards: bitboards.clone(),
        current_player: player,
		color: color,
        total_white_captured_stone: total_white_captured_stone,
        total_black_captured_stone: total_black_captured_stone,
        all_depth_white_captured_stone_value: all_depth_white_captured_stone_value,
        all_depth_black_captured_stone_value: all_depth_black_captured_stone_value,
        available_move: vec![],
		previous_heuristic: previous_heuristic,
        heuristic: 0,
        is_playable: 0,
        win_state: win_state,
        current_move_pos: current_move_pos,
        board_info: BoardStateInfo {
            player: player,
            is_wrong_move: 0,
            stone_captured: 0,
            capturable: false,
            capturing: false,
            captured_pattern_blocking_value: 0,
            pattern_value: 0,
            blocker_value: 0,
            is_winning: (0, 0),
            axe_free_value: [false, false, false, false],
            pattern_axe: [(0, 3), (0, 3), (0, 3), (0, 3)],
            blocker_axe: [(0, 3), (0, 3), (0, 3), (0, 3)],
        },
        axes: [[0, 0, 0, 0], [0, 0, 0, 0]],
        max_capturing_stone_next_move: 0,
    };
    return new_state;
}

pub fn create_child(state: &mut State) -> Vec<State> {
    let mut copy_bitboards: Bitboards;
    let mut childs_list: Vec<State>;
    let index_box: Vec<usize> = get_search_box_bitboard(&state.bitboards);
    let len = index_box.len();
    childs_list = Vec::new();
    // let mut saved_child = create_new_state(
    //     &mut state.bitboards.clone(),
    //     -state.current_player,
	// 	-state.color,
    //     index_box[0],
    //     state.total_white_captured_stone,
    //     state.total_black_captured_stone,
    //     state.all_depth_white_captured_stone_value,
    //     state.all_depth_black_captured_stone_value,
    //     state.win_state,
	// 	state.heuristic,
    // );
    // saved_child.heuristic = heuristic_ratios::MIN_VALUE;
    for pos in 0..len {
        copy_bitboards = state.bitboards.clone();
        let current_move_pos: usize = index_box[pos];
        let mut child = create_new_state(
            &mut copy_bitboards,
            -state.current_player,
			-state.color,
            current_move_pos,
            state.total_white_captured_stone,
            state.total_black_captured_stone,
            state.all_depth_white_captured_stone_value,
            state.all_depth_black_captured_stone_value,
            state.win_state,
			state.heuristic,
        );
        child.heuristic = heuristic(&mut child);
		if child.color == -1 && state.heuristic == heuristic_ratios::MAX_VALUE {
			child.heuristic == heuristic_ratios::MAX_VALUE;
		}
		else if child.color == 1 {
			child.heuristic = child.heuristic + child.previous_heuristic;
		}
		else {
			child.heuristic = -child.heuristic + child.previous_heuristic;
		}
        if child.board_info.stone_captured > state.max_capturing_stone_next_move {
            state.max_capturing_stone_next_move = child.board_info.stone_captured;
        }
        // if child.is_playable == global_var::VALID_MOVE
        //     && (saved_child.current_move_pos == 0 || child.heuristic > saved_child.heuristic)
        // {
        //     saved_child = child.clone();
        // }
        // let mut playable: bool = false;
        if child.heuristic == heuristic_ratios::MAX_VALUE {
            childs_list.clear();
            childs_list.push(child);
            break;
        }
        // if child.is_playable == 0 {
        //     for x in 0..4 {
        //         if child.board_info.pattern_axe[x].1 != 3 {
        //             playable = true;
        //         }
        //         if child.board_info.blocker_axe[x].1 != 3 {
        //             playable = true;
        //         }
        //     }
        // }
        if child.is_playable == global_var::VALID_MOVE {
            childs_list.push(child);
        }
    }
	return childs_list;
    // if childs_list.len() == 0 {
    //     childs_list.push(saved_child);
    // }
    // childs_list.sort_by_key(|d| Reverse(d.heuristic));
    // let mut new_list: Vec<State> = Vec::new();
    // let mut len = childs_list.len();
    // if len > 7 {
    //     len = 7;
    // }
    // for child in 0..len {
    //     new_list.push(childs_list[child].clone());
    // }
    // return new_list;
}

pub fn state_is_terminated(state: &mut State) -> bool {
    if (state.current_player == global_var::PLAYER_WHITE_NB
        && state.total_white_captured_stone >= 10)
        || (state.current_player == global_var::PLAYER_BLACK_NB
            && state.total_black_captured_stone >= 10)
    {
        return true;
    } else if state.board_info.pattern_axe[0].1 == 5 {
        return true;
    }
    return false;
}
