//! Methods to check moves and bits.

use crate::bitboard_operations::apply_bit;
use crate::bitboard_operations::apply_capture;
use crate::bitboards::create_bitboards_from_vec;
use crate::bitboards::create_bits_axes_from_pos;
use crate::bitboards::get_bits_in_bitboard_from_pos;
use crate::bitpattern::pattern_axes_dispatcher;
use crate::data_struct::Bitboards;
use crate::data_struct::BoardStateInfo;
use crate::data_struct::State;
use crate::global_var;
use crate::global_var::BLOCKER;
use crate::global_var::CAPTURE_PATTERN;
use crate::global_var::PATTERN;
// use crate::print::print_axes;
use crate::utils::is_on_axe;
use pyo3::prelude::*;

pub fn check_stone_color(pos: usize, bitboards: &Bitboards) -> i8 {
    if get_bits_in_bitboard_from_pos(pos, &bitboards.white_board) != 0 {
        return global_var::PLAYER_WHITE_NB;
    } else if get_bits_in_bitboard_from_pos(pos, &bitboards.black_board) != 0 {
        return global_var::PLAYER_BLACK_NB;
    } else {
        return 0;
    }
}

fn check_overlapping_stone(pos: usize, bitboards: &Bitboards) -> bool {
    if get_bits_in_bitboard_from_pos(pos, &bitboards.white_board) != 0
        || get_bits_in_bitboard_from_pos(pos, &bitboards.black_board) != 0
    {
        return false;
    }
    return true;
}

pub fn check_is_wrong_move(state: &State) -> i8 {
    if !check_overlapping_stone(state.bit_current_move_pos, &state.bitboards) {
        return global_var::OVERLAPPING_STONE_MOVE;
    }
    return global_var::VALID_MOVE;
}

pub fn checking_and_apply_bits_move(state: &mut State) -> BoardStateInfo {
    let mut board_state_info = BoardStateInfo {
        player: state.current_player,
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
    };

    board_state_info.is_wrong_move = check_is_wrong_move(state);
    if board_state_info.is_wrong_move != global_var::VALID_MOVE {
        return board_state_info;
    } else {
        apply_bit(
            &mut state.bitboards,
            state.bit_current_move_pos as usize,
            state.current_player,
        );
        let axes = create_bits_axes_from_pos(state.bit_current_move_pos, &state.bitboards);
        pattern_axes_dispatcher(
            &mut board_state_info,
            &mut state.bitboards,
            &axes,
            state.bit_current_move_pos as usize,
            state.current_player,
        );
        state.axes = axes;
        if state.current_player == global_var::PLAYER_WHITE_NB {
            state.white_move_to_win = board_state_info.nb_move_to_win;
        } else {
            state.black_move_to_win = board_state_info.nb_move_to_win;
        }
        return board_state_info;
    }
}

pub fn get_move_info(state: &mut State) -> BoardStateInfo {
    let axes = create_bits_axes_from_pos(state.bit_current_move_pos, &state.bitboards);
    let mut board_state_info = BoardStateInfo {
        player: state.current_player,
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
    };
    pattern_axes_dispatcher(
        &mut board_state_info,
        &mut state.bitboards,
        &axes,
        state.bit_current_move_pos as usize,
        state.current_player,
    );
    if state.current_player == global_var::PLAYER_WHITE_NB {
        state.white_move_to_win = board_state_info.nb_move_to_win;
    } else {
        state.black_move_to_win = board_state_info.nb_move_to_win;
    }
    return board_state_info;
}

pub fn check_is_double_triple(axe_pattern: [(usize, usize); 4]) -> bool {
    let mut count = 0;
    // println!("axe: {:?}", axe_pattern);
    for axe in 0..axe_pattern.len() {
        if axe_pattern[axe].1 == 0 {
            if axe_pattern[axe].0 >= 5 && axe_pattern[axe].0 <= 7 {
                count += 1;
            }
        }
    }
    return if count >= 2 { true } else { false };
}

pub fn check_is_capturable(axes: &[u16; 4], blocker_axes: &[u16; 4]) -> bool {
    let mut capturable = false;
    for axe in 0..axes.len() {
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        player_axe >>= 1;
        blocker_axe >>= 1;
        let shift: [usize; 4] = [0, 1, 2, 4];
        for s in shift.iter() {
            let player_shifted = player_axe >> s;
            let blocker_shifted = blocker_axe >> s;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            if (player_casted & CAPTURE_PATTERN[1].0) == CAPTURE_PATTERN[1].0 {
                if blocker_casted & CAPTURE_PATTERN[0].0 != 0
                    && blocker_casted & CAPTURE_PATTERN[0].0 != CAPTURE_PATTERN[0].0
                {
                    capturable = true;
                }
            }
            if player_casted & PATTERN[5].0 == PATTERN[5].0 {
                capturable = false;
            }
        }
        if capturable {
            return capturable;
        }
    }
    return capturable;
}

