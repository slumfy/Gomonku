//! Methods to iter throught bit axes and finds patterns.

use crate::bitboards::Bitboards;
use crate::check_move::check_and_apply_capture;
use crate::check_move::check_blocker;
use crate::check_move::check_flank;
use crate::check_move::check_is_double_triple;
use crate::check_move::check_is_unblockable_five;
use crate::global_var;
use crate::heuristic::BoardStateInfo;
use crate::patterns::BLOCKER;
use crate::patterns::PATTERN;

pub fn pattern_axes_dispatcher(
    board_state_info: &mut BoardStateInfo,
    bitboards: &mut Bitboards,
    axes: &[[u16; 4]; 2],
    pos: usize,
    player: i8,
) {
    let axe_pattern: [[(usize, usize); 4]; 2];
    if player == global_var::PLAYER_WHITE_NB {
        // println!("white player pattern in row:");
        // check and apply capture
        board_state_info.stone_captured =
            check_and_apply_capture(bitboards, &axes[0], &axes[1], pos, player);
        board_state_info.flank = check_flank(&axes[0], &axes[1]);
        axe_pattern = pattern_axes_finder(bitboards, &axes[0], &axes[1], pos, player);
        return_pattern_value(board_state_info, axe_pattern[0], pos, player);
		// return_blocker_value(board_state_info, axe_pattern[1], pos, player);
        if check_is_double_triple(axe_pattern[0]) {
            board_state_info.is_wrong_move = global_var::DOUBLE_TRIPLE_MOVE;
        }
    } else if player == global_var::PLAYER_BLACK_NB {
        // println!("black player pattern in row:");
        board_state_info.stone_captured =
            check_and_apply_capture(bitboards, &axes[1], &axes[0], pos, player);
        board_state_info.flank = check_flank(&axes[1], &axes[0]);
        axe_pattern = pattern_axes_finder(bitboards, &axes[1], &axes[0], pos, player);
        return_pattern_value(board_state_info, axe_pattern[0], pos, player);
		// return_blocker_value(board_state_info, axe_pattern[1], pos, player);
        if check_is_double_triple(axe_pattern[0]) {
            board_state_info.is_wrong_move = global_var::DOUBLE_TRIPLE_MOVE;
        }
    }
}

fn return_pattern_value(
    board_state_info: &mut BoardStateInfo,
    axe_pattern: [(usize, usize); 4],
    pos: usize,
    player: i8,
) {
    // println!("pattern on axe {:?}", axe_pattern);
    let mut pat_value: i32 = 0;
	let mut move_to_win: i8 = 5;
    for pat in 0..axe_pattern.len() {
		if PATTERN[axe_pattern[pat].0].5 < move_to_win  && axe_pattern[pat].1 == 0 {
			// println!("MOVE WIN NB {}", PATTERN[axe_pattern[pat].0].5);
			move_to_win = PATTERN[axe_pattern[pat].0].5;
		}
        if axe_pattern[pat].0 == 0 && axe_pattern[pat].1 != 3 {
            board_state_info.is_winning = (pos, player);
        }
        if axe_pattern[pat].1 == 5 {
            board_state_info.pattern_value = 100000;
        } else if axe_pattern[pat].1 == 1 {
            pat_value += PATTERN[axe_pattern[pat].0].3;
        } else if axe_pattern[pat].1 == 0 {
            pat_value += PATTERN[axe_pattern[pat].0].3 * 10;
        }
    }
	board_state_info.nb_move_to_win = move_to_win;
    board_state_info.pattern_value += pat_value;
}

fn return_blocker_value(
    board_state_info: &mut BoardStateInfo,
    axe_pattern: [(usize, usize); 4],
    pos: usize,
    player: i8,
) {
    // println!("pattern on axe {:?}", axe_pattern);
    let mut pat_value: i32 = 0;
	let mut move_to_win: i8 = 5;
    for pat in 0..axe_pattern.len() {
        if axe_pattern[pat].1 == 2 {
            pat_value += PATTERN[axe_pattern[pat].0].3 * 10;
        } else if axe_pattern[pat].1 == 1 {
            pat_value += PATTERN[axe_pattern[pat].0].3;
        }
    }
    board_state_info.pattern_value += pat_value;
}

