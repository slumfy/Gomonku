//! Methods to update stone on bitboard.

use crate::bitboards::create_bits_axes_from_pos;
use crate::bitpattern::pattern_axes_finder;
use crate::data_struct::Bitboards;
use crate::data_struct::BoardStateInfo;
use crate::global_var;
use crate::heuristic_ratios;

fn return_captured_pattern_blocking_value(
    bitboards: &mut Bitboards,
    axes: [[u16; 4]; 2],
    pos: usize,
    opponent: i8,
) -> i64 {
    let opponent_pattern_axes: [(usize, usize); 4];
    if opponent == global_var::PLAYER_WHITE_NB {
        opponent_pattern_axes =
            pattern_axes_finder(bitboards, &axes[0], &axes[1], pos, opponent)[0];
    } else {
        opponent_pattern_axes =
            pattern_axes_finder(bitboards, &axes[1], &axes[0], pos, opponent)[0];
    }
    let mut value = 0;
    for axe_index in 0..4 {
        let found_opponent_pattern_on_axe = opponent_pattern_axes[axe_index].0;
        let numbers_of_blocker_on_pattern = opponent_pattern_axes[axe_index].1;
        // Not additional capture value for capture pattern 4 and 5 because it will not prevent winning.
        if numbers_of_blocker_on_pattern != 3
            && numbers_of_blocker_on_pattern != 5
            && found_opponent_pattern_on_axe > 4
        {
            value += heuristic_ratios::HEURISTIC_BLOCKER[found_opponent_pattern_on_axe][1];
            value += heuristic_ratios::HEURISTIC_BLOCKER[found_opponent_pattern_on_axe][2];
        }
    }
    return value;
}

pub fn apply_capture_and_return_captured_pattern_blocking_value(
    bitboards: &mut Bitboards,
    board_state_info: &mut BoardStateInfo,
    axe_mouvement_value: usize,
    direction_sign: isize,
    pos: usize,
    player: i8,
) {
    let opponent = -player;
    for n in 1..3 {
        if direction_sign == -1 {
            let axes = create_bits_axes_from_pos(pos - (n * axe_mouvement_value), bitboards);
            board_state_info.captured_pattern_blocking_value +=
                return_captured_pattern_blocking_value(
                    bitboards,
                    axes,
                    pos - (n * axe_mouvement_value),
                    opponent,
                );
            remove_bit(bitboards, pos - (n * axe_mouvement_value), opponent);
        } else {
            let axes = create_bits_axes_from_pos(pos + (n * axe_mouvement_value), bitboards);
            board_state_info.captured_pattern_blocking_value +=
                return_captured_pattern_blocking_value(
                    bitboards,
                    axes,
                    pos + (n * axe_mouvement_value),
                    opponent,
                );
            remove_bit(bitboards, pos + (n * axe_mouvement_value), opponent);
        }
    }
}

pub fn apply_bit(bitboards: &mut Bitboards, pos: usize, player: i8) {
    let real_pos = pos % 64;
    let bit_pos = 63 - real_pos;
    let bitboards_index = pos / 64;
    let mask = 1 << bit_pos;
    if player == 1 {
        bitboards.white_board[bitboards_index] |= mask;
    } else {
        bitboards.black_board[bitboards_index] |= mask;
    }
}

pub fn remove_bit(bitboards: &mut Bitboards, pos: usize, player: i8) {
    let real_pos = pos % 64;
    let bit_pos = 63 - real_pos;
    let bitboards_index = pos / 64;
    let mask = !(1 << bit_pos);
    if player == 1 {
        bitboards.white_board[bitboards_index] &= mask;
    } else {
        bitboards.black_board[bitboards_index] &= mask;
    }
}
