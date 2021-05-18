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
    // First test, just testing AI will choose undefeatable 5
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);
    assert_eq!(ai_returned_move.0, 175);

    // Second test, testing if AI choose simple blocking two instead of double block three
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 182, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 10, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 11, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 183, global_var::PLAYER_BLACK_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        180,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);
    assert_eq!(ai_returned_move.0, 9);

    // Third test, testing blocking a three and blocking a two in the same time instead of blocking four and doing a three.
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 182, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 198, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 202, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 217, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 221, global_var::PLAYER_WHITE_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        180,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);

    assert_eq!(ai_returned_move.0, 183);

    // Test 4, testing AI not blocking a double three.
    println!("test 4");
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 201, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 220, global_var::PLAYER_WHITE_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        180,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);

    assert_eq!(ai_returned_move.0, 161);

    // Test 5, testing AI chosing move that cannot be captured.
    println!("test 5");
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 182, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 183, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 60, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 41, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 42, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 24, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 62, global_var::PLAYER_WHITE_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        180,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);
    print_board_from_bitboard(&bitboards);

    assert_eq!(ai_returned_move.0, 184);

    // Test 6, testing AI not doing a three blocking because it's actually not preventing a capture "--0X-X--".
    println!("test 6");
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 182, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 183, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 41, global_var::PLAYER_WHITE_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_BLACK_NB,
        183,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);

    assert_eq!(ai_returned_move.0, 42);

    // Test 7, testing AI not doing a three blocking because it's actually not preventing a capture "--0-XX--".
    println!("test 7");
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 178, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 41, global_var::PLAYER_WHITE_NB);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_BLACK_NB,
        183,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);

    assert_eq!(ai_returned_move.0, 42);

    // Test 8, testing AI not doing a three blocking because it's actually not preventing a capture "--XX-0-".
    println!("test 8");
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 183, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 41, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 300, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 301, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 299, global_var::PLAYER_BLACK_NB);

    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_BLACK_NB,
        183,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);

    assert_eq!(ai_returned_move.0, 302);

    // Test 9, testing AI not doing a three blocking because it's actually not preventing a capture "--XX-0-".
    println!("test 9");
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 300, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 301, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 178, global_var::PLAYER_BLACK_NB);

    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_BLACK_NB,
        180,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);

    assert_eq!(ai_returned_move.0, 182);

    // Test 10, testing AI should preventing a capture "--0XX--".
    println!("test 10");
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 181, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 182, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 183, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 300, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 301, global_var::PLAYER_WHITE_NB);

    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_BLACK_NB,
        180,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);

    assert_eq!(ai_returned_move.0, 184);

    // Test 11, testing AI should preventing a capture "--X0XX--".
    println!("test 11");
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    apply_bit(&mut bitboards, 180, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 181, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, 182, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 183, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 300, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, 301, global_var::PLAYER_WHITE_NB);

    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        global_var::PLAYER_BLACK_NB,
        180,
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
        heuristic_ratios::HEURISTIC_MIN_VALUE,
        heuristic_ratios::HEURISTIC_MAX_VALUE,
        1,
    );
    let ai_returned_move = algorithms::return_move(&mut state);

    assert_eq!(ai_returned_move.0, 184);
}
