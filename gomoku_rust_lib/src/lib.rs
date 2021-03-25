use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{wrap_pyfunction, wrap_pymodule};
// use std::time::Instant;
mod check;
use check::checking_move;

mod negamax;
mod state;
mod tests;
use crate::tests::__pyo3_get_function_test_double_triple;
use crate::tests::__pyo3_get_function_test_get_pydict;
use crate::tests::__pyo3_get_function_test_returning_dict_to_python;
use crate::tests::__pyo3_get_function_test_updating_from_other_function;

// static _ALPHABET: [char; 26] = [
//     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
//     'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
// ];

#[pyfunction]
fn negamax(board: Vec<Vec<i32>>, player: i32, x: i32, y: i32) -> PyResult<((i32, i32), i32)> {
    let mut mutboard: Vec<Vec<i32>> = board;
    let mut state: state::State = state::create_new_state(&mut mutboard, player, (x, y));
    let start = Instant::now();
    let value = negamax::negamax(&mut state, 2, -1000, 1000, player);
    let ai_move = negamax::return_move(&mut state, value);
    let end = Instant::now();
    println!("previous_move: {:?}", state.current_move);
    println!("time to process {:?}", end.duration_since(start));
    println!(
        "negamax in board {:?}:{}",
        ai_move.0 .0, ALPHABET[ai_move.0 .1 as usize]
    );
    println!("negamax {:?}", ai_move);
    Ok(ai_move)
}

#[pyfunction]
fn show_state(board: Vec<Vec<i32>>, player: i32, x: i32, y: i32) {
    let mut mutboard: Vec<Vec<i32>> = board;
    let state: state::State = state::create_new_state(&mut mutboard, player, (x, y));

    // let value = negamax::negamax(&mut state, 2, -1000, 1000, player);
    let _value = checking_move(state);

    // let start_time = Instant::now();
    // let end_time = Instant::now();
    // println!("time to process {:?}", end_time.duration_since(start_time));
    // println!("negamax {:?}:{}", value.0 .0, ALPHABET[value.0 .1 as usize]);
    // println!("negamax heuristic {:?}", value.1);

    Ok(((0, 0), 0))
}

#[pyfunction]
fn place_stone(board: Vec<Vec<i32>>, player: i32, x: i32, y: i32) -> PyResult<PyObject> {
    let start = Instant::now();
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);
    let mut mutboard: Vec<Vec<i32>> = board;
    let mut eated_piece = 0i32;
    let wrong_status = 0;
    if wrong_status == 1 {
        dict.set_item("game_status", -1)?;
    } else {
        dict.set_item("game_status", 0)?;
    }
    if wrong_status == 0 {
        mutboard[x as usize][y as usize] = player;
        eated_piece = 0;
    }
    if 0 >= 5 {
        dict.set_item("wining_position", (x, y))?;
    }
    dict.set_item("eated_piece", eated_piece)?;
    dict.set_item("board", &mutboard)?;
    let end = Instant::now();
    println!("time to process {:?}", end.duration_since(start));
    Ok(dict.to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
pub fn gomoku_tests(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_returning_dict_to_python, m)?)?;
    m.add_function(wrap_pyfunction!(test_updating_from_other_function, m)?)?;
    m.add_function(wrap_pyfunction!(test_get_pydict, m)?)?;
    m.add_function(wrap_pyfunction!(test_double_triple, m)?)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn gomoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(place_stone, m)?)?;
    m.add_function(wrap_pyfunction!(ai_move, m)?)?;
    m.add_wrapped(wrap_pymodule!(gomoku_tests))?;
    Ok(())
}
