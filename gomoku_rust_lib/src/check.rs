use crate::check_bits::bits_check_move_is_in_capturing_position;
use crate::state::State;
use crate::utils::create_axes_from_stone_position;
use std::cmp::max;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn check_is_in_table(x: isize, y: isize, xsign: isize, ysign: isize, offset: isize) -> i8 {
    if x + offset * xsign > 18
        || x + offset * xsign < 0
        || y + offset * ysign > 18
        || y + offset * ysign < 0
    {
        return 1;
    }
    return 0;
}

#[allow(dead_code)]
fn check_move_biggest_alignment_in_axes(axes: &Vec<Vec<i8>>, player: i8) -> i8 {
    let mut count_axe_0: i8 = 1;
    let mut count_axe_0_lock: bool = false;
    let mut count_axe_1: i8 = 1;
    let mut count_axe_1_lock: bool = false;
    let mut count_axe_2: i8 = 1;
    let mut count_axe_2_lock: bool = false;
    let mut count_axe_3: i8 = 1;
    let mut count_axe_3_lock: bool = false;

    // count left part
    for i in vec![3, 2, 1, 0] {
        if !count_axe_0_lock && axes[0][i] == player {
            count_axe_0 += 1;
        } else {
            count_axe_0_lock = true;
        }
        if !count_axe_1_lock && axes[1][i] == player {
            count_axe_1 += 1;
        } else {
            count_axe_1_lock = true;
        }
        if !count_axe_2_lock && axes[2][i] == player {
            count_axe_2 += 1;
        } else {
            count_axe_2_lock = true;
        }
        if !count_axe_3_lock && axes[3][i] == player {
            count_axe_3 += 1;
        } else {
            count_axe_3_lock = true;
        }
        // Check if still need to check in some axes
        if count_axe_0_lock && count_axe_1_lock && count_axe_2_lock && count_axe_3_lock {
            break;
        }
    }

    count_axe_0_lock = false;
    count_axe_1_lock = false;
    count_axe_2_lock = false;
    count_axe_3_lock = false;

    // count right part
    for i in 5..9 {
        if !count_axe_0_lock && axes[0][i] == player {
            count_axe_0 += 1;
        } else {
            count_axe_0_lock = true;
        }
        if !count_axe_1_lock && axes[1][i] == player {
            count_axe_1 += 1;
        } else {
            count_axe_1_lock = true;
        }
        if !count_axe_2_lock && axes[2][i] == player {
            count_axe_2 += 1;
        } else {
            count_axe_2_lock = true;
        }
        if !count_axe_3_lock && axes[3][i] == player {
            count_axe_3 += 1;
        } else {
            count_axe_3_lock = true;
        }
        // Check if still need to check in some axes
        if count_axe_0_lock && count_axe_1_lock && count_axe_2_lock && count_axe_3_lock {
            break;
        }
    }
    return max(count_axe_0, max(count_axe_1, max(count_axe_2, count_axe_3)));
}

#[allow(dead_code)]
fn check_move_is_in_capturing_position_in_axe(axe: &Vec<i8>, player: i8) -> bool {
    let opponent: i8 = -player;
    if (axe[3] == opponent || (axe[3] == player && axe[2] == opponent))
        && (axe[5] == opponent || (axe[5] == player && axe[6] == opponent))
    {
        return true;
    } else {
        return false;
    }
}

pub fn check_move_is_capturing_stone_in_axe(axe: &Vec<i8>, player: i8) -> i8 {
    let opponent: i8 = -player;
    let mut count_eat: i8 = 0;

    if axe[3] == opponent && axe[2] == player {
        count_eat += 1;
    }
    if axe[5] == opponent && axe[6] == player {
        count_eat += 1;
    }
    if axe[3] == opponent && axe[2] == opponent && axe[1] == player {
        count_eat += 2;
    }
    if axe[5] == opponent && axe[6] == opponent && axe[7] == player {
        count_eat += 2;
    }

    return count_eat;
}

