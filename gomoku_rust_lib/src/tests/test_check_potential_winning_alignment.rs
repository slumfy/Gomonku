use crate::bitboard_operations::apply_bit;
use crate::bitboard_operations::remove_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::check_move::check_potential_winning_alignment;
use crate::data_struct;
use crate::data_struct::Bitboards;
use crate::global_var;
use crate::print::print_board_from_bitboard;
use crate::state;

pub fn test_check_potential_winning_alignment() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    // No blocker
    let mut pos = 180;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        pos,
        0,
        0,
        (0, 0),
        0,
    );
    state.axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let mut ret = check_potential_winning_alignment(&state);
    assert_eq!(ret, [true, true, true, true]);

    // One stone block
    println!(" 2 ");
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_BLACK_NB);
    state = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        pos,
        0,
        0,
        (0, 0),
        0,
    );
    state.axes = create_bits_axes_from_pos(pos, &mut bitboards);
    ret = check_potential_winning_alignment(&state);
    assert_eq!(ret, [true, true, true, true]);

    println!(" 3 ");
    // stone in up left corner
    pos = 0;
    state = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        pos,
        0,
        0,
        (0, 0),
        0,
    );
    state.axes = create_bits_axes_from_pos(pos, &mut bitboards);
    ret = check_potential_winning_alignment(&state);
    assert_eq!(ret, [true, true, false, true]);

    println!(" 4 ");
    // stone in up left corner with a blocker in pos 1
    pos = 0;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_BLACK_NB);
    state = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        pos,
        0,
        0,
        (0, 0),
        0,
    );
    state.axes = create_bits_axes_from_pos(pos, &mut bitboards);
    print_board_from_bitboard(&bitboards);
    ret = check_potential_winning_alignment(&state);
    assert_eq!(ret, [true, true, false, false]);

    println!(" 5 ");
    // stone in middle stuck by two blocker in pos 1 and pos -1
    pos = 180;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos - 1, global_var::PLAYER_BLACK_NB);
    state = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        pos,
        0,
        0,
        (0, 0),
        0,
    );
    state.axes = create_bits_axes_from_pos(pos, &mut bitboards);
    ret = check_potential_winning_alignment(&state);
    print_board_from_bitboard(&bitboards);
    assert_eq!(ret, [true, true, true, false]);
}
