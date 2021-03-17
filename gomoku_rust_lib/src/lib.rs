use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{wrap_pyfunction, wrap_pymodule};
mod check;
mod tata;
use crate::tata::__pyo3_get_function_toto;

#[pyfunction]
fn win_check(mut map: [[i32; 19]; 19], player: i32, x: i32, y: i32) -> PyResult<i32> {
    // println!("{} player ", player);
    // println!("{} x ", x);
    // println!("{} y ", y);
    // println!("{:?} map ", map);
    Ok(check::check_win_position(map, player, x, y))
}

#[pyfunction]
fn eat_check(mut map: [[i32; 19]; 19], player: i32, x: i32, y: i32) -> PyResult<i32> {
    // println!("{} player ", player);
    // println!("{} x ", x);
    // println!("{} y ", y);
    // println!("{:?} map ", map);
    check::check_eat_position(map, player, x, y);
    Ok(1)
}

#[pyfunction]
fn wrong_check(mut map: [[i32; 19]; 19], player: i32, x: i32, y: i32) -> PyResult<i32> {
    // println!("{} player ", player);
    // println!("{} x ", x);
    // println!("{} y ", y);
    // println!("{:?} map ", map);
    Ok(check::check_wrong_position(map, player, x, y))
}

#[pyfunction]
fn place_stone(mut map: [[i32; 19]; 19], player: i32, x: i32, y: i32) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);
    println!("{} player ", player);
    println!("{} x ", x);
    println!("{} y ", y);
    println!("{:?} map ", map);
    if check::check_wrong_position(map, player, x, y) == 1 {
        dict.set_item("game_status", -1);
    } else {
        dict.set_item("game_status", 0);
    }
    Ok(dict.to_object(py))
}

#[pyfunction]
pub fn test_getting_dict_from_python(dict: PyObject) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    println!("In test_getting_dict_from_python.");
}

/// A Python module implemented in Rust.
#[pymodule]
pub fn gomoku_tests(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_getting_dict_from_python, m)?)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn gomoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(win_check, m)?)?;
    m.add_function(wrap_pyfunction!(wrong_check, m)?)?;
    m.add_function(wrap_pyfunction!(eat_check, m)?)?;
    m.add_function(wrap_pyfunction!(place_stone, m)?)?;
	m.add_function(wrap_pyfunction!(toto, m)?)?;
    m.add_wrapped(wrap_pymodule!(gomoku_tests))?;
    Ok(())
}
