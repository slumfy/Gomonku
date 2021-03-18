use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::FromPyObject;

#[pyclass]
struct RustDict {
    player: i32,
    x: i32,
    y: i32,
}

#[macro_export]
#[pyfunction]
pub fn test_getting_dict_from_python(python_dict: PyDict) {
    println!("In test_getting_dict_from_python.");

    let gil = Python::acquire_gil();
    let py = gil.python();
    println!(python_dict.get_item("player": i32));
}
