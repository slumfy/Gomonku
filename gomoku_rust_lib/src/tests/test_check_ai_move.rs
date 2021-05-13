use crate::algorithms;
use crate::bitboard_operations::apply_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::check_move::check_potential_winning_alignment;
use crate::data_struct;
use crate::data_struct::Bitboards;
use crate::global_var;
use crate::heuristic_ratios;
use crate::print::print_board_from_bitboard;
use crate::print::print_pos_in_human_format;
use crate::state;

pub fn test_ai_move() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 41, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 45, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 64, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 85, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 102, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 103, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 104, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 121, global_var::PLAYER_BLACK_NB);
    // apply_bit(&mut bitboards, 123, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 139, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 140, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 141, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 142, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 157, global_var::PLAYER_BLACK_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        160,
        0,
        0,
        (0, 0),
        0,
    );
    let depth = 1;
    state.axes = create_bits_axes_from_pos(160, &mut bitboards);
    algorithms::negamax(
        &mut state,
        depth,
        heuristic_ratios::HEURISTIC_MIN_VALUE + 1,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);
    println!("AI MOVE : ");
    print_pos_in_human_format(ai_returned_move.0);
    print_board_from_bitboard(&bitboards);
}
