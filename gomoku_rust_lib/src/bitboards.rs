//! Methods to create bitboards from vec and axes from pos.

use crate::global_var;
use crate::utils::is_on_axe;

#[derive(Copy, Clone, Hash)]
pub struct Bitboards {
    pub white_board: [u64; 6],
    pub black_board: [u64; 6],
}

pub fn create_bits_axes_from_pos(
    move_pos: usize,
    bitboards: &Bitboards,
    player: i8,
) -> [[u16; 4]; 2] {
    fn set_bit_in_axe_from_bitboard(
        bits_axes_array: &mut [[u16; 4]; 2],
        bits_axes_array_index: usize,
        bitboard: &[u64; 6],
        index: usize,
        move_pos: usize,
        axe_increment_value: usize,
        i: usize,
        move_index: usize,
    ) {
        let mut ret;

        // Getting left part
        if bits_axes_array[bits_axes_array_index][index] & (1 << 15) == 0 {
            if (move_pos as isize - axe_increment_value as isize * i as isize)
                < (global_var::BOARD_MIN_LIMITS as isize)
                || move_pos as isize - axe_increment_value as isize * i as isize
                    > global_var::BOARD_MAX_LIMITS as isize
            {
                ret = global_var::OUT_OF_BOARD_MOVE;
            } else {
                ret = get_bits_in_bitboard_from_pos(move_pos - axe_increment_value * i, bitboard);
            }
            if ret == global_var::OUT_OF_BOARD_MOVE
                || !is_on_axe(axe_increment_value, move_pos, i, -1)
            {
                bits_axes_array[bits_axes_array_index][index] |= 1 << 15;
            } else if ret == 1 {
                bits_axes_array[bits_axes_array_index][index] |= 1 << (move_index + i);
            }
        }
        // Getting right part
        if bits_axes_array[bits_axes_array_index][index] & 1 == 0 {
            if (move_pos as isize + axe_increment_value as isize * i as isize)
                < (global_var::BOARD_MIN_LIMITS as isize)
                || move_pos + axe_increment_value * i > global_var::BOARD_MAX_LIMITS
            {
                ret = global_var::OUT_OF_BOARD_MOVE;
            } else {
                ret = get_bits_in_bitboard_from_pos(move_pos + axe_increment_value * i, bitboard);
            }
            if ret == global_var::OUT_OF_BOARD_MOVE
                || !is_on_axe(axe_increment_value, move_pos, i, 1)
            {
                bits_axes_array[bits_axes_array_index][index] |= 1;
            } else if ret == 1 {
                bits_axes_array[bits_axes_array_index][index] |= 1 << (move_index - i);
            }
        }
    }

    let mut bits_axes_array: [[u16; 4]; 2] = [[0, 0, 0, 0], [0, 0, 0, 0]];
    let mut index = 0;
    let move_index = 8;
    for axe_increment_value in global_var::AXE_MOUVEMENT_VALUE.iter() {
        if player == global_var::PLAYER_WHITE_NB {
            bits_axes_array[0][index] = 1 << move_index;
        } else {
            bits_axes_array[1][index] = 1 << move_index;
        }
        for i in 1..5 {
            // Getting stone from white board
            set_bit_in_axe_from_bitboard(
                &mut bits_axes_array,
                0,
                &bitboards.white_board,
                index,
                move_pos,
                *axe_increment_value,
                i,
                move_index,
            );

            // Getting stone from black board
            set_bit_in_axe_from_bitboard(
                &mut bits_axes_array,
                1,
                &bitboards.black_board,
                index,
                move_pos,
                *axe_increment_value,
                i,
                move_index,
            );
        }
        index += 1;
    }
    return bits_axes_array;
}

pub fn create_bitboards_from_vec(board: &Vec<Vec<i8>>) -> Bitboards {
    let mut new_bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    for x in 0..19 {
        for y in 0..19 {
            let real_pos = (x * 19 + y) % 64;
            let bit_pos = 63 - real_pos;
            let bitboards_index = (x * 19 + y) / 64;
            let mask = 1 << bit_pos;
            if board[x][y] == 1 {
                new_bitboards.white_board[bitboards_index] =
                    new_bitboards.white_board[bitboards_index] | mask;
            } else if board[x][y] == -1 {
                new_bitboards.black_board[bitboards_index] =
                    new_bitboards.black_board[bitboards_index] | mask;
            }
        }
    }
    return new_bitboards;
}

pub fn create_vec_from_bitboards(bitboards: &Bitboards) -> Vec<Vec<i8>> {
    let mut board: Vec<Vec<i8>> = vec![];
    for x in 0..19 {
        board.push(vec![]);
        for y in 0..19 {
            let real_pos = (x * 19 + y) % 64;
            let bit_pos = 63 - real_pos;
            let bitboards_index = (x * 19 + y) / 64;
            let mask = 1 << bit_pos;
            if bitboards.white_board[bitboards_index] & mask != 0 {
                board[x].push(1);
            } else if bitboards.black_board[bitboards_index] & mask != 0 {
                board[x].push(-1);
            } else {
                board[x].push(0);
            }
        }
    }
    return board;
}

pub fn get_bits_in_bitboard_from_pos(pos: usize, bitboard: &[u64; 6]) -> i8 {
    let real_pos = pos % 64;
    let bit_pos = 63 - real_pos;
    let bitboard_index = pos / 64;
    let mask: u64 = 1 << bit_pos;
    if bitboard[bitboard_index as usize] & mask != 0 {
        return 1;
    } else {
        return 0;
    }
}
