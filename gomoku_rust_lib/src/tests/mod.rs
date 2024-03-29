//! Rust module use by pytest

extern crate pyo3;

mod test_algorithm_benchmark;
mod test_check_ai_move;
mod test_check_is_capturable;
mod test_check_is_unblockable_five;
mod test_check_potential_winning_alignment;
mod test_is_on_axe;
mod test_pattern_axes_finder;
mod test_pattern_blocker;
mod test_print;
use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::HashMap;
use test_check_ai_move::test_ai_move;
use test_check_is_capturable::test_check_is_capturable;
use test_check_is_unblockable_five::test_check_pattern_is_not_capturable_five;
use test_check_potential_winning_alignment::test_check_potential_winning_alignment;
use test_is_on_axe::test_is_on_axe;
use test_pattern_axes_finder::test_pattern_axes_finder;
use test_pattern_blocker::test_blockers;

#[pyfunction]
pub fn pytest_algorithm_benchmark() {
    test_algorithm_benchmark::test_negamax_benchmark();
}

#[pyfunction]
pub fn pytest_print_pos_in_human_format() {
    test_print::test_print_pos_in_human_format();
}

#[pyfunction]
pub fn pytest_ai_move() {
    test_ai_move();
}

#[pyfunction]
pub fn pytest_check_is_capturable() {
    test_check_is_capturable();
}

#[pyfunction]
pub fn pytest_check_potential_winning_alignment() {
    test_check_potential_winning_alignment();
}

#[pyfunction]
pub fn pytest_pattern_axes_finder() {
    test_pattern_axes_finder();
}

#[pyfunction]
pub fn pytest_is_on_axe() {
    test_is_on_axe()
}

#[pyfunction]
pub fn pytest_check_pattern_is_not_capturable_five() {
    test_check_pattern_is_not_capturable_five()
}

#[pyfunction]
pub fn pytest_test_blockers() {
    test_blockers()
}

#[pyfunction]
pub fn pytest_returning_dict_to_python(
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

#[pyfunction]
pub fn pytest_updating_from_other_function(
    mut board: Vec<Vec<i32>>,
    mut player: i32,
    x: i32,
    y: i32,
) -> PyResult<PyObject> {
    fn update_map_board(board: &mut Vec<Vec<i32>>, player: i32, x: i32, y: i32) {
        board[x as usize][y as usize] = player;
    }

    fn update_player(player: &mut i32, new_player: i32) {
        *player = new_player;
    }
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
pub fn pytest_get_pydict(py_obj: HashMap<String, i32>) {
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
