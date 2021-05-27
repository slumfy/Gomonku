use crate::bitboard_operations::apply_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::check_move::check_is_capturable;
use crate::data_struct;
use crate::data_struct::Bitboards;
use crate::global_var;
use crate::heuristic_ratios;
use crate::print::print_board_from_bitboard;
use crate::print::print_pos_in_human_format;
use crate::state;

pub fn test_check_is_capturable() {
    // First test, just testing AI will choose undefeatable 5
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 182, global_var::PLAYER_BLACK_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        160,
        0,
        0,
        0,
        0,
        (0, 0),
    );
    state.axes = create_bits_axes_from_pos(182, &mut bitboards);
    // Testing for white, pointless because its a black move. Should be false
    assert_eq!(check_is_capturable(&state.axes[0], &state.axes[1]), false);
    // Testing for black
    assert_eq!(check_is_capturable(&state.axes[1], &state.axes[0]), true);
}
