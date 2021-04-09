use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use pyo3::{wrap_pyfunction, wrap_pymodule};
use std::time::Instant;

mod bitboards;
mod bitpattern;
mod check;
mod check_bits;
mod global_var;
mod heuristic;
mod negamax;
mod search_space;
mod state;
mod tests;
mod utils;
use bitboards::print_bitboards;
use check::checking_move_biggest_alignment_and_stone_captured;
use check_bits::checking_and_apply_bits_move;

use crate::tests::__pyo3_get_function_test_double_triple;
use crate::tests::__pyo3_get_function_test_get_pydict;
use crate::tests::__pyo3_get_function_test_returning_dict_to_python;
use crate::tests::__pyo3_get_function_test_updating_from_other_function;
use state::apply_state_move;

static ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[pyfunction]
fn ai_move(
    board: Vec<Vec<i8>>,
    player: i8,
    x: isize,
    y: isize,
    turn: isize,
    wining_position: Vec<((isize, isize), i8)>,
) -> PyResult<((isize, isize), i32)> {
    println!("player {:?} x {:?} y {:?}", player, x, y);
    let white_captured_stone: i8;
    let black_captured_stone: i8;
    unsafe {
        white_captured_stone = global_var::WHITE_CAPTURED_STONE;
        black_captured_stone = global_var::BLACK_CAPTURED_STONE;
    }
    let mut mutboard: Vec<Vec<i8>> = board;
    let mut bitboards = bitboards::create_bitboards_from_vec(&mutboard);
    let bit_current_move_pos: i16 = (x * 19 + y) as i16;
    let ai_move: ((isize, isize), i32);
    let mut state: state::State = state::create_new_state(
        &mut mutboard,
        &mut bitboards,
        player,
        (x, y),
        bit_current_move_pos,
        white_captured_stone,
        black_captured_stone,
        wining_position,
    );
    let start = Instant::now();
    if turn == 0 {
        ai_move = ((9, 9), 0);
    } else {
        let value = negamax::negamax(
            &mut state,
            global_var::DEEP,
            global_var::HEURISTIC_MIN_VALUE,
            global_var::HEURISTIC_MAX_VALUE,
            player,
        );
        ai_move = negamax::return_move(&mut state, value);
    }
    let end = Instant::now();
    println!(
        "previous_move: {:?} heuristic {}",
        state.current_move, state.heuristic
    );
    println!("time to process {:?}", end.duration_since(start));
    println!(
        "white eat: {:?} black eat: {:?}",
        white_captured_stone, black_captured_stone
    );
    println!(
        "negamax in board {:?}:{} turn {}",
        ai_move.0 .0, ALPHABET[ai_move.0 .1 as usize], turn
    );
    println!("negamax {:?}", ai_move);
    Ok(ai_move)
}

// TODO : see if function still usefull and reimplement it with bitboard
// #[pyfunction]
// fn check_move_is_a_fiverow(
//     board: Vec<Vec<i8>>,
//     player: i8,
//     x: isize,
//     y: isize,
//     wining_position: Vec<((isize, isize), i8)>,
// ) -> PyResult<bool> {
//     let mut mutboard: Vec<Vec<i8>> = board;
//     let mut bitboards = bitboards::create_bitboards_from_vec(&mutboard);
//     let white_captured_stone: i8;
//     let black_captured_stone: i8;
//     unsafe {
//         white_captured_stone = global_var::WHITE_CAPTURED_STONE;
//         black_captured_stone = global_var::BLACK_CAPTURED_STONE;
//     }
//     let bit_current_move_pos: i16 = (x * 19 + y) as i16;
//     let state: state::State = state::create_new_state(
//         &mut mutboard,
//         &mut bitboards,
//         player,
//         (x, y),
//         bit_current_move_pos,
//         white_captured_stone,
//         black_captured_stone,
//         wining_position,
//     );
//     let alignement = checking_move_biggest_alignment_and_stone_captured(&state);
//     if alignement["biggest_alignment"] >= 5 {
//         Ok(true)
//     } else {
//         Ok(false)
//     }
// }

#[pyfunction]
fn check_move_is_a_fiverow() -> PyResult<bool> {
    Ok(false)
}

#[pyfunction]
fn place_stone(
    board: Vec<Vec<i8>>,
    player: i8,
    x: isize,
    y: isize,
    wining_position: Vec<((isize, isize), i8)>,
) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);

    println!("place stone for player {:?} at x {:?} y {:?}", player, x, y);

    let mut mutboard: Vec<Vec<i8>> = board;

    let white_captured_stone: i8;
    let black_captured_stone: i8;
    unsafe {
        white_captured_stone = global_var::WHITE_CAPTURED_STONE;
        black_captured_stone = global_var::BLACK_CAPTURED_STONE;
    }
    let bit_current_move_pos: i16 = (x * 19 + y) as i16;

    let mut bitboards = bitboards::create_bitboards_from_vec(&mutboard); // BITBOARDS CREATION
    let mut state: state::State = state::create_new_state(
        &mut mutboard,
        &mut bitboards,
        player,
        (x, y),
        bit_current_move_pos,
        white_captured_stone,
        black_captured_stone,
        wining_position,
    );

    let board_check: HashMap<String, i8> = checking_and_apply_bits_move(&mut state);
    if board_check["is_wrong_move"] == global_var::VALID_MOVE {
        apply_state_move(&mut state, board_check["stone_captured"]);
        dict.set_item("game_status", 0)?;
        dict.set_item("stone_captured", board_check["stone_captured"])?;
        if player == global_var::PLAYER_WHITE_NB {
            unsafe {
                global_var::WHITE_CAPTURED_STONE += board_check["stone_captured"];
            }
        } else {
            unsafe {
                global_var::BLACK_CAPTURED_STONE += board_check["stone_captured"];
            }
        }
        if board_check["biggest_alignment"] >= 5 {
            dict.set_item("wining_position", &state.current_move)?;
        }
    } else {
        println!("Wrong move status = {:?}", board_check["is_wrong_move"]);
        dict.set_item("game_status", board_check["is_wrong_move"])?;
    }
    let vecboard = bitboards::create_vec_from_bitboards(&state.bitboards);
    dict.set_item("board", vecboard)?;
    Ok(dict.to_object(py))
}

#[pyfunction]
fn get_rust_box(board: Vec<Vec<i8>>) -> PyResult<Vec<(usize, usize)>> {
    let mutboard: Vec<Vec<i8>> = board;
    let bitboards = bitboards::create_bitboards_from_vec(&mutboard);
    let search_bitbox = search_space::get_search_box_bitboard(bitboards);
    // println!("bitbox: {:?}", search_bitbox);
    let search_box = search_space::unwrap_bitlist(search_bitbox);
    // println!("searchbox: {:?}", search_box);
    Ok(search_box)
}

#[pyfunction]
fn reset_game() {
    unsafe {
        global_var::WHITE_CAPTURED_STONE = 0;
        global_var::BLACK_CAPTURED_STONE = 0;
    }
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
    m.add_function(wrap_pyfunction!(get_rust_box, m)?)?;
    m.add_function(wrap_pyfunction!(ai_move, m)?)?;
    m.add_function(wrap_pyfunction!(check_move_is_a_fiverow, m)?)?;
    m.add_function(wrap_pyfunction!(reset_game, m)?)?;
    m.add_wrapped(wrap_pymodule!(gomoku_tests))?;
    Ok(())
}
