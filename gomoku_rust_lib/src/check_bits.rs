use crate::state::State;

pub fn check_is_in_board(pos: i16) -> bool {
    if pos < 0 || pos > 360 {
        return false;
    }
    return true;
}

pub fn get_bits_in_bitboard_from_pos(pos: i16, bitboard: &[i64; 6]) -> i8 {
    if !check_is_in_board(pos) {
        return -2;
    }
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

// #[allow(dead_code)]
// pub fn bits_check_move_is_in_capturing_position(move_pos: i16, state: &State) -> bool {
//     let mut tree_bits: i8;
//     let mut mask: i8;

//     for axe_increment_value in vec![20, 19, 18, 1] {
//         tree_bits = 1 << 1;
//         if state.current_player == 1 {
//             if get_bits_in_bitboard_from_pos(
//                 move_pos + axe_increment_value,
//                 &state.bitboards.black_board,
//             ) == 1
//             {
//                 mask = 1;
//             } else {
//                 mask = 0;
//             }
//             tree_bits = tree_bits | mask;
//             if get_bits_in_bitboard_from_pos(
//                 move_pos - axe_increment_value,
//                 &state.bitboards.black_board,
//             ) == 1
//             {
//                 mask = 1 << 2;
//             } else {
//                 mask = 0;
//             }
//             tree_bits = tree_bits | mask;
//         } else {
//             if get_bits_in_bitboard_from_pos(
//                 move_pos + axe_increment_value,
//                 &state.bitboards.white_board,
//             ) == 1
//             {
//                 mask = 1;
//             } else {
//                 mask = 0;
//             }
//             tree_bits = tree_bits | mask;
//             if get_bits_in_bitboard_from_pos(
//                 move_pos - axe_increment_value,
//                 &state.bitboards.white_board,
//             ) == 1
//             {
//                 mask = 1 << 2;
//             } else {
//                 mask = 0;
//             }
//             tree_bits = tree_bits | mask;
//         }
//         if tree_bits == 1 + (1 << 1) + (1 << 2) {
//             return true;
//         }
//     }
//     return false;
// }

#[allow(dead_code)]
pub fn bits_check_move_is_in_capturing_position(move_pos: i16, state: &State) -> bool {
    let mut seven_bits: i8;
    let mut mask: i8;

    // YXXY -> 110 -> 011 -> 11010010 -> 011
    // XXY

    for axe_increment_value in vec![20, 19, 18, 1] {
        seven_bits = 1 << 1;
        if state.current_player == 1 {
            if get_bits_in_bitboard_from_pos(
                move_pos - axe_increment_value,
                &state.bitboards.white_board,
            ) == 1
            {
                mask = 1;
            } else {
                mask = 0;
            }
            seven_bits = seven_bits | mask;

            if get_bits_in_bitboard_from_pos(
                move_pos + axe_increment_value,
                &state.bitboards.white_board,
            ) == 1
            {
                mask = 1 << 2;
            } else {
                mask = 0;
            }
            seven_bits = seven_bits | mask;

            if get_bits_in_bitboard_from_pos(
                move_pos - (axe_increment_value * 2),
                &state.bitboards.black_board,
            ) == 1
            {
                mask = 1 << 3;
            } else {
                mask = 0;
            }
            seven_bits = seven_bits | mask;
            if get_bits_in_bitboard_from_pos(
                move_pos - axe_increment_value,
                &state.bitboards.black_board,
            ) == 1
            {
                mask = 1 << 4;
            } else {
                mask = 0;
            }
            seven_bits = seven_bits | mask;
            if get_bits_in_bitboard_from_pos(
                move_pos + axe_increment_value,
                &state.bitboards.black_board,
            ) == 1
            {
                mask = 1 << 5;
            } else {
                mask = 0;
            }
            seven_bits = seven_bits | mask;
            if get_bits_in_bitboard_from_pos(
                move_pos + (axe_increment_value * 2),
                &state.bitboards.black_board,
            ) == 1
            {
                mask = 1 << 6;
            } else {
                mask = 0;
            }
            seven_bits = seven_bits | mask;
        }

        // for ???X??? : 0101111 | 0100111 | 0101110 | 0100110
        // for ??XX??? : 1101011  | 1101010
        // for ???XX?? : 0111101  | 1100101
        println!("seven_bits == {:?}", seven_bits);
        if seven_bits == 47
            || seven_bits == 39
            || seven_bits == 46
            || seven_bits == 38
            || seven_bits == 107
            || seven_bits == 106
            || seven_bits == 61
            || seven_bits == 101
        {
            return true;
        }
    }
    return false;
}
