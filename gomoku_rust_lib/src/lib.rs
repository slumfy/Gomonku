//! Lib for rust, bridge between python and rust, using pyo3 module.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use pyo3::{wrap_pyfunction, wrap_pymodule};
use std::time::Instant;

mod algorithms;
mod data_struct;
mod bitboard_operations;
mod bitboards;
mod bitpattern;
mod check_move;
mod global_var;
mod heuristic;
mod heuristic_ratios;
mod print;
mod pytests;
mod search_space;
mod state;
mod test_check_free_development;
mod test_check_is_unblockable_five;
mod test_is_on_axe;
mod test_pattern_axes_finder;
mod utils;

use crate::check_move::get_move_info;
use check_move::__pyo3_get_function_check_move_is_still_winning;
use check_move::check_pos_still_win;
use check_move::checking_and_apply_bits_move;
use crate::data_struct::BoardStateInfo;

use crate::pytests::__pyo3_get_function_pytest_check_free_development;
use crate::pytests::__pyo3_get_function_pytest_check_is_unblockable_five;
use crate::pytests::__pyo3_get_function_pytest_get_pydict;
use crate::pytests::__pyo3_get_function_pytest_is_on_axe;
use crate::pytests::__pyo3_get_function_pytest_pattern_axes_finder;
use crate::pytests::__pyo3_get_function_pytest_returning_dict_to_python;
use crate::pytests::__pyo3_get_function_pytest_updating_from_other_function;

static ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[pyfunction]
fn ai_move(
    board: Vec<Vec<i8>>,
    player: i8,
    x: usize,
    y: usize,
    turn: isize,
    wining_position: (usize, i8),
	nb_move_to_win: i8,
    display_ai_time: bool,
    search_algorithm: String,
) -> PyResult<((usize, usize), i32)> {
    println!("AIplayer {:?} x {:?} y {:?}", player, x, y);
    let white_captured_stone: i8;
    let black_captured_stone: i8;

    unsafe {
        white_captured_stone = global_var::WHITE_CAPTURED_STONE;
        black_captured_stone = global_var::BLACK_CAPTURED_STONE;
    }
    let mut bitboards = bitboards::create_bitboards_from_vec(&board);
    let bit_current_move_pos: usize = x * 19 + y;
    let ai_move: (usize, i32);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        -player,
        bit_current_move_pos,
        white_captured_stone,
        black_captured_stone,
        wining_position,
		nb_move_to_win,
    );
	let state_info = get_move_info(&mut state);
    println!("Black move_to_win {}", state.black_move_to_win);
    println!("White move_to_win {}", state.white_move_to_win);
    let start_time = Instant::now();

    if turn == 0 {
        ai_move = (180, 0);
    } else {
        unsafe {
            global_var::MAX_DEPTH_REACH = 0;
        }
        if search_algorithm == "negamax" {
            println!("using negamax");
            algorithms::negamax(
                &mut state,
                global_var::DEPTH,
                heuristic_ratios::HEURISTIC_MIN_VALUE,
                heuristic_ratios::HEURISTIC_MAX_VALUE,
                player,
            );
        } else if search_algorithm == "negamax_with_transpotable" {
            println!("using negamax_with_transpotable");
            algorithms::negamax_with_transpotable(
                &mut state,
                global_var::DEPTH,
                heuristic_ratios::HEURISTIC_MIN_VALUE,
                heuristic_ratios::HEURISTIC_MAX_VALUE,
                player,
            );
        } else if search_algorithm == "negascout" {
            println!("using negascout");
            algorithms::negascout(
                &mut state,
                global_var::DEPTH,
                heuristic_ratios::HEURISTIC_MIN_VALUE,
                heuristic_ratios::HEURISTIC_MAX_VALUE,
                player,
            );
        } else if search_algorithm == "negascout_with_transpotable" {
            println!("using negascout_with_transpotable");
            algorithms::negascout_with_transpotable(
                &mut state,
                global_var::DEPTH,
                heuristic_ratios::HEURISTIC_MIN_VALUE,
                heuristic_ratios::HEURISTIC_MAX_VALUE,
                player,
            );
        }

        ai_move = algorithms::return_move(&mut state);
    }
    if display_ai_time {
        let end_time = Instant::now();
        println!("time to process {:?}", end_time.duration_since(start_time));
    }

    let ai_x_move = (ai_move.0 / 19) as usize;
    let ai_y_move = (ai_move.0 % 19) as usize;

    println!(
        "previous_move: {:?} heuristic {}",
        state.bit_current_move_pos, state.heuristic
    );
    println!(
        "white eat: {:?} black eat: {:?}",
        white_captured_stone, black_captured_stone
    );
    println!(
        "negamax in board {:?}:{} turn {}",
        ai_x_move, ALPHABET[ai_y_move as usize], turn
    );
    println!("negamax {:?}", ai_move);
    Ok(((ai_x_move, ai_y_move), ai_move.1))
}

