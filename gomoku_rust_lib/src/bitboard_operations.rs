//! Methods to update stone on bitboard.

use crate::bitboards::Bitboards;

pub fn apply_capture(bitboards: &mut Bitboards, axe: usize, s: isize, pos: usize, player: i8) {
    let opponent = -player;
    for n in 1..3 {
        if s == -1 {
            remove_bit(bitboards, pos - (n * axe), opponent);
        } else {
            remove_bit(bitboards, pos + (n * axe), opponent);
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
