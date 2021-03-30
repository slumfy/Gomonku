use crate::check::check_is_wrong_move;
use crate::check::check_move_is_capturing_stone_in_axe;
use crate::global_var;
use crate::state::State;
use crate::utils::create_axes_from_stone_position;

pub fn heuristic(state: &mut State) -> i32 {
    let mut value = 0i32;
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

    for axe in &axes {
        // stone_captured += check_move_is_capturing_stone_in_axe(&axe, player);
        value += heuristic_get_alignment_and_flanks_values_from_axe(&axe, player);
    }
    if state.current_player == 1 {
        state.white_captured_stone += stone_captured;
        if state.white_captured_stone >= 10 {
            return global_var::HEURISTIC_MAX_VALUE;
        } else {
            // value += state.white_captured_stone as i32 * state.white_captured_stone as i32;
        }
    } else {
        state.black_captured_stone += stone_captured;
        if state.black_captured_stone >= 10 {
            return global_var::HEURISTIC_MAX_VALUE;
        } else {
            // value += state.black_captured_stone as i32 * state.black_captured_stone as i32;
        }
    }

    println!("Value here is {:?}", value);
    return value;
}

fn heuristic_get_alignment_and_flanks_values_from_axe(axe: &Vec<i8>, player: i8) -> i32 {
    let mut alignment: i8 = 0;
    let mut flanked: i8 = 0;
    let mut free_space: i8 = 0;
    let opponent: i8 = -player;
    let mut value: i32 = 0;

    for i in vec![3, 2, 1, 0] {
        if axe[i] == player {
            alignment += 1;
        } else if axe[i] == opponent {
            flanked += 1;
            break;
        } else if axe[i] == -2 {
            free_space += 1;
        } else {
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
            free_space += 1;
        } else {
            break;
        }
    }
    alignment *= alignment;
    if free_space == 2 {
        alignment = 0;
    }
    value = (alignment as i32 * alignment as i32) * 2 + free_space as i32;
    return value;
}
