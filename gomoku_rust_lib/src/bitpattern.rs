//! Methods to iter throught bit axes and finds patterns.

use crate::check_move::check_and_apply_capture;
use crate::check_move::check_blocker;
use crate::check_move::check_is_capturable;
use crate::check_move::check_is_double_triple;
use crate::check_move::check_is_unblockable_five;
use crate::check_move::check_one_bit_in_pattern;
use crate::data_struct::Bitboards;
use crate::data_struct::BoardStateInfo;
use crate::global_var;
use crate::global_var::BLOCKER;
use crate::global_var::PATTERN;
use crate::heuristic_ratios::HEURISTIC_PATTERN;

pub fn pattern_axes_dispatcher(
    board_state_info: &mut BoardStateInfo,
    bitboards: &mut Bitboards,
    axes: &[[u16; 4]; 2],
    pos: usize,
    player: i8,
) {
    // println!("player axe dispatcher {} pos {}", player, pos);
    if player == global_var::PLAYER_WHITE_NB {
        computing_move(board_state_info, &axes[0], &axes[1], bitboards, pos, player);
    } else if player == global_var::PLAYER_BLACK_NB {
        computing_move(board_state_info, &axes[1], &axes[0], bitboards, pos, player);
    }
}

fn computing_move(
    board_state_info: &mut BoardStateInfo,
    player_axe: &[u16; 4],
    opponent_axe: &[u16; 4],
    bitboards: &mut Bitboards,
    pos: usize,
    player: i8,
) {
    let axe_pattern: [[(usize, usize); 4]; 2];
    board_state_info.stone_captured =
        check_and_apply_capture(bitboards, player_axe, opponent_axe, pos, player);
    board_state_info.capturable = check_is_capturable(player_axe, opponent_axe);
    board_state_info.capturing = check_is_capturable(opponent_axe, player_axe);
    axe_pattern = pattern_axes_finder(bitboards, player_axe, opponent_axe, pos, player);
    board_state_info.pattern_axe = axe_pattern[0];
    board_state_info.blocker_axe = axe_pattern[1];
    if check_is_double_triple(axe_pattern[0]) {
        board_state_info.is_wrong_move = global_var::DOUBLE_TRIPLE_MOVE;
        return;
    }
    return_pattern_value(board_state_info, axe_pattern[0], pos, player);
    return_blocker_value(board_state_info, axe_pattern[1], pos, player);
}

