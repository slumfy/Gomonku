//! Rust module use by pytest

extern crate pyo3;

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

// #[pyfunction]
// pub fn test_double_triple() {
//     let player = 1;
//     let current_move = player;
//     let mut axes: Vec<Vec<i8>> = vec![];
//     // test no double triple, empty map
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test in same axe not double triple
//     axes = vec![];

//     axes.push(vec![0, 0, 1, 1, current_move, 1, 0, 1, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test not double triple
//     axes = vec![];
//     axes.push(vec![0, 1, 1, 0, current_move, 1, 1, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test in two different axes double triple
//     axes = vec![];
//     axes.push(vec![0, 0, 1, 1, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 1, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), true);

//     // test in two different axes double triple
//     axes = vec![];
//     axes.push(vec![0, 1, 0, 1, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 1, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), true);

//     // test in two different axes double triple
//     axes = vec![];
//     axes.push(vec![0, 1, 1, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 1, 0, 1, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), true);

//     // test in more than two different axes double triple
//     axes = vec![];
//     axes.push(vec![0, 1, 1, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 1, 0, 0]);
//     axes.push(vec![0, 0, 1, 1, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 1, 0, 1, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), true);

//     // test is not double triple
//     axes = vec![];
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 1, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 1, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test is not double triple
//     axes = vec![];
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test is not double triple
//     axes = vec![];
//     axes.push(vec![1, 0, 0, 0, current_move, 0, 0, 0, 1]);
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 0, 0, 1]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test is not double triple because blocked
//     axes = vec![];
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 1, -1, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 1, 1, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test is double triple
//     axes = vec![];
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 1, 0, -1]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 1, 1, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), true);

//     // test is not double triple because more than 3
//     axes = vec![];
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 1, 1, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 1, 1, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test is not double triple because more than 3
//     axes = vec![];
//     axes.push(vec![0, 0, 0, 1, current_move, 1, 1, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 1, 1, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);

//     // test is not double triple because blocked
//     axes = vec![];
//     axes.push(vec![0, 0, 0, 0, current_move, 1, 1, -1, -1]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 1, 1, 0, current_move, 0, 0, 0, 0]);
//     axes.push(vec![0, 0, 0, 0, current_move, 0, 0, 0, 0]);

//     assert_eq!(check_move_is_double_triple(&axes, player), false);
// }
