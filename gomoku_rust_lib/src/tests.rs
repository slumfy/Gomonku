//! Rust module use by pytest

extern crate pyo3;

use crate::bitboard_operations::apply_bit;
use crate::bitboards::Bitboards;
use crate::global_var;
use crate::print::print_board_from_bitboard;
use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::HashMap;

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

use crate::check_move::check_is_unblockable_five;

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
    print_board_from_bitboard(&bitboards);

    assert_eq!(check_is_unblockable_five(&mut bitboards, 180, 0, 1), false);
}
