use std::collections::HashMap;

use crate::bitboards::create_bits_axes_from_pos;
use crate::bitboards::Bitboards;
use crate::bitpattern::pattern_axes_dispatcher;
use crate::state::State;

pub fn check_pos_is_in_board(pos: i16) -> bool {
    if pos < 0 || pos > 360 {
        return false;
    }
    return true;
}

pub fn get_line_from_pos(pos: i16) -> i16 {
    return pos / 19;
}

pub fn get_bits_in_bitboard_from_pos(pos: i16, bitboard: &[u64; 6]) -> i8 {
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

fn is_no_stone_on_bitboard_pos(pos: i16, bitboards: &Bitboards) -> bool {
    if get_bits_in_bitboard_from_pos(pos, &bitboards.white_board) != 0
        || get_bits_in_bitboard_from_pos(pos, &bitboards.black_board) != 0
    {
        return false;
    }
    return true;
}

fn check_is_wrong_move(state: &State) -> i8 {
    if !check_pos_is_in_board(state.bit_current_move_pos) {
        return -1;
    }
    if !is_no_stone_on_bitboard_pos(state.bit_current_move_pos, &state.bitboards) {
        return -2;
    }
    return 0;
}

pub fn checking_and_apply_bits_move(state: &mut State) -> HashMap<String, i8> {
    let mut bitboard_check: HashMap<String, i8> = HashMap::new();
    let pattern_return_infos: HashMap<String, i8>;
    let axes = create_bits_axes_from_pos(state.bit_current_move_pos, &state);

    bitboard_check.insert(String::from("is_wrong_move"), check_is_wrong_move(state));
    pattern_return_infos = pattern_axes_dispatcher(
        &mut state.bitboards,
        &axes,
        state.bit_current_move_pos as usize,
        state.current_player,
    );
    bitboard_check.insert(String::from("biggest_alignment"), 1);
    bitboard_check.insert(
        String::from("stone_captured"),
        pattern_return_infos["stone_captured"],
    );
    return bitboard_check;
}

#[allow(dead_code)]
pub fn bits_check_move_is_in_capturing_position(move_pos: i16, state: &State) -> bool {
    let mut seven_bits: i8;
    let mut bitboard;

    // YXXY -> 110 -> 011 -> 11010010 -> 011
    // XXY

    fn add_current_player_stone_to_seven_bits(
        bitboard: &[u64; 6],
        axe_increment_value: i16,
        move_pos: i16,
        mut seven_bits: i8,
    ) -> i8 {
        let mut mask: i8;

        if get_bits_in_bitboard_from_pos(move_pos - axe_increment_value, bitboard) == 1 {
            mask = 1 << 6;
        } else {
            mask = 0;
        }
        seven_bits = seven_bits | mask;

        if get_bits_in_bitboard_from_pos(move_pos + axe_increment_value, bitboard) == 1 {
            mask = 1 << 4;
        } else {
            mask = 0;
        }
        seven_bits = seven_bits | mask;
        return seven_bits;
    }

    fn add_opponent_player_stone_to_seven_bits(
        bitboard: &[u64; 6],
        axe_increment_value: i16,
        move_pos: i16,
        mut seven_bits: i8,
    ) -> i8 {
        let mut mask: i8;

        if get_bits_in_bitboard_from_pos(move_pos - (axe_increment_value * 2), bitboard) == 1 {
            mask = 1 << 3;
        } else {
            mask = 0;
        }
        seven_bits = seven_bits | mask;

        if get_bits_in_bitboard_from_pos(move_pos - axe_increment_value, bitboard) == 1 {
            mask = 1 << 2;
        } else {
            mask = 0;
        }
        seven_bits = seven_bits | mask;

        if get_bits_in_bitboard_from_pos(move_pos + axe_increment_value, bitboard) == 1 {
            mask = 1 << 1;
        } else {
            mask = 0;
        }
        seven_bits = seven_bits | mask;
        if get_bits_in_bitboard_from_pos(move_pos + (axe_increment_value * 2), bitboard) == 1 {
            mask = 1;
        } else {
            mask = 0;
        }
        seven_bits = seven_bits | mask;
        return seven_bits;
    }

    for axe_increment_value in vec![20, 19, 18, 1] {
        if state.current_player == 1 {
            bitboard = &state.bitboards.white_board;
        } else {
            bitboard = &state.bitboards.black_board;
        }

        seven_bits = 1 << 5;
        seven_bits = add_current_player_stone_to_seven_bits(
            bitboard,
            axe_increment_value,
            move_pos,
            seven_bits,
        );

        if state.current_player == 1 {
            bitboard = &state.bitboards.black_board;
        } else {
            bitboard = &state.bitboards.white_board;
        }
        seven_bits = add_opponent_player_stone_to_seven_bits(
            bitboard,
            axe_increment_value,
            move_pos,
            seven_bits,
        );

        // for ???X??? : 0101111 | 0100111 | 0101110 | 0100110
        // for ??XX??? : 1101011  | 1101010
        // for ???XX?? : 0111101  | 0110101
        if seven_bits == 0x2F
            || seven_bits == 0x27
            || seven_bits == 0x2E
            || seven_bits == 0x26
            || seven_bits == 0x6B
            || seven_bits == 0x6A
            || seven_bits == 0x3D
            || seven_bits == 0x35
        {
            return true;
        }
    }

    return false;
}
