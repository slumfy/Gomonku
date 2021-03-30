#[path = "check.rs"]
mod check;
use crate::check::check_alignment_for_given_pos;
use crate::check::check_is_wrong_move;
use crate::check::create_axes_from_stone_position;
use crate::state::State;

pub fn heuristic(state: &mut State) -> i32 {
    let mut value = 0i32;

    let axes = create_axes_from_stone_position(
        state,
        state.current_move.0,
        state.current_move.1,
        state.current_player,
    );
    let player = state.current_player;
    if check_is_wrong_move(state, &axes) < 0 {
        return -1000;
    }

    let board_check = check::checking_move_biggest_alignment_and_stone_captured(state); // current alignement and current eat_value
    if state.current_player == 1 {
        state.white_captured_stone += board_check["stone_captured"];
        if state.white_captured_stone >= 10 {
            value += 1000
        } else {
            value += state.white_captured_stone as i32 * state.white_captured_stone as i32;
        }
    } else {
        state.black_captured_stone += board_check["stone_captured"];
        if state.black_captured_stone >= 10 {
            value += 1000
        } else {
            value += state.black_captured_stone as i32 * state.black_captured_stone as i32;
        }
    }
    if state.win_move.len() > 0 {
        for winmove in 0..state.win_move.len() {
            if check_alignment_for_given_pos(
                &state,
                state.win_move[winmove].0 .0,
                state.win_move[winmove].0 .1,
                state.win_move[winmove].1,
            ) == true
            {
                if state.win_move[winmove].1 == state.current_player {
                    state.win_state += 1;
                    value += 1000;
                } else {
                    state.win_state += -1;
                    value -= 1000;
                }
                break;
            }
        }
    }
    value += board_check["biggest_alignment"] as i32;
    if board_check["biggest_alignment"] == 5 {
        state
            .win_move
            .push((state.current_move, state.current_player));
    }

    return value;
}
