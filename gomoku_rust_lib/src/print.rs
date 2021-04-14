//! Crate specialy used for printing solutions.

use crate::bitboards::create_vec_from_bitboards;
use crate::bitboards::Bitboards;
use crate::check_move::check_stone_color;
use crate::global_var;
use crate::search_space::get_search_box_bitboard;
use crate::state::State;

pub fn print_axes(axes: &[[u16; 4]; 2], player_color: i8) {
    if player_color != global_var::PLAYER_BLACK_NB {
        println!("white axes : ");
        println!();
        println!("|----------------|");
        for axe in &axes[0] {
            print!("|");
            print!("{:016b}", axe);
            print!("|");
            println!();
        }
        println!("|----------------|");
        println!();
    }

    if player_color != global_var::PLAYER_WHITE_NB {
        println!("black axes : ");
        println!();
        println!("|----------------|");
        for axe in &axes[1] {
            print!("|");
            print!("{:016b}", axe);
            print!("|");
            println!();
        }
        println!("|----------------|");
    }
}

pub fn print_heuristic_table(state: &State) {
    let len = state.available_move.len();
    let mut table: Vec<Vec<String>> = vec![];
    let mut line: Vec<String> = vec![];
    let mut trigger = 0;
    let box_list = get_search_box_bitboard(&state.bitboards);

    for x in 0..19 {
        for y in 0..19 {
            for idx in 0..len {
                if (state.available_move[idx].bit_current_move_pos) / 19 == x
                    && (state.available_move[idx].bit_current_move_pos) % 19 == y
                {
                    if state.available_move[idx].heuristic == global_var::HEURISTIC_MAX_VALUE {
                        line.push("MAX".to_string());
                    } else if state.available_move[idx].heuristic == global_var::HEURISTIC_MIN_VALUE
                    {
                        line.push("MIN".to_string());
                    } else {
                        line.push(state.available_move[idx].heuristic.to_string());
                    }
                    trigger = 1;
                }
            }
            if trigger == 0 {
                for b in 0..box_list.len() {
                    let pos = x * 19 + y;
                    if pos == box_list[b] {
                        let color = check_stone_color(pos, &state.bitboards);
                        if color == global_var::PLAYER_WHITE_NB {
                            line.push("*".to_string());
                        } else if color == global_var::PLAYER_BLACK_NB {
                            line.push("°".to_string());
                        } else {
                            line.push("/".to_string());
                        }
                        trigger = 1;
                    }
                }
            }
            if trigger == 0 {
                line.push("----".to_string());
            } else {
                trigger = 0;
            }
        }
        table.push(line);
        line = vec![];
    }
    println!("heuristic table:");
    for x in 0..table.len() {
        for y in 0..table.len() {
            print!("{:5}", table[x][y]);
        }
        println!("");
    }
}

pub fn print_axe_value(axe: usize) {
    if axe == 0 {
        println!("DIAGONALE UPLEFT:")
    } else if axe == 1 {
        println!("COLONE:")
    } else if axe == 2 {
        println!("DIAGONALE UPRIGHT:")
    } else {
        println!("LIGNE:")
    }
}

pub fn print_bitboards(bitboards: &Bitboards, player_color: i8) {
    println!();
    if player_color != global_var::PLAYER_BLACK_NB {
        for x in 0..6 {
            println!("white_board[{}]: {:064b}", x, bitboards.white_board[x]);
        }
        println!();
    }

    if player_color != global_var::PLAYER_BLACK_NB {
        for x in 0..6 {
            println!("black_board[{}]: {:064b}", x, bitboards.black_board[x]);
        }
        println!();
    }
}

pub fn print_board_from_bitboard(bitboards: &Bitboards) {
    let board = create_vec_from_bitboards(bitboards);
    println!();
    for board_box in board {
        println!("{:?}", board_box);
    }
}
