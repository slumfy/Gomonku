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
    current_move_pos: usize,
    total_white_captured_stone: i8,
    total_black_captured_stone: i8,
    all_depth_white_captured_stone_value: i64,
    all_depth_black_captured_stone_value: i64,
    win_state: (usize, i8),
    nb_move_to_win: i8,
) -> State {
    let mut new_state = State {
        bitboards: bitboards.clone(),
        current_player: player,
        total_white_captured_stone: total_white_captured_stone,
        total_black_captured_stone: total_black_captured_stone,
        all_depth_white_captured_stone_value: all_depth_white_captured_stone_value,
        all_depth_black_captured_stone_value: all_depth_black_captured_stone_value,
        white_move_to_win: 5,
        black_move_to_win: 5,
        available_move: vec![],
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
            nb_move_to_win: 5,
            axe_free_value: [false, false, false, false],
            pattern_axe: [(0, 3), (0, 3), (0, 3), (0, 3)],
            blocker_axe: [(0, 3), (0, 3), (0, 3), (0, 3)],
        },
        axes: [[0, 0, 0, 0], [0, 0, 0, 0]],
		max_eat_next_move: 0,
    };
    if player == 1 {
        new_state.black_move_to_win = nb_move_to_win;
    } else {
        new_state.white_move_to_win = nb_move_to_win;
    }
    return new_state;
}

pub fn create_child(state: &mut State) -> Vec<State> {
    let mut copy_bitboards: Bitboards;
    let mut childs_list: Vec<State>;
    let index_box: Vec<usize> = get_search_box_bitboard(&state.bitboards);
    let len = index_box.len();
    childs_list = Vec::new();
    let mut saved_child = create_new_state(
        &mut state.bitboards.clone(),
        -state.current_player,
        index_box[0],
        state.total_white_captured_stone,
        state.total_black_captured_stone,
        state.all_depth_white_captured_stone_value,
        state.all_depth_black_captured_stone_value,
        state.win_state,
        0,
    );
    saved_child.heuristic = heuristic_ratios::HEURISTIC_MIN_VALUE - 1;
    let mut stone_threaten: u32 = 0;
    for pos in 0..len {
        copy_bitboards = state.bitboards.clone();
        let current_move_pos: usize = index_box[pos];
        let nb_move_to_win: i8 = if -state.current_player == 1 {
            state.black_move_to_win
        } else {
            state.white_move_to_win
        };
        let mut child = create_new_state(
            &mut copy_bitboards,
            -state.current_player,
            current_move_pos,
            state.total_white_captured_stone,
            state.total_black_captured_stone,
            state.all_depth_white_captured_stone_value,
            state.all_depth_black_captured_stone_value,
            state.win_state,
            nb_move_to_win,
        );
        child.heuristic = heuristic(&mut child);
		if child.board_info.stone_captured > state.max_eat_next_move {
			state.max_eat_next_move = child.board_info.stone_captured;
		}
        if child.is_playable == global_var::VALID_MOVE
            && (saved_child.current_move_pos == 0 || child.heuristic > saved_child.heuristic)
        {
            saved_child = child.clone();
        }
        //TEST improved selection
        let mut playable: bool = false;
        if child.heuristic == heuristic_ratios::HEURISTIC_MAX_VALUE {
            childs_list.clear();
            childs_list.push(child);
            break;
        }
        if child.is_playable == 0 {
            for x in 0..4 {
                if child.board_info.pattern_axe[x].1 != 3 {
                    playable = true;
                }
                if child.board_info.blocker_axe[x].1 != 3 {
                    playable = true;
                }
            }
        }
        if playable == true {
            childs_list.push(child);
        }
    }
    if childs_list.len() == 0 {
        childs_list.push(saved_child);
    }
	return(childs_list);
    // childs_list.sort_by_key(|d| Reverse(d.heuristic));
    // let mut new_list: Vec<State> = Vec::new();
    // let mut len = childs_list.len();
    // if len > 10 {
    //     len = 10;
    // }
    // for child in 0..len {
    //     new_list.push(childs_list[child].clone());
    // }
    // return new_list;
}

pub fn state_is_terminated(state: &mut State) -> bool {
    if state.total_white_captured_stone >= 10 || state.total_black_captured_stone >= 10 {
        return true;
    }
	let mut opponent_capture_score: i8 = 0;
	if state.current_player == global_var::PLAYER_WHITE_NB {
		opponent_capture_score = state.total_black_captured_stone;
	}
	else {
		opponent_capture_score = state.total_white_captured_stone;
	}
    if state.board_info.pattern_axe[0].1 == 5 {
		if state.available_move.len() == 0 {
			state.available_move = create_child(state);
		}
		if opponent_capture_score + state.max_eat_next_move >= 10 {
			return false;
		}
        return true;
    }
    return false;
}