fn check_in_map(
    axe_mouvement_value: i16,
    pattern_pos: i16,
    offset: i16,
    direction_sign: i16,
) -> bool {
    let calcul = pattern_pos + axe_mouvement_value * offset * direction_sign;
    // println!("calcul = {}", calcul);
    if calcul < 0 {
        return false;
    }
    let line_checked = calcul / 19;
    let line = pattern_pos / 19;
    // Ligne
    if axe_mouvement_value == 1 {
        // println!("linechecked {} line {}", line_checked,line);
        if line_checked != line {
            return false;
        }
    } else {
        // println!("not ligne checked {} calcul: {}",line_checked ,line + offset * direction_sign);
        if line_checked != line + offset * direction_sign {
            return false;
        }
    }
    return true;
}

fn check_border(pos: usize, l: usize, axe: usize, pattern_length: usize) -> usize {
    let axe_mouvement_value: i16 = global_var::AXE_MOUVEMENT_VALUE[axe] as i16;
    let pattern_pos: i16 = pos as i16 - l as i16 * axe_mouvement_value;
    let low = pattern_pos + axe_mouvement_value;
    let high = pattern_pos + axe_mouvement_value * (pattern_length - 2) as i16;
    let mut border_count: usize = 0;
    // println!("pattern pos = {}, l = {}, axe = {}, pos = {}, pattern_length = {}, amv {}", pattern_pos, l, axe_mouvement_value, pos, pattern_length, axe_mouvement_value);
    // println!("low = {} high = {}",pattern_pos + axe_mouvement_value,pattern_pos + axe_mouvement_value * (pattern_length - 2) as i16);
    if check_in_map(axe_mouvement_value, low, 1, -1) == false {
        border_count += 1;
    }
    if check_in_map(axe_mouvement_value, high, 1, 1) == false {
        border_count += 1;
    }
    // println!("bordercount {}", border_count);
    return border_count;
}

pub fn check_blocker(
    blocker_checker: u8,
    blocker_casted: u8,
    pos: usize,
    b: usize,
    p: usize,
    l: usize,
    axe: usize,
) -> usize {
    let mut is_blocked: usize;
    let mut hole_value = false;
    if PATTERN[p].2 != 0 {
        hole_value = check_one_bit_in_pattern(&blocker_casted, PATTERN[p].2);
    }
    let border_count = check_border(pos, l, axe, PATTERN[p].1);
    // println!("blocker {:08b} blocker_casted {:08b} blocker_checked {:08b}, l {} , p {} , b {}",BLOCKER[b].0,blocker_casted,blocker_checker,l,p,b);
    if p == 5 || p == 6 {
        if b == 1 && hole_value == true && (p == 5 && blocker_checker & 0x80 != 0x80)
            || (p == 6 && blocker_checker & 0x4 != 0x4)
        {
            return 0;
        }
        if b == 1 && hole_value == true && blocker_checker == BLOCKER[b].0 {
            return 3;
        }
        if blocker_checker == BLOCKER[b].0 {
            return 2;
        } else if blocker_checker != 0 {
            if border_count > 0 {
                return 2;
            }
            return 1;
        }
    }
    if PATTERN[p].2 != 0 && l != PATTERN[p].2 && hole_value == true {
        is_blocked = 0;
    } else if PATTERN[p].2 != 0 && hole_value == true && (p != 5 || p != 6) {
        is_blocked = 2;
    } else if blocker_checker == BLOCKER[b].0 && PATTERN[p].2 == 0 {
        is_blocked = 2;
    } else if blocker_checker != 0 && PATTERN[p].2 == 0 {
        is_blocked = 1;
        if is_blocked + border_count >= 2 {
            is_blocked = 2;
        }
    } else if border_count > 0 {
        is_blocked = border_count;
    } else {
        is_blocked = 0;
    }
    return is_blocked;
}

pub fn check_one_bit_in_pattern(pattern: &u8, length: usize) -> bool {
    let mask: u8 = 0x80 >> length;
    if pattern & mask != 0 {
        return true;
    } else {
        return false;
    }
}

pub fn check_is_unblockable_five(
    bitboards: &mut Bitboards,
    pos: usize,
    axe_index: usize,
    player: i8,
) -> bool {
    for n in 0..5 {
        let check_pos = pos + n * global_var::AXE_MOUVEMENT_VALUE[axe_index];
        let axes = create_bits_axes_from_pos(check_pos, bitboards);
        let order: (usize, usize) = if player == 1 { (0, 1) } else { (1, 0) };
        if check_is_capturable(&axes[order.0], &axes[order.1]) {
            return false;
        }
    }
    return true;
}

