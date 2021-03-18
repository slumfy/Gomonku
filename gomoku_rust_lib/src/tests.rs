extern crate pyo3;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;

#[macro_export]
#[pyfunction]
pub fn test_returning_dict_to_python(
    mut map: Vec<Vec<i32>>,
    player: i32,
    x: i32,
    y: i32,
) -> PyResult<PyObject> {
    println!("In test_getting_dict_from_python.");

    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);

    dict.set_item("eated_piece", 10);
    dict.set_item("board", &map);
    Ok(dict.to_object(py))
}

fn update_map_board(mut map: &mut Vec<Vec<i32>>, player: i32, x: i32, y: i32) {
    map[x as usize][y as usize] = player;
}

fn update_player(player: &mut i32, new_player: i32) {
    *player = new_player;
}

#[macro_export]
#[pyfunction]
pub fn test_updating_from_other_function(
    mut map: Vec<Vec<i32>>,
    mut player: i32,
    x: i32,
    y: i32,
) -> PyResult<PyObject> {
    println!("In test_updating_map_from_other_function.");

    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);
    update_map_board(&mut map, player, x, y);
    update_player(&mut player, 2);
    update_map_board(&mut map, player, 0, 1);
    dict.set_item("eated_piece", 10);
    dict.set_item("board", &map);
    Ok(dict.to_object(py))
}

#[macro_export]
#[pyfunction]
pub fn test_get_PyObject(python_obj: PyObject) {}
