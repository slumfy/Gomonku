//! Lib for rust, bridge between python and rust, using pyo3 module.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use pyo3::{wrap_pyfunction, wrap_pymodule};
use std::time::Instant;

mod algorithms;
mod bitboard_operations;
mod bitboards;
mod bitpattern;
mod check_move;
mod data_struct;
mod global_var;
mod heuristic;
mod heuristic_ratios;
mod print;
mod search_space;
mod state;
mod tests;
mod utils;

#[macro_use]
extern crate lazy_static;

use crate::check_move::get_move_info;
use crate::data_struct::BoardStateInfo;
use bitboards::create_bits_axes_from_pos;
use check_move::__pyo3_get_function_check_move_is_still_winning;
use check_move::check_pos_still_win;
use check_move::checking_and_apply_bits_move;

use tests::__pyo3_get_function_pytest_ai_move;
use tests::__pyo3_get_function_pytest_algorithm_benchmark;
use tests::__pyo3_get_function_pytest_check_is_capturable;
use tests::__pyo3_get_function_pytest_check_pattern_is_not_capturable_five;
use tests::__pyo3_get_function_pytest_check_potential_winning_alignment;
use tests::__pyo3_get_function_pytest_get_pydict;
use tests::__pyo3_get_function_pytest_is_on_axe;
use tests::__pyo3_get_function_pytest_pattern_axes_finder;
use tests::__pyo3_get_function_pytest_print_pos_in_human_format;
use tests::__pyo3_get_function_pytest_returning_dict_to_python;
use tests::__pyo3_get_function_pytest_test_blockers;
use tests::__pyo3_get_function_pytest_updating_from_other_function;
//END TEST BLOCKER

#[pyfunction]
pub fn ai_move(
    board: Vec<Vec<i8>>,
    player: i8,
    x: usize,
    y: usize,
    turn: isize,
    wining_position: (usize, i8),
    display_ai_time: bool,
    search_algorithm: String,
    depth: i32,
) -> PyResult<((usize, usize), u128)> {
    // println!("AIplayer {:?} x {:?} y {:?}", player, x, y);
    let total_white_captured_stone: i8;
    let total_black_captured_stone: i8;

    unsafe {
        total_white_captured_stone = global_var::TOTAL_WHITE_CAPTURED_STONE;
        total_black_captured_stone = global_var::TOTAL_BLACK_CAPTURED_STONE;
        global_var::DEPTH = depth;
    }
    let mut bitboards = bitboards::create_bitboards_from_vec(&board);
    let current_move_pos: usize = x * 19 + y;
    let ai_move: (usize, i64);
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        -player,
		-1,
        current_move_pos,
        total_white_captured_stone,
        total_black_captured_stone,
        0,
        0,
        wining_position,
		0
    );
    get_move_info(&mut state);
    let mut time: u128 = 0;
    let start_time = Instant::now();
    if turn == 0 {
        ai_move = (180, 0);
    } else {
        if search_algorithm == "negamax" {
            println!("using negamax");
            state.heuristic = 0;
            // For alpha, sending  min value + 1 to prevent overflow when changing sign.
            algorithms::negamax(
				&mut state,
				depth,
				heuristic_ratios::MIN_VALUE,
                heuristic_ratios::MAX_VALUE);
        } else if search_algorithm == "negascout" {
            println!("using negascout");
            algorithms::negascout(
                &mut state,
                depth,
                heuristic_ratios::MIN_VALUE,
                heuristic_ratios::MAX_VALUE,
                player,
            );
        } else if search_algorithm == "minimax" {
            println!("using minimax");
            algorithms::minimax(
                &mut state,
                depth,
                heuristic_ratios::MIN_VALUE,
                heuristic_ratios::MAX_VALUE,
                true,
            );
        }
        ai_move = algorithms::return_move(&mut state);
    }
    if display_ai_time {
        let end_time = Instant::now();
        time = end_time.duration_since(start_time).as_millis();
        println!("time to process {:?}", time);
    }
    let ai_x_move = (ai_move.0 / 19) as usize;
    let ai_y_move = (ai_move.0 % 19) as usize;
    Ok(((ai_x_move, ai_y_move), time))
}

