use crate::check_bits::get_bits_in_bitboard_from_pos;
use crate::check_bits::get_line_from_pos;
use crate::state::State;

#[derive(Copy, Clone)]
pub struct Bitboards {
    pub white_board: [u64; 6],
    pub black_board: [u64; 6],
}
pub fn create_bits_axes_from_pos(move_pos: i16, state: &State) -> [[u16; 4]; 2] {
    fn check_is_on_axe(
        axe_increment_value: i16,
        move_pos: i16,
        i: i16,
        direction_sign: i16,
    ) -> bool {
        if axe_increment_value == 1 {
            if get_line_from_pos(move_pos + axe_increment_value * i * direction_sign)
                != get_line_from_pos(move_pos)
            {
                return false;
            }
        } else {
            if get_line_from_pos(move_pos + axe_increment_value * i * direction_sign)
                != get_line_from_pos(move_pos) + i * direction_sign
            {
                return false;
            }
        }
        return true;
    }

    fn set_bit_in_axe_from_bitboard(
        bits_axes_array: &mut [[u16; 4]; 2],
        bits_axes_array_index: usize,
        bitboard: &[u64; 6],
        index: usize,
        move_pos: i16,
        axe_increment_value: i16,
        i: i16,
        move_index: i16,
    ) {
        let mut ret;

        // Getting left part
        if bits_axes_array[bits_axes_array_index][index] & (1 << 15) == 0 {
            ret = get_bits_in_bitboard_from_pos(move_pos - axe_increment_value * i, bitboard);
            if ret == -2 || !check_is_on_axe(axe_increment_value, move_pos, i, -1) {
                bits_axes_array[bits_axes_array_index][index] =
                    bits_axes_array[bits_axes_array_index][index] | 1 << 15;
            } else if ret == 1 {
                bits_axes_array[bits_axes_array_index][index] =
                    bits_axes_array[bits_axes_array_index][index] | 1 << move_index + i;
            }
        }
        // Getting right part
        if bits_axes_array[bits_axes_array_index][index] & 1 == 0 {
            ret = get_bits_in_bitboard_from_pos(move_pos + axe_increment_value * i, bitboard);
            if ret == -2 || !check_is_on_axe(axe_increment_value, move_pos, i, 1) {
                bits_axes_array[bits_axes_array_index][index] =
                    bits_axes_array[bits_axes_array_index][index] | 1;
            } else if ret == 1 {
                bits_axes_array[bits_axes_array_index][index] =
                    bits_axes_array[bits_axes_array_index][index] | 1 << move_index - i;
            }
        }
    }

    let mut bits_axes_array: [[u16; 4]; 2] = [[0, 0, 0, 0], [0, 0, 0, 0]];
    let mut index = 0;
    let move_index = 8;
    for axe_increment_value in vec![20, 19, 18, 1] {
        if state.current_player == 1 {
            bits_axes_array[0][index] = 1 << move_index;
        } else {
            bits_axes_array[1][index] = 1 << move_index;
        }
        for i in 1..5 {
            // Getting stone from white board
            set_bit_in_axe_from_bitboard(
                &mut bits_axes_array,
                0,
                &state.bitboards.white_board,
                index,
                move_pos,
                axe_increment_value,
                i,
                move_index,
            );

            // Getting stone from black board
            set_bit_in_axe_from_bitboard(
                &mut bits_axes_array,
                1,
                &state.bitboards.black_board,
                index,
                move_pos,
                axe_increment_value,
                i,
                move_index,
            );
        }

        println!("axes value = {:?}", axe_increment_value);
        println!("bits_axes_array white here = {:?}", bits_axes_array[0]);
        println!("");
        println!("bits_axes_array black here = {:?}", bits_axes_array[1]);

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

pub fn apply_bitmove(bitboards: &mut Bitboards, pos: usize, player: i8) {
    let real_pos = pos % 64;
    let bit_pos = 63 - real_pos;
    let bitboards_index = pos / 64;
    let mask = 1 << bit_pos;
    if player == 1 {
        bitboards.white_board[bitboards_index] = bitboards.white_board[bitboards_index] | mask;
    } else {
        bitboards.black_board[bitboards_index] = bitboards.black_board[bitboards_index] | mask;
    }
}
