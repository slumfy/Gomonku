use pyo3::prelude::*;

#[macro_export]
#[pyfunction]
pub fn toto(mut map: [[i32; 19]; 19], player: i32, x: i32, y: i32) -> PyResult<i32> {
    // println!("{} player ", player);
    // println!("{} x ", x);
    // println!("{} y ", y);
    // println!("{:?} map ", map);
	println!(" fonction from another file ");
    Ok(0)
}