//! Crate specialy used for printing solutions.

use crate::bitboards::create_vec_from_bitboards;
use crate::check_move::check_stone_color;
use crate::data_struct::Bitboards;
use crate::data_struct::State;
use crate::global_var;
use crate::heuristic_ratios;
use crate::search_space::get_search_box_bitboard;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn print_heuristic_table(state: &State) {
    let len = state.available_move.len();
    let mut table: Vec<Vec<String>> = vec![];
    let mut line: Vec<String> = vec![];
    let mut trigger = 0;
    let box_list = get_search_box_bitboard(&state.bitboards);

    line.push("".to_string());
    line.push("  A".to_string());
    line.push("  B".to_string());
    line.push("  C".to_string());
    line.push("  D".to_string());
    line.push("  E".to_string());
    line.push("  F".to_string());
    line.push("  G".to_string());
    line.push("  H".to_string());
    line.push("  I".to_string());
    line.push("  J".to_string());
    line.push("  K".to_string());
    line.push("  L".to_string());
    line.push("  M".to_string());
    line.push("  N".to_string());
    line.push("  O".to_string());
    line.push("  P".to_string());
    line.push("  Q".to_string());
    line.push("  R".to_string());
    line.push("  S".to_string());
    table.push(line);
    line = vec![];
    for x in 0..19 {
        line.push((19 - x).to_string());
        for y in 0..19 {
            for idx in 0..len {
                if (state.available_move[idx].current_move_pos) / 19 == x
                    && (state.available_move[idx].current_move_pos) % 19 == y
                {
                    if state.available_move[idx].heuristic >= heuristic_ratios::MAX_VALUE {
                        line.push("MAX".to_string());
                    } else if state.available_move[idx].heuristic <= heuristic_ratios::MIN_VALUE {
                        line.push("MIN".to_string());
                    } else {
                        // Printing Billion value move
                        if state.available_move[idx].heuristic >= 1_000_000_000
                            || state.available_move[idx].heuristic <= -1_000_000_000
                        {
                            line.push(
                                vec![
                                    "".to_string(),
                                    (state.available_move[idx].heuristic / 1_000_000_000)
                                        .to_string(),
                                    "B".to_string(),
                                ]
                                .join(""),
                            );
                        }
                        // Printing Million value move
                        else if (state.available_move[idx].heuristic >= 1_000_000
                            && state.available_move[idx].heuristic < 1_000_000_000)
                            || (state.available_move[idx].heuristic <= -1_000_000
                                && state.available_move[idx].heuristic > -1_000_000_000)
                        {
                            line.push(
                                vec![
                                    "".to_string(),
                                    (state.available_move[idx].heuristic / 1_000_000).to_string(),
                                    "M".to_string(),
                                ]
                                .join(""),
                            );
                        }
                        // Printing Kilo value move
                        else if (state.available_move[idx].heuristic >= 1_000
                            && state.available_move[idx].heuristic < 1_000_000)
                            || (state.available_move[idx].heuristic <= -1_000
                                && state.available_move[idx].heuristic > -1_000_000)
                        {
                            line.push(
                                vec![
                                    "".to_string(),
                                    (state.available_move[idx].heuristic / 1_000).to_string(),
                                    "K".to_string(),
                                ]
                                .join(""),
                            );
                        }
                        // Printing Small value move
                        else {
                            line.push(
                                vec![
                                    " ".to_string(),
                                    state.available_move[idx].heuristic.to_string(),
                                    " ".to_string(),
                                ]
                                .join(""),
                            );
                        }
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
                            line.push(" * ".to_string());
                        } else if color == global_var::PLAYER_BLACK_NB {
                            line.push(" ° ".to_string());
                        } else {
                            line.push(" / ".to_string());
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
    // println!("heuristic table:");
    // for x in 0..(table.len()) {
    //     for y in 0..(table.len()) {
    //         print!("{:5}", table[x][y]);
    //     }
    //     println!("");
    // }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn print_board_from_bitboard(bitboards: &Bitboards) {
    let board = create_vec_from_bitboards(bitboards);
    println!();
    println!("    A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S");
    let mut x = 0;
    for board_box in board {
        print!("{:?} ", x);
        if x < 10 {
            print!(" ");
        }
        x += 1;
        println!("{:?}", board_box);
    }
}

static ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[allow(dead_code)]
pub fn print_pos_in_human_format(pos: usize) {
    let x_pos = 19 - pos / 19;
    let y_pos = pos % 19;
    println!("{:?}, {:?}", x_pos, ALPHABET[y_pos]);
}
