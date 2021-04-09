use std::collections::HashMap;

use crate::bitboards::apply_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::bitboards::Bitboards;
use crate::bitpattern::pattern_axes_dispatcher;
use crate::global_var;
use crate::state::State;

pub fn get_line_from_pos(pos: usize) -> usize {
    return pos / 19;
}

pub fn get_bits_in_bitboard_from_pos(pos: usize, bitboard: &[u64; 6]) -> i8 {
    let real_pos = pos % 64;
    let bit_pos = 63 - real_pos;
    let bitboard_index = pos / 64;
    let mask = 1 << bit_pos;
    if bitboard[bitboard_index as usize] & mask != 0 {
        return 1;
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

pub fn checking_and_apply_bits_move(state: &mut State) -> HashMap<String, i8> {
    let mut bitboard_check: HashMap<String, i8> = HashMap::new();
    let pattern_return_infos: HashMap<String, i8>;
    bitboard_check.insert(String::from("is_wrong_move"), check_is_wrong_move(state));
    if bitboard_check["is_wrong_move"] != global_var::VALID_MOVE {
        return bitboard_check;
    } else {
        let axes = create_bits_axes_from_pos(
            state.bit_current_move_pos,
            &state.bitboards,
            state.current_player,
        );

        apply_bit(
            &mut state.bitboards,
            state.bit_current_move_pos as usize,
            state.current_player,
        );
        pattern_return_infos = pattern_axes_dispatcher(
            &mut state.bitboards,
            &axes,
            state.bit_current_move_pos as usize,
            state.current_player,
        );
        if pattern_return_infos["double_triple"] == 1 {
            *bitboard_check.get_mut("is_wrong_move").unwrap() = -1;
        }
        bitboard_check.insert(String::from("biggest_alignment"), 1);
        bitboard_check.insert(
            String::from("stone_captured"),
            pattern_return_infos["stone_captured"],
        );
        return bitboard_check;
    }
}