fn pattern_axes_finder(
    bitboards: &mut Bitboards,
    axes: &[u16; 4],
    blocker_axes: &[u16; 4],
    pos: usize,
    player: i8,
) -> [[(usize, usize); 4]; 2] {
    let mut return_pattern: [(usize, usize); 4] = [(0, 3), (0, 3), (0, 3), (0, 3)];
	let mut return_blocker: [(usize, usize); 4] = [(0, 3), (0, 3), (0, 3), (0, 3)];
    let mut is_blocked: usize;
    for axe in 0..axes.len() {
        // print_axe_value(axe);
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        let mut found_pattern: (usize, usize) = (PATTERN.len(), 0);
        player_axe >>= 1;
        blocker_axe >>= 1;
        for l in 0..6 {
            let player_shifted = player_axe >> l;
            let blocker_shifted = blocker_axe >> l;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            is_blocked = 0;
            find_patter(&mut return_pattern, player_casted, blocker_casted, bitboards, is_blocked, found_pattern, axe, pos, player, l);
			find_blocker(&mut return_blocker, player_casted, blocker_casted, bitboards, is_blocked, found_pattern, axe, pos, player, l);
			if return_pattern[0].1 == 5 {
				return [return_pattern, return_blocker];
			}
        }

    }
    return [return_pattern, return_blocker];
}

fn find_patter(
	return_pattern: &mut [(usize, usize); 4],
	player_casted: u8,
	blocker_casted: u8,
	bitboards: &mut Bitboards,
	mut is_blocked: usize,
	mut found_pattern: (usize, usize),
	axe: usize,
	pos: usize,
	player: i8,
	l: usize) {
	for p in 0..PATTERN.len() {
		if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
			if p == 0 {
				// println!("FIVE");
				if check_is_unblockable_five(bitboards, pos - l, axe, player) == true {
					*return_pattern = [(0, 5), (0, 5), (0, 5), (0, 5)];
					break;
				} else {
					found_pattern.0 = 0;
					found_pattern.1 = 0;
					break;
				}
			}
			for b in 0..BLOCKER.len() {
				if BLOCKER[b].1 == PATTERN[p].1 {
					let blocker_checker: u8 = blocker_casted & BLOCKER[b].0;
					// println!("pattern {}", PATTERN[p].4);
					is_blocked =
						check_blocker(blocker_checker, blocker_casted, pos, b, p, l, axe);
				}
			}
			if is_blocked < 2 && p < found_pattern.0 {
				found_pattern.0 = p;
				found_pattern.1 = is_blocked;
				// println!("{} found {} blocker", PATTERN[p].4, is_blocked);
				break;
			}
		}
	}
	if found_pattern.0 < PATTERN.len() {
		return_pattern[axe] = found_pattern;
		// println!("PATTERN FOUND {}", PATTERN[found_pattern.0].4,);
	}
}

fn find_blocker(
	return_blocker: &mut [(usize, usize); 4],
	player_casted: u8,
	blocker_casted: u8,
	bitboards: &mut Bitboards,
	mut is_blocked: usize,
	mut found_pattern: (usize, usize),
	axe: usize,
	pos: usize,
	player: i8,
	l: usize) {
	for p in 1..PATTERN.len() {
		if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
			for b in 0..BLOCKER.len() {
				if BLOCKER[b].1 == PATTERN[p].1 {
					let blocker_checker: u8 = blocker_casted & BLOCKER[b].0;
					// println!("pattern {}", PATTERN[p].4);
					is_blocked =
						check_blocker(blocker_checker, blocker_casted, pos, b, p, l, axe);
				}
			}
			if is_blocked > 0 && p < found_pattern.0 {
				found_pattern.0 = p;
				found_pattern.1 = is_blocked;
				// println!("{} found {} blocker", PATTERN[p].4, is_blocked);
				break;
			}
		}
	}
	if found_pattern.0 < PATTERN.len() {
		return_blocker[axe] = found_pattern;
	}
}