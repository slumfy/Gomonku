//! Crate specialy used for printing solutions.

use crate::bitboards::Bitboards;
use crate::state::State;

pub fn print_axes(axes: &[[u16; 4]; 2]) {
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

pub fn print_heuristic_table(state: &State) {
    let len = state.available_move.len();
    let mut table: Vec<Vec<i32>> = vec![];
    let mut line: Vec<i32> = vec![];
    let mut xmax = 0;
    let mut xmin = 18;
    let mut ymax = 0;
    let mut ymin = 18;
    let mut trigger = 0;
    for idx in 0..len {
        // println!("x {} y {}", state.available_move[idx].current_move.0, state.available_move[idx].current_move.1);
        xmax = std::cmp::max((state.available_move[idx].bit_current_move_pos) / 19, xmax);
        xmin = std::cmp::min(xmin, (state.available_move[idx].bit_current_move_pos) / 19);
        ymax = std::cmp::max(ymax, (state.available_move[idx].bit_current_move_pos) % 19);
        ymin = std::cmp::min(ymin, (state.available_move[idx].bit_current_move_pos) % 19);
    }
    println!(
        "xmax: {}, xmin: {}, ymax: {},ymin: {}",
        xmax, xmin, ymax, ymin
    );
    for x in 0..19 {
        for y in 0..19 {
            for idx in 0..len {
                if (state.available_move[idx].bit_current_move_pos) / 19 == x
                    && (state.available_move[idx].bit_current_move_pos) % 19 == y
                {
                    line.push(state.available_move[idx].heuristic);
                    trigger = 1;
                }
            }
            if trigger == 0 {
                line.push(1111);
            } else {
                trigger = 0;
            }
        }
        table.push(line);
        line = vec![];
    }
    println!("heuristic table:");
    for x in 0..table.len() {
        println!("{:5?}", table[x]);
    }
}

fn print_axe_value(axe: usize) {
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

pub fn print_bitboards(bitboards: &Bitboards) {
    for x in 0..6 {
        println!("white_board[{}]: {:064b}", x, bitboards.white_board[x]);
        println!("black_board[{}]: {:064b}", x, bitboards.black_board[x]);
    }
    println!();
}
