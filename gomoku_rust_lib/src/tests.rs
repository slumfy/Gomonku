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