#[pyfunction]
fn place_stone(mut board: Vec<Vec<i8>>, player: i8, x: usize, y: usize) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);

    println!("place stone for player {:?} at x {:?} y {:?}", player, x, y);

    let white_captured_stone: i8;
    let black_captured_stone: i8;
    unsafe {
        white_captured_stone = global_var::WHITE_CAPTURED_STONE;
        black_captured_stone = global_var::BLACK_CAPTURED_STONE;
    }
    let bit_current_move_pos: usize = x * 19 + y;

    let mut bitboards = bitboards::create_bitboards_from_vec(&board); // BITBOARDS CREATION
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        player,
        bit_current_move_pos,
        white_captured_stone,
        black_captured_stone,
        (0, 0),
		5
    );

    let board_state_info: BoardStateInfo = checking_and_apply_bits_move(&mut state);
    if board_state_info.is_wrong_move == global_var::VALID_MOVE {
        dict.set_item("game_status", 0)?;
		dict.set_item("nb_move_to_win", board_state_info.nb_move_to_win)?;
        dict.set_item("stone_captured", board_state_info.stone_captured)?;
        if player == global_var::PLAYER_WHITE_NB {
            unsafe {
                global_var::WHITE_CAPTURED_STONE += board_state_info.stone_captured;
            }
        } else {
            unsafe {
                global_var::BLACK_CAPTURED_STONE += board_state_info.stone_captured;
            }
        }
        if board_state_info.is_winning.1 != 0 {
            dict.set_item("wining_position", board_state_info.is_winning)?;
        }
        println!("winstate =>> {:?}", board_state_info.is_winning);
    } else {
        println!("Wrong move status = {:?}", board_state_info.is_wrong_move);
        dict.set_item("game_status", board_state_info.is_wrong_move)?;
    }
    board = bitboards::create_vec_from_bitboards(&state.bitboards);
    dict.set_item("board", board)?;
    Ok(dict.to_object(py))
}

#[pyfunction]
fn get_rust_box(board: Vec<Vec<i8>>) -> PyResult<Vec<(usize, usize)>> {
    let mutboard: Vec<Vec<i8>> = board;
    let bitboards = bitboards::create_bitboards_from_vec(&mutboard);
    let search_bitbox = search_space::get_search_box_bitboard(&bitboards);
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
    m.add_function(wrap_pyfunction!(pytest_returning_dict_to_python, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_updating_from_other_function, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_get_pydict, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_check_is_unblockable_five, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_check_free_development, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_is_on_axe, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_pattern_axes_finder, m)?)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn gomoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(place_stone, m)?)?;
    m.add_function(wrap_pyfunction!(get_rust_box, m)?)?;
    m.add_function(wrap_pyfunction!(ai_move, m)?)?;
    m.add_function(wrap_pyfunction!(check_move_is_still_winning, m)?)?;
    m.add_function(wrap_pyfunction!(reset_game, m)?)?;
    m.add_wrapped(wrap_pymodule!(gomoku_tests))?;
    Ok(())
}
