use crate::global_var;
use crate::state::create_child;
use crate::state::state_is_terminated;
use crate::state::State;
use std::cmp::Reverse;


pub fn negamax(mut state: &mut State, depth: i32, mut alpha: i32, beta: i32, color: i8) -> i32 {
	update_max_depth(depth);
    if depth != 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    // println!("current state: {:?} player to play {} current heuristic {} depth {}", state.bit_current_move_pos, state.current_player, state.heuristic, depth);
    if depth == 0 || state_is_terminated(state) == true {
        return state.heuristic * color as i32;
    }
    let mut value: i32 = global_var::HEURISTIC_MIN_VALUE;
    let len = state.available_move.len();
    for child in 0..len {
        let negamax = -negamax(
            &mut state.available_move[child],
            depth - 1,
            -beta,
            -alpha,
            -color,
        );
        value = std::cmp::max(value, negamax);
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            // println!("pruning");
            break;
        }
    }
    // // println!("alpha {}  beta {}", alpha, beta);
    state.heuristic = value;
    return value;
}

fn update_max_depth(depth: i32) {
	unsafe {
		if global_var::DEPTH - depth > global_var::MAX_DEPTH_REACH {
			global_var::MAX_DEPTH_REACH = global_var::DEPTH - depth;
		}
	}
}

pub fn return_move(state: &mut State, heuristic: i32) -> (usize, i32) {
    print_heuristic_table(state);
	unsafe{println!("MAX DEPTH: {}", global_var::MAX_DEPTH_REACH);}
    // println!("heuristic of returned move : {:?}", heuristic);
    state.available_move.sort_by_key(|d| Reverse(d.heuristic));
	// for x in 0..state.available_move.len() {
	// 	println!("child nb {}, child heuristic {}", x,state.available_move[x].heuristic);
	// }
    // println!(
    //     "ret move x = {} y = {}, ret heuristic {}",
    //     (state.available_move[0].bit_current_move_pos) / 19,
    //     (state.available_move[0].bit_current_move_pos) % 19,
    //     state.available_move[0].heuristic
    // );
    return (
        (state.available_move[0].bit_current_move_pos),
        state.available_move[0].heuristic,
    );
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
