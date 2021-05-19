//! Methods to iter throught bit axes and finds patterns.

use crate::check_move::check_and_apply_capture;
use crate::check_move::check_blocker;
use crate::check_move::check_is_capturable;
use crate::check_move::check_is_double_triple;
use crate::check_move::check_pattern_blocker;
use crate::check_move::check_pattern_is_not_capturable;
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
    set_move_to_win_and_is_winning(board_state_info, axe_pattern[0], pos, player);
}

fn set_move_to_win_and_is_winning(
    board_state_info: &mut BoardStateInfo,
    axe_pattern: [(usize, usize); 4],
    pos: usize,
    player: i8,
) {
    let mut move_to_win: i8 = 5;
    for pattern_index in 0..axe_pattern.len() {
        // Checking if the pattern have less move to win and replace current move to win.
        if PATTERN[axe_pattern[pattern_index].0].4 < move_to_win
            && axe_pattern[pattern_index].1 != 2
            && axe_pattern[pattern_index].1 != 3
        {
            move_to_win = PATTERN[axe_pattern[pattern_index].0].4;
        }
        // Checking if player is winning
        if axe_pattern[pattern_index].0 == 0 && axe_pattern[pattern_index].1 != 3 {
            board_state_info.is_winning = (pos, player);
        }
    }
    board_state_info.nb_move_to_win = move_to_win;
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
    let mut pattern_is_blocked: usize;
    let mut blocker_is_blocking: usize;
    for axe_index in 0..axes.len() {
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
            pattern_is_blocked = 0;
            blocker_is_blocking = 0;
            find_pattern(
                &mut return_pattern,
                player_casted,
                blocker_casted,
                bitboards,
                pattern_is_blocked,
                &mut found_pattern,
                axe_index,
                pos,
                player,
                l,
            );
            // println!("return_pattern = {:?}, l = {}", return_pattern, l);
            find_blocker(
                &mut return_blocker,
                blocker_casted,
                player_casted,
                blocker_is_blocking,
                &mut found_blocker,
                axe_index,
                pos,
                l,
            );
            if return_pattern[0].1 == 5 {
                return [return_pattern, return_blocker];
            }
        }
    }
    // println!("return_pat {:?}, return_blo {:?}", return_pattern, return_blocker);
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
                if check_pattern_is_not_capturable(
                    bitboards,
                    pos - (l * global_var::AXE_MOUVEMENT_VALUE[axe]),
                    axe,
                    player,
                    5,
                ) == true
                {
                    *return_pattern = [(0, 5), (0, 5), (0, 5), (0, 5)];
                    break;
                }
            }
            for b in 0..BLOCKER.len() {
                if BLOCKER[b].1 == PATTERN[p].1 || BLOCKER[b].1 == 6 && (p == 5 || p == 6) {
                    let blocker_checker: u8 = blocker_casted & BLOCKER[b].0;
                    is_blocked =
                        check_pattern_blocker(blocker_checker, blocker_casted, pos, b, p, l, axe);
                    // println!("pattern {} is blocked {}", PATTERN[p].3, is_blocked);
                }
            }
            let mut stone_in_pattern = false;
            // println!("l {}", l);
            for pos in 0..5 {
                if global_var::STONE_POS_IN_PAT[p][pos] != 0x00
                    && (0x80 >> l) & global_var::STONE_POS_IN_PAT[p][pos]
                        == global_var::STONE_POS_IN_PAT[p][pos]
                {
                    // println!("l    {:08b}", l);
                    // println!("spip {:08b} {}, {}", global_var::STONE_POS_IN_PAT[p][pos],p,pos);
                    stone_in_pattern = true;
                    break;
                }
            }
            if is_blocked < 2
                && stone_in_pattern == true
                && found_pattern.0 != 0
                && (is_blocked > found_pattern.1 || p < found_pattern.0)
            {
                found_pattern.0 = p;
                found_pattern.1 = is_blocked;
                // println!("{} found {} blocker", PATTERN[p].3, is_blocked);
                break;
            }
        }
    }
    if return_pattern[0].1 != 5
        && found_pattern.0 < PATTERN.len()
        && l <= PATTERN[found_pattern.0].1
    {
        return_pattern[axe] = *found_pattern;
    }
}

fn find_blocker(
    return_blocker: &mut [(usize, usize); 4],
    player_casted: u8,
    blocker_casted: u8,
    mut is_blocked: usize,
    found_blocker: &mut (usize, usize),
    axe: usize,
    pos: usize,
    l: usize,
) {
    let mut blocker_count: usize;
    for p in 1..PATTERN.len() {
        if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
            for b in 0..BLOCKER.len() {
                if BLOCKER[b].1 == PATTERN[p].1 || BLOCKER[b].1 == 6 && (p == 5 || p == 6) {
                    let blocker_checker: u8 = blocker_casted & BLOCKER[b].0;
                    blocker_count =
                        check_blocker(blocker_checker, blocker_casted, pos, b, p, l, axe);
                    // println!("pattern {}, isblocked {}, blocker_count {} b = {}", PATTERN[p].3,is_blocked,blocker_count, b);
                    if is_blocked < blocker_count {
                        is_blocked = blocker_count;
                    }
                }
            }
            // println!("{} found {} blocker l = {} p = {}", PATTERN[p].3, is_blocked, l,p);
            if is_blocked == 3 || (is_blocked > 0 && p < found_blocker.0) {
                if is_blocked == 3 {
                    found_blocker.0 = 0;
                } else {
                    found_blocker.0 = p;
                }
                found_blocker.1 = is_blocked;
                break;
            }
        }
    }
    // println!("foundblocker: {:?}", found_blocker);
    if (found_blocker.0 < PATTERN.len() && l <= PATTERN[found_blocker.0].1) || is_blocked == 3 {
        // println!("BLOCKER FOUND {} value {:?} len {} l: {}", PATTERN[found_blocker.0].4,found_blocker,PATTERN[found_blocker.0].1 ,l);
        return_blocker[axe] = *found_blocker;
    }
}
