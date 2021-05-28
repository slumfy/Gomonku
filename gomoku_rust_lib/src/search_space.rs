//! Creation of boxes where the AI search solutions.

use crate::check_move::check_is_in_bitpos_list;
use crate::data_struct::Bitboards;
use crate::ALL_MOVES_LIST;

pub fn get_search_box_bitboard(bitboard: &Bitboards) -> Vec<usize> {
    let mut box_position: Vec<usize> = vec![];
    unsafe {
        for pos in &ALL_MOVES_LIST {
            let bitboard_index = pos / 64;
            let real_pos = pos % 64;
            let bit_pos = 63 - real_pos;
            let mask = 1 << bit_pos;
            if bitboard.white_board[bitboard_index] & mask != 0
                || bitboard.black_board[bitboard_index] & mask != 0
            {
                create_box_for_bitpos(*pos, &mut box_position);
            }
        }
    }
    return box_position;
}

#[allow(dead_code)]
fn create_box_for_bitpos(bitpos: usize, box_position: &mut Vec<usize>) {
    let box_size = 1;
    let y = bitpos % 19;
    let x = bitpos / 19;
    let xmin = if x < box_size { 0 } else { x - box_size };
    let xmax = if x + box_size >= 18 { 18 } else { x + box_size };
    let ymin = if y < box_size { 0 } else { y - box_size };
    let ymax = if y + box_size >= 18 { 18 } else { y + box_size };
    for idx in xmin..xmax + 1 {
        for idy in ymin..ymax + 1 {
            if check_is_in_bitpos_list(box_position, idx * 19 + idy) == false {
                box_position.push(idx * 19 + idy);
            }
        }
    }
}

#[allow(dead_code)]
pub fn unwrap_bitlist(box_position: Vec<usize>) -> Vec<(usize, usize)> {
    let len = box_position.len();
    let mut unwrap = vec![];
    for pos in 0..len {
        unwrap.push((box_position[pos] / 19, box_position[pos] % 19));
    }
    return unwrap;
}
