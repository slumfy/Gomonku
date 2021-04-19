use crate::bitboard_operations::apply_bit;
use crate::data_struct::Bitboards;
use crate::check_move::check_free_development;
use crate::global_var;
// use crate::print::print_board_from_bitboard;
use crate::state::create_new_state;

pub fn test_check_free_development() {
    let mut development_value: i32;

    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    // First try in the middle, should be 4 * nb_of_axes * 2 = 32
    let mut bit_current_move_pos: usize = 180;
    let mut state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    development_value = check_free_development(&state);
    assert_eq!(development_value, 32);

    // Try in the top right corner, should be 4 * 6 - (4 * 4 * 2) = -8
    bit_current_move_pos = 0;
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    development_value = check_free_development(&state);
    assert_eq!(development_value, -8);

    // Try in the top left corner, should be 4 * 6 - (4 * 4 * 2) = -8
    bit_current_move_pos = 18;
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    development_value = check_free_development(&state);
    assert_eq!(development_value, -8);

    // Try in the bottom left corner, should be 4 * 6 - (4 * 4 * 2) = -8
    bit_current_move_pos = global_var::BOARD_MAX_LIMITS;
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    development_value = check_free_development(&state);
    assert_eq!(development_value, -8);

    // Try in the bottom right corner, should be 4 * 6 - (4 * 4 * 2) = -8
    bit_current_move_pos = 342;
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    development_value = check_free_development(&state);
    assert_eq!(development_value, -8);

    // Try have friend stone in axes
    bit_current_move_pos = 180;
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 1,
        global_var::PLAYER_WHITE_NB,
    );
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos,
        global_var::PLAYER_WHITE_NB,
    );
    development_value = check_free_development(&state);
    assert_eq!(development_value, 36);

    bit_current_move_pos = 180;
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 1,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 1,
        global_var::PLAYER_WHITE_NB,
    );
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos,
        global_var::PLAYER_WHITE_NB,
    );
    development_value = check_free_development(&state);
    assert_eq!(development_value, 40);

    bit_current_move_pos = 180;
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 1,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 4,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 1,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 4,
        global_var::PLAYER_WHITE_NB,
    );
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    apply_bit(&mut bitboards, bit_current_move_pos, 1);
    development_value = check_free_development(&state);
    assert_eq!(development_value, 48);

    bit_current_move_pos = 180;
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 1,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 4,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 19,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 19,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 20,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 40,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 1,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 4,
        global_var::PLAYER_WHITE_NB,
    );
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    apply_bit(&mut bitboards, bit_current_move_pos, 1);
    development_value = check_free_development(&state);
    assert_eq!(development_value, 64);

    // Test ennemy stone block

    bit_current_move_pos = 180;
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 1,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 4,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 19,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 19,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 20,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 40,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 1,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 4,
        global_var::PLAYER_WHITE_NB,
    );

    // Enemy stone blocking 4 free potential space
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 20,
        global_var::PLAYER_BLACK_NB,
    );
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    apply_bit(&mut bitboards, bit_current_move_pos, 1);
    development_value = check_free_development(&state);
    assert_eq!(development_value, 60);

    bit_current_move_pos = 180;
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 1,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 4,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 19,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 19,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 20,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 40,
        global_var::PLAYER_WHITE_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 1,
        global_var::PLAYER_WHITE_NB,
    );
    // Enemy stone blocking 2 free potential space and a friendly stone
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 2,
        global_var::PLAYER_BLACK_NB,
    );
    apply_bit(
        &mut bitboards,
        bit_current_move_pos - 4,
        global_var::PLAYER_WHITE_NB,
    );

    // Enemy stone blocking 4 free potential space
    apply_bit(
        &mut bitboards,
        bit_current_move_pos + 20,
        global_var::PLAYER_BLACK_NB,
    );
    state = create_new_state(
        &mut bitboards,
        global_var::PLAYER_WHITE_NB,
        bit_current_move_pos,
        0,
        0,
        (0, 0),
		5
    );
    apply_bit(&mut bitboards, bit_current_move_pos, 1);
    development_value = check_free_development(&state);
    assert_eq!(development_value, 53);
}
