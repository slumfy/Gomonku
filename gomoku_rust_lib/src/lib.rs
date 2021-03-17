use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod check;

#[pyfunction]
fn rust_calculating_move(map: [[i32; 19]; 19], player: i32, x: i32, y: i32) -> PyResult<i32> {
    // println!("{} player ", player);
    // println!("{} x ", x);
    // println!("{} y ", y);
    // println!("{:?} map ", map);
    Ok(check::check_win_position(map, player, x, y))
}

/// A Python module implemented in Rust.
#[pymodule]
fn gomoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_calculating_move, m)?)?;
    Ok(())
}
