//! Rust module use by pytest

extern crate pyo3;

use crate::bitboard_operations::apply_bit;
use crate::bitboard_operations::remove_bit;
use crate::bitboards::Bitboards;
use crate::check_move::check_free_development;
use crate::check_move::check_is_unblockable_five;
use crate::global_var;
use crate::print::print_board_from_bitboard;
use crate::state::create_new_state;
use crate::state::State;
use crate::utils::is_on_axe;
use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::HashMap;

#[pyfunction]
pub fn test_check_free_development() {
    let mut development_value: i32 = 0;

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
    );
    apply_bit(&mut bitboards, bit_current_move_pos, 1);
    development_value = check_free_development(&state);
    assert_eq!(development_value, 53);
}

#[pyfunction]
pub fn test_is_on_axe() {
    // Test line
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MIN_LIMITS,
            1,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MIN_LIMITS,
            1,
            -1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MAX_LIMITS,
            1,
            -1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MAX_LIMITS,
            1,
            1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MAX_LIMITS,
            1,
            -19
        ),
        false
    );

    // Test column
    assert_eq!(is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 0, 1, 1), true);
    assert_eq!(is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 0, 2, 1), true);
    assert_eq!(is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 0, 3, 1), true);
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[1],
            global_var::BOARD_MIN_LIMITS,
            4,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[1],
            global_var::BOARD_MIN_LIMITS,
            5,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[1],
            global_var::BOARD_MIN_LIMITS,
            1,
            -1
        ),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 1, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 2, -1),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 3, -1),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 4, -1),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 5, -1),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 6, -1),
        false
    );

    // Test diagonal right - left
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MIN_LIMITS,
            1,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MIN_LIMITS,
            2,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MIN_LIMITS,
            1,
            -1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MAX_LIMITS,
            1,
            1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MAX_LIMITS,
            2,
            1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MAX_LIMITS,
            3,
            1
        ),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 1, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 2, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 3, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 4, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 5, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 1, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 2, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 3, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 4, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 5, -1),
        true
    );
}

#[pyfunction]
pub fn test_returning_dict_to_python(
    board: Vec<Vec<i32>>,
    player: i32,
    x: i32,
    y: i32,
) -> PyResult<PyObject> {
    println!("In test_getting_dict_from_python.");

    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);

    dict.set_item("eated_piece", 10)?;
    dict.set_item("board", &board)?;
    dict.set_item("player", player)?;
    dict.set_item("x", x)?;
    dict.set_item("y", y)?;
    Ok(dict.to_object(py))
}

fn update_map_board(board: &mut Vec<Vec<i32>>, player: i32, x: i32, y: i32) {
    board[x as usize][y as usize] = player;
}

fn update_player(player: &mut i32, new_player: i32) {
    *player = new_player;
}

#[pyfunction]
pub fn test_updating_from_other_function(
    mut board: Vec<Vec<i32>>,
    mut player: i32,
    x: i32,
    y: i32,
) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);
    update_map_board(&mut board, player, x, y);
    update_player(&mut player, 2);
    update_map_board(&mut board, player, 0, 1);
    dict.set_item("eated_piece", 10)?;
    dict.set_item("board", &board)?;
    Ok(dict.to_object(py))
}

#[pyfunction]
pub fn test_get_pydict(py_obj: HashMap<String, i32>) {
    assert_eq!(
        py_obj.get_key_value(&String::from("map")),
        Some((&String::from("map"), &0i32))
    );
    assert_eq!(
        py_obj.get_key_value(&String::from("player")),
        Some((&String::from("player"), &1i32))
    );
    assert_eq!(
        py_obj.get_key_value(&String::from("x")),
        Some((&String::from("x"), &3i32))
    );
    assert_eq!(
        py_obj.get_key_value(&String::from("y")),
        Some((&String::from("y"), &0i32))
    );
}

#[pyfunction]
pub fn test_check_is_unblockable_five() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let mut pos = 0;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 0, 3, 1), true);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in colone first pos
    let mut pos = 19;
    apply_bit(&mut bitboards, 0, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 19, 3, 1), false);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in colone second pos
    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, 1, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 19, 3, 1), false);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in colone third pos
    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, 2, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 19, 3, 1), false);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in colone fourth pos
    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, 3, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 19, 3, 1), false);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];
    // Can capture in colone last pos
    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, 4, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 19, 3, 1), false);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 19, 3, 1), true);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in diagonal
    let mut pos = 20;
    apply_bit(&mut bitboards, 0, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 20, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 20, 3, 1), false);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 20;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 20, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 20, 3, 1), true);

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Try diagonal alignment
    let mut pos = 180;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 20;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 180, 3, 1), true);

    // Can capture in colone
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 180;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;
    apply_bit(&mut bitboards, pos - 19, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);

    assert_eq!(check_is_unblockable_five(&mut bitboards, 180, 0, 1), false);

    // Cannot capture in colone because there is already a black stone
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 180;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;
    apply_bit(&mut bitboards, pos - 19, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 38, global_var::PLAYER_BLACK_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);

    assert_eq!(check_is_unblockable_five(&mut bitboards, 180, 0, 1), true);

    // Cannot capture in line because bigger than 2 whites stone
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 180;
    let mut black_pos = pos - 1;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    black_pos += 19;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    black_pos += 19;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    black_pos += 19;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    black_pos += 19;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);

    assert_eq!(check_is_unblockable_five(&mut bitboards, 180, 0, 1), true);

    // Reverse check is unblockable five in two ways
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 6;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    remove_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 0, 3, 1), true);
    apply_bit(&mut bitboards, 7, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 0, 3, 1), true);

    // Reverse check is unblockable five in two ways
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 300;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    remove_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 0, 3, 1), true);
    apply_bit(&mut bitboards, 319, global_var::PLAYER_WHITE_NB);
    assert_eq!(check_is_unblockable_five(&mut bitboards, 0, 3, 1), true);
}
