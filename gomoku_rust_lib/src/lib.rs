use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod check;

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

/// A Python module implemented in Rust.
#[pymodule]
fn gomoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(win_check, m)?)?;
	m.add_function(wrap_pyfunction!(wrong_check, m)?)?;
	m.add_function(wrap_pyfunction!(eat_check, m)?)?;
    Ok(())
}
