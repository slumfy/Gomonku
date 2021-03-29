#[path = "check.rs"]
mod check;
use check::check_is_in_table;
use check::check_is_wrong_move;
use check::create_axes_from_stone_position;
use check::check_alignment_for_given_pos;
#[path = "heuristic.rs"]
mod heuristic;

pub struct State {
    pub board: Vec<Vec<i8>>,
    pub available_move: Vec<State>,
    pub player_to_play: i8,
	pub white_eat_value: i8,
	pub black_eat_value: i8,
    pub heuristic: i32,
	pub win_state:i32,
	pub win_move: Vec<((isize,isize),i8)>,
    pub current_move: (isize, isize),
}

pub fn create_new_state(
    board: &mut Vec<Vec<i8>>,
    player: i8,
    current_move: (isize, isize),
	player_eat_value: (i8,i8),
	win_move: Vec<((isize,isize),i8)>
) -> State {
    let mut new_state = State {
        board: board.to_vec(),
        player_to_play: player,
		white_eat_value: player_eat_value.0,
		black_eat_value: player_eat_value.1,
        available_move: vec![],
        heuristic: 0,
		win_move: win_move,
		win_state: 0,
        current_move: current_move,
    };
	if new_state.win_move.len() > 0 {
		for winmove in 0..new_state.win_move.len() {
			if new_state.win_move[winmove].1 == new_state.player_to_play {
				new_state.win_state == 1;
			}
			else {
				new_state.win_state == -1;
			}
			break ;
		}
	}
	new_state.heuristic = heuristic::heuristic(&mut new_state);
    return new_state;
}

fn get_box(state: &mut State) -> ((isize, isize), (isize, isize)) {
    let offset = 5;
    let mut x_tuple: (isize, isize) = (0, 18);
    let mut y_tuple: (isize, isize) = (0, 18);
    if state.current_move.0 - offset >= 0 {
        x_tuple.0 = state.current_move.0 - offset;
    }
    if state.current_move.0 + offset <= 18 {
        x_tuple.1 = state.current_move.0 + offset;
    }
    if state.current_move.1 - offset >= 0 {
        y_tuple.0 = state.current_move.1 - offset;
    }
    if state.current_move.1 + offset <= 18 {
        y_tuple.1 = state.current_move.1 + offset;
    }
	// println!("tuple {:?}  x {} y {}",(x_tuple, y_tuple),state.current_move.0,state.current_move.1);
    return (x_tuple, y_tuple);
}

pub fn create_child(state: &mut State) -> Vec<State> {
    let mut cpyboard: Vec<Vec<i8>>;
	let mut cpywinpos: Vec<((isize,isize),i8)>;
    let mut childlist: Vec<State>;
    let indexbox: ((isize, isize), (isize, isize)) = get_box(state);
    childlist = Vec::new();
    for x in indexbox.0.0..indexbox.0.1 {
        for y in indexbox.1.0..indexbox.1.1 {
			cpyboard = state.board.clone();
			cpywinpos = state.win_move.clone();
			let mut child = create_new_state(&mut cpyboard, -state.player_to_play, (x, y), (state.white_eat_value,state.black_eat_value),cpywinpos);
            let axes = create_axes_from_stone_position(&child,child.current_move.0,child.current_move.1,child.player_to_play);
            if check_is_wrong_move(&child, &axes) == 0 {
				apply_state_move(&mut child,8);
                childlist.push(child);
            }
        }
    }
    return childlist;
}

fn capture_stone(board: &mut Vec<Vec<i8>>, poslist: Vec<(isize, isize)>) -> i8 {
    let mut count_capture: i8 = 0;
    for (x, y) in poslist {
        board[x as usize][y as usize] = 0;
        count_capture += 1;
    }
    return count_capture;
}

fn capture_position_routine(
    board: &mut Vec<Vec<i8>>,
    player: i8,
    x: isize,
    y: isize,
    xsign: isize,
    ysign: isize,
) -> i8 {
    let mut poslist = Vec::new();
    let mut count_capture: i8 = 0;
    if check_is_in_table(x, y, xsign, ysign, 1) == 0
        && board[(x + 1 * xsign) as usize][(y + 1 * ysign) as usize] != player
        && board[(x + 1 * xsign) as usize][(y + 1 * ysign) as usize] != 0
    {
        if check_is_in_table(x, y, xsign, ysign, 2) == 0
            && board[(x + 2 * xsign) as usize][(y + 2 * ysign) as usize] == player
        {
            poslist.push((x + 1 * xsign, y + 1 * ysign));
            count_capture += capture_stone(board, poslist);
        } else if check_is_in_table(x, y, xsign, ysign, 2) == 0
            && board[(x + 2 * xsign) as usize][(y + 2 * ysign) as usize] != 0
        {
            if check_is_in_table(x, y, xsign, ysign, 3) == 0
                && board[(x + 3 * xsign) as usize][(y + 3 * ysign) as usize] == player
            {
                poslist.push((x + 1 * xsign, y + 1 * ysign));
                poslist.push((x + 2 * xsign, y + 2 * ysign));
                count_capture += capture_stone(board, poslist);
            }
        }
    }
    return count_capture;
}

pub fn check_capturing_position(
    board: &mut Vec<Vec<i8>>,
    player: i8,
    x: isize,
    y: isize,
    stone_captured: i8,
) {
    let mut count_capture = 0;
    count_capture += capture_position_routine(board, player, x, y, 0, 1);
    if count_capture == stone_captured {
        return;
    }
    count_capture += capture_position_routine(board, player, x, y, 0, -1);
    if count_capture == stone_captured {
        return;
    }
    count_capture += capture_position_routine(board, player, x, y, 1, 0);
    if count_capture == stone_captured {
        return;
    }
    count_capture += capture_position_routine(board, player, x, y, -1, 0);
    if count_capture == stone_captured {
        return;
    }
    count_capture += capture_position_routine(board, player, x, y, 1, 1);
    if count_capture == stone_captured {
        return;
    }
    count_capture += capture_position_routine(board, player, x, y, 1, -1);
    if count_capture == stone_captured {
        return;
    }
    count_capture += capture_position_routine(board, player, x, y, -1, 1);
    if count_capture == stone_captured {
        return;
    }
    count_capture += capture_position_routine(board, player, x, y, -1, -1);
    if count_capture == stone_captured {
        return;
    }
}

fn apply_capturing_stone(state: &mut State, stone_captured: i8) {
    let player = state.player_to_play;
    let board = &mut state.board;
    check_capturing_position(
        board,
        player,
        state.current_move.0,
        state.current_move.1,
        stone_captured,
    )
}

pub fn apply_state_move(state: &mut State, stone_captured: i8) {
    state.board[state.current_move.0 as usize][state.current_move.1 as usize] =
        state.player_to_play;
    if stone_captured > 0 {
        apply_capturing_stone(state, stone_captured);
    }
}

pub fn state_is_terminated(state: &mut State) -> bool {
	if state.player_to_play == 1 {
		if state.white_eat_value >= 10 {return true;}
	}
	else {
		if state.black_eat_value >= 10 {return true;}
	}
	if state.win_move.len() > 0  && state.win_state == 1 {
		if check_alignment_for_given_pos(&state,state.win_move[0].0.0,state.win_move[0].0.1,state.win_move[0].1) == true {return true;}
	}
	return false;
}