#[allow(dead_code)]
/// Return true if there is any double triple in the axes, false otherwise.
pub fn check_move_is_double_triple(axes: &Vec<Vec<i8>>, player: i8) -> bool {
    let mut triple_count = 0;

    for axe in axes {
        // Checking left part of axe double triple
        // 0110M0000
        // 0101M0000
        // 0011M0000
        if axe[5] == 0
            && ((axe[3] == player && axe[2] == player && axe[1] == 0)
                || (axe[3] == player && axe[2] == 0 && axe[1] == player && axe[0] == 0)
                || (axe[3] == 0 && axe[2] == player && axe[1] == player && axe[0] == 0))
        {
            triple_count += 1;
        }
        // Checking right part of axe double triple
        // 0000M1010
        // 0000M1100
        // 0000M0110
        else if axe[3] == 0
            && ((axe[5] == player && axe[6] == player && axe[7] == 0)
                || (axe[5] == player && axe[6] == 0 && axe[7] == player && axe[8] == 0)
                || (axe[5] == 0 && axe[6] == player && axe[7] == player && axe[8] == 0))
        {
            triple_count += 1;
        }
        // Checking center part of axe double triple
        // 0001M1000
        // 0010M1000
        else if axe[5] == player
            && axe[6] == 0
            && ((axe[3] == player && axe[2] == 0)
                || (axe[3] == 0 && axe[2] == player && axe[1] == 0))
        {
            triple_count += 1;
        }
        // Checking center part of axe double triple
        // 0001M0100
        else if axe[5] == 0 && axe[6] == player && axe[7] == 0 && axe[3] == player && axe[2] == 0
        {
            triple_count += 1;
        }
    }
    if triple_count > 1 {
        return true;
    }
    return false;
}

#[allow(dead_code)]
pub fn check_is_wrong_move(state: &State, axes: &Vec<Vec<i8>>) -> i8 {
    let stone_x = state.current_move.0;
    let stone_y = state.current_move.1;
    if stone_x < 0 || stone_x > 19 || stone_y < 0 || stone_y > 19 {
        return -1;
    } else if state.board[stone_x as usize][stone_y as usize] != 0 {
        return -2;
    }
    let player: i8 = state.current_player;

    for axe in axes {
        if check_move_is_in_capturing_position_in_axe(&axe, player) {
            return -3;
        }
    }
    if check_move_is_double_triple(&axes, player) == true {
        return -4;
    }
    return 0;
}

#[allow(dead_code)]
pub fn checking_move(state: &State) -> HashMap<String, i8> {
    let mut board_check: HashMap<String, i8> = HashMap::new();
    println!("LAAAA");
    if bits_check_move_is_in_capturing_position(state.bit_current_move_pos, state) {
        println!("IS IN CAPTURING POSITION WITH BIT CHECK !!!!!!! YEAHHH");
    }
    let axes = create_axes_from_stone_position(
        state,
        state.current_move.0,
        state.current_move.1,
        state.current_player,
    );
    let player: i8 = state.current_player;
    let mut count_eat: i8 = 0;

    board_check.insert(
        String::from("is_wrong_move"),
        check_is_wrong_move(state, &axes),
    );
    if board_check["is_wrong_move"] == 0 {
        for axe in &axes {
            count_eat += check_move_is_capturing_stone_in_axe(&axe, player);
        }
        board_check.insert(
            String::from("biggest_alignment"),
            check_move_biggest_alignment_in_axes(&axes, player),
        );
        board_check.insert(String::from("stone_captured"), count_eat);
    }
    return board_check;
}

#[allow(dead_code)]
pub fn checking_move_biggest_alignment_and_stone_captured(state: &State) -> HashMap<String, i8> {
    let mut board_check: HashMap<String, i8> = HashMap::new();
    let player: i8 = state.current_player;
    let axes =
        create_axes_from_stone_position(state, state.current_move.0, state.current_move.1, player);
    let mut count_eat: i8 = 0;

    for axe in &axes {
        count_eat += check_move_is_capturing_stone_in_axe(&axe, player);
    }
    board_check.insert(
        String::from("biggest_alignment"),
        check_move_biggest_alignment_in_axes(&axes, player),
    );
    board_check.insert(String::from("stone_captured"), count_eat);
    return board_check;
}

#[allow(dead_code)]
pub fn check_alignment_for_given_pos(state: &State, x: isize, y: isize, player: i8) -> bool {
    let axes = create_axes_from_stone_position(state, x, y, player);
    if state.board[x as usize][y as usize] == player {
        if check_move_biggest_alignment_in_axes(&axes, player) == 5 {
            return true;
        }
    }
    return false;
}