pub fn check_potential_winning_alignment(state: &State) -> [bool; 4] {
    let player_axes;
    let opponent_axes;
    if state.current_player == global_var::PLAYER_WHITE_NB {
        player_axes = state.axes[0];
        opponent_axes = state.axes[1];
    } else {
        player_axes = state.axes[1];
        opponent_axes = state.axes[0];
    };
    let mut axe_free_value: [bool; 4] = [false, false, false, false];

    for axe_index in 0..player_axes.len() {
        let mut free_space: i8 = 0;
        let mut left_blocked = false;
        let mut right_blocked = false;

        let player_axe = player_axes[axe_index];
        let opponent_axe = opponent_axes[axe_index];
        for i in 1..7 {
            if !is_on_axe(
                global_var::AXE_MOUVEMENT_VALUE[axe_index],
                state.bit_current_move_pos,
                i as usize,
                1,
            ) {
                right_blocked = true;
            }
            if !is_on_axe(
                global_var::AXE_MOUVEMENT_VALUE[axe_index],
                state.bit_current_move_pos,
                i as usize,
                -1,
            ) {
                left_blocked = true;
            }

            let player_shifted = player_axe >> 8 - i;
            let player_casted = player_shifted as u8;
            let opponent_shifted = opponent_axe >> 8 - i;
            let opponent_casted = opponent_shifted as u8;
            if opponent_casted & 1 == 1 {
                right_blocked = true;
            } else if (player_casted & 0 == 0 || player_casted & 1 == 1) && !left_blocked {
                free_space += 1;
            }
            let player_shifted = player_axe >> 8 + i;
            let player_casted = player_shifted as u8;
            let opponent_shifted = opponent_axe >> 8 + i;
            let opponent_casted = opponent_shifted as u8;
            if opponent_casted & 1 == 1 {
                left_blocked = true;
            } else if (player_casted & 0 == 0 || player_casted & 1 == 1) && !right_blocked {
                free_space += 1;
            }
        }
        println!("free_space at end : {:?}", free_space);
        if free_space >= 4 {
            axe_free_value[axe_index] = true;
        } else {
            axe_free_value[axe_index] = false;
        }
    }

    return axe_free_value;
}

pub fn check_pos_still_win(bitboards: Bitboards, pos: usize, player: i8) -> bool {
    // println!("pos: {}, x: {} , y: {}", pos, pos / 19, pos % 19);
    let two_players_axes = create_bits_axes_from_pos(pos, &bitboards);
    let player_axes = if player == global_var::PLAYER_WHITE_NB {
        two_players_axes[0]
    } else {
        two_players_axes[1]
    };
    for axe in 0..player_axes.len() {
        let mut player_axe = player_axes[axe];
        player_axe >>= 1;
        for l in 0..6 {
            let player_shifted = player_axe >> l;
            let player_casted = player_shifted as u8;
            if (player_casted & PATTERN[0].0) == PATTERN[0].0 {
                // println!("pattern check: {:08b}", player_casted & PATTERN[0].0);
                return true;
            }
        }
    }
    return false;
}

pub fn check_move_is_capturing_stone(axes: &[u16; 4], blocker_axes: &[u16; 4]) -> i8 {
    let mut stone_captured: i8 = 0;
    for axe in 0..axes.len() {
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        player_axe >>= 1;
        blocker_axe >>= 1;
        let shift: [usize; 2] = [0, 3];
        for s in shift.iter() {
            let player_shifted = player_axe >> s;
            let blocker_shifted = blocker_axe >> s;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            if (player_casted & CAPTURE_PATTERN[0].0) == CAPTURE_PATTERN[0].0 {
                if (blocker_casted & CAPTURE_PATTERN[1].0) == CAPTURE_PATTERN[1].0 {
                    if *s == 3 {
                        stone_captured += 2;
                    } else {
                        stone_captured += 2;
                    }
                }
            }
        }
    }
    return stone_captured;
}

pub fn check_and_apply_capture(
    bitboards: &mut Bitboards,
    axes: &[u16; 4],
    blocker_axes: &[u16; 4],
    pos: usize,
    player: i8,
) -> i8 {
    let mut stone_captured: i8 = 0;
    for axe in 0..axes.len() {
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        player_axe >>= 1;
        blocker_axe >>= 1;
        let shift: [usize; 2] = [0, 3];
        for s in shift.iter() {
            let player_shifted = player_axe >> s;
            let blocker_shifted = blocker_axe >> s;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            if (player_casted & CAPTURE_PATTERN[0].0) == CAPTURE_PATTERN[0].0 {
                if (blocker_casted & CAPTURE_PATTERN[1].0) == CAPTURE_PATTERN[1].0 {
                    if *s == 3 {
                        stone_captured += 2;
                        apply_capture(
                            bitboards,
                            global_var::AXE_MOUVEMENT_VALUE[axe],
                            -1,
                            pos,
                            player,
                        );
                    } else {
                        stone_captured += 2;
                        apply_capture(
                            bitboards,
                            global_var::AXE_MOUVEMENT_VALUE[axe],
                            1,
                            pos,
                            player,
                        );
                    }
                }
            }
        }
    }
    return stone_captured;
}

#[pyfunction]
fn check_move_is_still_winning(
    board: Vec<Vec<i8>>,
    wining_position: (usize, i8),
) -> PyResult<bool> {
    let bitboards = create_bitboards_from_vec(&board);
    let still_winning = check_pos_still_win(bitboards, wining_position.0, wining_position.1);
    if still_winning == true {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn check_is_in_bitpos_list(box_position: &mut Vec<usize>, bitpos: usize) -> bool {
    let len = box_position.len();
    for pos in 0..len {
        if box_position[pos] == bitpos {
            return true;
        }
    }
    return false;
}
