use crate::check::check_is_wrong_move;
use crate::check::check_move_is_capturing_stone_in_axe;
use crate::global_var;
use crate::state::State;
use crate::utils::create_axes_from_stone_position;
use std::collections::HashMap;

static FREE_SPACE_RATIO: i32 = 1;
static ALIGNMENT_RATIO: i32 = 5;
static CAPTURE_RATIO: i32 = 15;

pub fn fibonacci_reccursive(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2),
    }
}

pub fn heuristic(state: &mut State) -> i32 {
    let mut value = 0i32;
    let mut alignment_value = 0i32;
    let player = state.current_player;
    let mut stone_captured = 0;
    let axes = create_axes_from_stone_position(
        state,
        state.current_move.0,
        state.current_move.1,
        state.current_player,
    );
    if check_is_wrong_move(state, &axes) < 0 {
        return global_var::HEURISTIC_MIN_VALUE;
    }

    let mut alignment_value: i32 = 0;
    let mut flanked: i32 = 0;
    let mut free_space: i32 = 0;
    for axe in &axes {
        stone_captured += check_move_is_capturing_stone_in_axe(&axe, player);

        // Getting alignment value
        let alignment_value_dict = heuristic_get_alignment_and_flanks_values_from_axe(&axe, player);
        flanked += alignment_value_dict["flanked"];
        free_space += alignment_value_dict["free_space"];

        if alignment_value_dict["flanked"] != 2 {
            alignment_value += fibonacci_reccursive(alignment_value_dict["alignment"] as i32 + 3)
                as i32
                * ALIGNMENT_RATIO;
        }

        // !Getting alignment value!
    }
    println!(
        "alignment first value = {:?} alignment value = {:?}",
        alignment_value, alignment_value as i32
    );
    println!("free_space value = {:?}", free_space * FREE_SPACE_RATIO);

    value += alignment_value as i32 + free_space * FREE_SPACE_RATIO;

    if state.current_player == 1 {
        state.white_captured_stone += stone_captured;
        if state.white_captured_stone >= 10 {
            return global_var::HEURISTIC_MAX_VALUE;
        } else {
            value += stone_captured as i32 * CAPTURE_RATIO;
            println!(
                "white_capture_value = {:?}",
                stone_captured as i32 * CAPTURE_RATIO
            );
        }
    } else {
        state.black_captured_stone += stone_captured;
        if state.black_captured_stone >= 10 {
            return global_var::HEURISTIC_MAX_VALUE;
        } else {
            println!(
                "black_capture_value = {:?}",
                stone_captured as i32 * CAPTURE_RATIO
            );

            value += stone_captured as i32 * CAPTURE_RATIO;
        }
    }
    // println!("value = {:?}", value);

    return value;
}

fn heuristic_get_alignment_and_flanks_values_from_axe(
    axe: &Vec<i8>,
    player: i8,
) -> HashMap<String, i32> {
    let mut alignment: i8 = 1;
    let mut flanked: i8 = 0;
    let mut free_space: i8 = 0;
    let opponent: i8 = -player;
    let mut alignment_value_dict: HashMap<String, i32> = HashMap::new();

    for i in vec![3, 2, 1, 0] {
        if axe[i] == player {
            alignment += 1;
        } else if axe[i] == opponent {
            flanked += 1;
            break;
        } else if axe[i] == -2 {
            break;
        } else {
            free_space += 1;
            break;
        }
    }
    for i in 5..9 {
        if axe[i] == player {
            alignment += 1;
        } else if axe[i] == opponent {
            flanked += 1;
            break;
        } else if axe[i] == -2 {
            break;
        } else {
            free_space += 1;
            break;
        }
    }

    alignment_value_dict.insert(String::from("alignment"), alignment as i32);
    alignment_value_dict.insert(String::from("free_space"), free_space as i32);
    alignment_value_dict.insert(String::from("flanked"), flanked as i32);
    return alignment_value_dict;
}