#[allow(dead_code)]
fn return_pattern_value(
    board_state_info: &mut BoardStateInfo,
    axe_pattern: [(usize, usize); 4],
    pos: usize,
    player: i8,
) {
    let mut pat_value: i32 = 0;
    let mut move_to_win: i8 = 5;
    for pat in 0..axe_pattern.len() {
        if PATTERN[axe_pattern[pat].0].4 < move_to_win
            && axe_pattern[pat].1 != 2
            && axe_pattern[pat].1 != 3
        {
            move_to_win = PATTERN[axe_pattern[pat].0].4;
        }
        if axe_pattern[pat].0 == 0 && axe_pattern[pat].1 != 3 {
            board_state_info.is_winning = (pos, player);
        }
        if axe_pattern[pat].1 == 5 {
            board_state_info.pattern_value = 100000;
        } else if axe_pattern[pat].1 != 3 && axe_pattern[pat].1 != 2 {
            pat_value += HEURISTIC_PATTERN[axe_pattern[pat].0][axe_pattern[pat].1];
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
    //  println!("blocker on axe {:?}", axe_pattern);
    let mut pat_value: i32 = 0;
    let mut move_to_win: i8 = 5;
    for pat in 0..axe_pattern.len() {
        if axe_pattern[pat].1 == 2 {
            pat_value += HEURISTIC_PATTERN[axe_pattern[pat].0][0];
        } else if axe_pattern[pat].1 == 1 {
            pat_value += HEURISTIC_PATTERN[axe_pattern[pat].0][1];
        }
    }
    board_state_info.blocker_value += pat_value;
}

pub fn pattern_axes_finder(
    bitboards: &mut Bitboards,
    axes: &[u16; 4],
    blocker_axes: &[u16; 4],
    pos: usize,
    player: i8,
) -> [[(usize, usize); 4]; 2] {
    let mut return_pattern: [(usize, usize); 4] = [(0, 3), (0, 3), (0, 3), (0, 3)];
    let mut return_blocker: [(usize, usize); 4] = [(0, 3), (0, 3), (0, 3), (0, 3)];
    let mut is_blocked: usize;
    for axe_index in 0..axes.len() {
        // print_axe_value(axe);
        let mut player_axe = axes[axe_index];
        let mut blocker_axe = blocker_axes[axe_index];
        player_axe >>= 1;
        blocker_axe >>= 1;
        let mut found_pattern: (usize, usize) = (PATTERN.len(), 0);
        let mut found_blocker: (usize, usize) = (PATTERN.len(), 0);
        for l in 0..6 {
            let player_shifted = player_axe >> l;
            let blocker_shifted = blocker_axe >> l;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            is_blocked = 0;
            find_pattern(
                &mut return_pattern,
                player_casted,
                blocker_casted,
                bitboards,
                is_blocked,
                &mut found_pattern,
                axe_index,
                pos,
                player,
                l,
            );
            find_blocker(
                &mut return_blocker,
                blocker_casted,
                player_casted,
                bitboards,
                is_blocked,
                &mut found_blocker,
                axe_index,
                pos,
                player,
                l,
            );
            if return_pattern[0].1 == 5 {
                return [return_pattern, return_blocker];
            }
        }
    }
    //   println!("return_pat {:?}, return_blo {:?}", return_pattern, return_blocker);
    return [return_pattern, return_blocker];
}

fn find_pattern(
    return_pattern: &mut [(usize, usize); 4],
    player_casted: u8,
    blocker_casted: u8,
    bitboards: &mut Bitboards,
    mut is_blocked: usize,
    found_pattern: &mut (usize, usize),
    axe: usize,
    pos: usize,
    player: i8,
    l: usize,
) {
    for p in 0..PATTERN.len() {
        if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
            if p == 0 {
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
                    //  println!("pattern {}", PATTERN[p].4);
                    is_blocked = check_blocker(blocker_checker, blocker_casted, pos, b, p, l, axe);
                }
            }
            if is_blocked <= 2 && p < found_pattern.0 {
                found_pattern.0 = p;
                found_pattern.1 = is_blocked;
                // println!("{} found {} blocker", PATTERN[p].4, is_blocked);
                break;
            }
        }
    }
    if found_pattern.0 < PATTERN.len() && l <= PATTERN[found_pattern.0].1 {
        return_pattern[axe] = *found_pattern;
        // println!("PATTERN FOUND {} len {} l: {}", PATTERN[found_pattern.0].4,PATTERN[found_pattern.0].1 ,l);
    }
}

fn find_blocker(
    return_blocker: &mut [(usize, usize); 4],
    player_casted: u8,
    blocker_casted: u8,
    bitboards: &mut Bitboards,
    mut is_blocked: usize,
    found_blocker: &mut (usize, usize),
    axe: usize,
    pos: usize,
    player: i8,
    l: usize,
) {
    for p in 1..PATTERN.len() {
        if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
            for b in 0..BLOCKER.len() {
                if BLOCKER[b].1 == PATTERN[p].1 || BLOCKER[b].1 == 6 && ( p == 5 || p == 6 ) {
                    let blocker_checker: u8 = blocker_casted & BLOCKER[b].0;
                    //  println!("pattern {}", PATTERN[p].4);
                    is_blocked = check_blocker(blocker_checker, blocker_casted, pos, b, p, l, axe);
					if is_blocked != 0 {break;}
                }
            }
			println!("{} found {} blocker", PATTERN[p].3, is_blocked);
            if is_blocked > 0 && p < found_blocker.0 {
				if is_blocked == 3 {found_blocker.0 = 0;}
                else {found_blocker.0 = p;}
                found_blocker.1 = is_blocked;
                break;
            }
        }
    }
    if found_blocker.0 < PATTERN.len() && l <= PATTERN[found_blocker.0].1 {
        // println!("BLOCKER FOUND {} value {:?} len {} l: {}", PATTERN[found_blocker.0].4,found_blocker,PATTERN[found_blocker.0].1 ,l);
        return_blocker[axe] = *found_blocker;
    }
}
