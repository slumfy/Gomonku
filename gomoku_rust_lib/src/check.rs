use crate::state::State;
use std::cmp::max;

fn create_axes_from_stone_position(state: &State) -> Vec<Vec<i8>> {
    let board = &state.board;
    let box_half_size: isize = 5;
    let player = state.player_to_play;
    let stone_x = state.current_move.0;
    let stone_y = state.current_move.1;
    let mut axes: Vec<Vec<i8>> = vec![];

    axes.push(vec![player]);
    axes.push(vec![player]);
    axes.push(vec![player]);
    axes.push(vec![player]);

    for i in 1..box_half_size {
        // Vertical x
        if stone_x - i >= 0 {
            axes[0].insert(0, board[(stone_x - i) as usize][stone_y as usize]);
        } else {
            axes[0].insert(0, -2);
        }
        if stone_x + i < 19 {
            axes[0].push(board[(stone_x + i) as usize][stone_y as usize]);
        } else {
            axes[0].push(-2);
        }

        // Horizontal y
        if stone_y - i >= 0 {
            axes[1].insert(0, board[(stone_x) as usize][(stone_y - i) as usize]);
        } else {
            axes[1].insert(0, -2);
        }
        if stone_y + i < 19 {
            axes[1].push(board[stone_x as usize][(stone_y + i) as usize]);
        } else {
            axes[1].push(-2);
        }

        // Diagonal top left
        if stone_y + i < 19 && stone_x + i < 19 {
            axes[2].push(board[(stone_x + i) as usize][(stone_y + i) as usize]);
        } else {
            axes[2].push(-2);
        }
        if stone_y - i >= 0 && stone_x - i >= 0 {
            axes[2].insert(0, board[(stone_x - i) as usize][(stone_y - i) as usize]);
        } else {
            axes[2].insert(0, -2);
        }

        // Diagonal top right
        if stone_y + i < 19 && stone_x - i >= 0 {
            axes[3].push(board[(stone_x - i) as usize][(stone_y + i) as usize]);
        } else {
            axes[3].push(-2);
        }
        if stone_y - i >= 0 && stone_x + i < 19 {
            axes[3].insert(0, board[(stone_x + i) as usize][(stone_y - i) as usize]);
        } else {
            axes[3].insert(0, -2);
        }
    }
    return axes;
}

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

fn check_move_is_in_sandwich_in_axe(axe: &Vec<i8>, player: i8) -> bool {
    let opponent: i8 = -player;
    if (axe[3] == opponent || (axe[3] == player && axe[2] == opponent))
        && (axe[5] == opponent || (axe[5] == player && axe[6] == opponent))
    {
        return true;
    } else {
        return false;
    }
}

fn check_move_is_eating_in_axe(axe: &Vec<i8>, player: i8) -> i8 {
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

use std::time::Instant;

pub fn count_biggest_alignment(state: State) -> i32 {
    let axes = create_axes_from_stone_position(&state);
    let player = state.player_to_play;
    let mut count_eat = 0;
    println!("vertical x = {:?}", axes[0]);
    println!("horizontal y = {:?}", axes[1]);
    println!("diagonal top left = {:?}", axes[2]);
    println!("diagonal top right = {:?}", axes[3]);
    let start_time_of_function = Instant::now();

    for axe in &axes {
        let start_time = Instant::now();
        if check_move_is_in_sandwich_in_axe(&axe, player) {
            let end_time = Instant::now();
            println!("the var is in sandwich... wrong move.");
            println!("time to process {:?}", end_time.duration_since(start_time));
            return 0;
        }
        let end_time = Instant::now();
        println!(
            "time to process check_move is in sandwich {:?}",
            end_time.duration_since(start_time)
        );

        let start_time = Instant::now();
        count_eat += check_move_is_eating_in_axe(&axe, player);
        let end_time = Instant::now();
        println!(
            "time to process check_move_is_eating_in_axe {:?}",
            end_time.duration_since(start_time)
        );
    }

    let start_time = Instant::now();
    println!(
        "biggest alignment in axe : {:?}",
        check_move_biggest_alignment_in_axes(&axes, player)
    );
    let end_time = Instant::now();
    println!(
        "time to process biggest alignment {:?}",
        end_time.duration_since(start_time)
    );

    println!("eated piece = {:?}", count_eat);
    let end_time_of_function = Instant::now();
    println!(
        "time to process all function {:?}",
        end_time_of_function.duration_since(start_time_of_function)
    );
    println!();

    return 0;
}
