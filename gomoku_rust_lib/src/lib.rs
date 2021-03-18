use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{wrap_pyfunction, wrap_pymodule};
mod check;
mod tests;
use crate::tests::__pyo3_get_function_test_getting_dict_from_python;

#[pyfunction]
fn place_stone(mut map: [[i32; 19]; 19], player: i32, x: i32, y: i32) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);
	let mut eated_piece= 0i32;
	let wrong_status = check::check_wrong_position(map, player, x, y);
    if wrong_status == 1 {
        dict.set_item("game_status", -1);
    } else {
        dict.set_item("game_status", 0);
    }
	if wrong_status == 0 {
		map[x as usize][y as usize] = player;
		eated_piece = check::check_eat_position(map, player, x, y);
	}
	if check::check_win_position(map, player, x, y) == 5 {
		dict.set_item("wining_position",(x,y));
	}
	dict.set_item("eated_piece",eated_piece);
	// dict.set_item("board", map);
	println!("{:?} map ", map);
    Ok(dict.to_object(py))
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
    m.add_function(wrap_pyfunction!(place_stone, m)?)?;
    m.add_wrapped(wrap_pymodule!(gomoku_tests))?;
    Ok(())
}