fn player_win(state: &mut data_struct::State, _opponent: i8) -> bool {
    // Run negamax with depth 1 to see if only min_value is returned.
    // If it's the case, it means that the player have win.
    unsafe {
        global_var::DEPTH = 1;
    }
    let ai_move: (usize, i64);
    algorithms::negamax(state, 1,heuristic_ratios::MIN_VALUE, heuristic_ratios::MAX_VALUE);
    ai_move = algorithms::return_move(state);
    if ai_move.1 == heuristic_ratios::MIN_VALUE {
        return true;
    } else {
        return false;
    }
}

#[pyfunction]
fn place_stone(mut board: Vec<Vec<i8>>, player: i8, x: usize, y: usize) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);

    let total_white_captured_stone: i8;
    let total_black_captured_stone: i8;
    unsafe {
        total_white_captured_stone = global_var::TOTAL_WHITE_CAPTURED_STONE;
        total_black_captured_stone = global_var::TOTAL_BLACK_CAPTURED_STONE;
        println!(
            "whitcap {} blackcap {}",
            total_white_captured_stone, total_black_captured_stone
        );
    }
    let current_move_pos: usize = x * 19 + y;

    let mut bitboards = bitboards::create_bitboards_from_vec(&board); // BITBOARDS CREATION
    let mut state: data_struct::State = state::create_new_state(
        &mut bitboards,
        player,
		-1,
        current_move_pos,
        total_white_captured_stone,
        total_black_captured_stone,
        0,
        0,
        (0, 0),
		0
    );
    state.axes = create_bits_axes_from_pos(current_move_pos, &mut bitboards);

    let board_state_info: BoardStateInfo = checking_and_apply_bits_move(&mut state);
    println!(
        "boardstate of returning move {} : {:?}",
        state.current_move_pos, board_state_info
    );
    if board_state_info.is_wrong_move == global_var::VALID_MOVE {
        dict.set_item("game_status", 0)?;
        dict.set_item("stone_captured", board_state_info.stone_captured)?;
        if player == global_var::PLAYER_WHITE_NB {
            unsafe {
                global_var::TOTAL_WHITE_CAPTURED_STONE += board_state_info.stone_captured;
            }
        } else {
            unsafe {
                global_var::TOTAL_BLACK_CAPTURED_STONE += board_state_info.stone_captured;
            }
        }
        if board_state_info.is_winning.1 != 0 {
            dict.set_item("wining_position", board_state_info.is_winning)?;
            state.win_state = board_state_info.is_winning;
            // Run algorithm again for opponent to see if player have win.
            if player_win(&mut state, -player) {
                dict.set_item("player_win", true)?;
            } else {
                dict.set_item("player_win", false)?;
            }
        }
    } else {
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
    let search_box = search_space::unwrap_bitlist(search_bitbox);
    Ok(search_box)
}

#[pyfunction]
fn reset_game() {
    unsafe {
        global_var::TOTAL_WHITE_CAPTURED_STONE = 0;
        global_var::TOTAL_BLACK_CAPTURED_STONE = 0;
        algorithms::reset_tt_table();
    }
}
#[pyfunction]
fn update_capture_for_player(player: i8, eated_stones: i8) {
    unsafe {
        if player == 1 {
            global_var::TOTAL_WHITE_CAPTURED_STONE = eated_stones;
        } else {
            global_var::TOTAL_BLACK_CAPTURED_STONE = eated_stones;
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
pub fn gomoku_tests(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pytest_ai_move, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_check_is_capturable, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_returning_dict_to_python, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_updating_from_other_function, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_get_pydict, m)?)?;
    m.add_function(wrap_pyfunction!(
        pytest_check_pattern_is_not_capturable_five,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(pytest_is_on_axe, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_pattern_axes_finder, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_algorithm_benchmark, m)?)?;
    m.add_function(wrap_pyfunction!(pytest_print_pos_in_human_format, m)?)?;
    m.add_function(wrap_pyfunction!(
        pytest_check_potential_winning_alignment,
        m
    )?)?;
    //TEST BLOCKER
    m.add_function(wrap_pyfunction!(pytest_test_blockers, m)?)?;
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
    m.add_function(wrap_pyfunction!(update_capture_for_player, m)?)?;
    m.add_wrapped(wrap_pymodule!(gomoku_tests))?;
    Ok(())
}
