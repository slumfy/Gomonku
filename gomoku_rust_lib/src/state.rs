#[path = "check.rs"]
mod check;
use check::create_axes_from_stone_position;
#[path = "heuristic.rs"]
mod heuristic;

pub struct State {
    pub board: Vec<Vec<i8>>,
    pub available_move: Vec<State>,
    pub player_to_play: i8,
    pub heuristic: i32,
    pub current_move: (isize, isize),
}

pub fn create_new_state(
    board: &mut Vec<Vec<i8>>,
    player: i8,
    current_move: (isize, isize),
) -> State {
    let new_state = State {
        board: board.to_vec(),
        player_to_play: player,
        available_move: vec![],
        heuristic: 0,
        current_move: current_move,
    };

    return new_state;
}

fn check_is_in_table(x: isize, y: isize, xsign: isize, ysign: isize, offset: isize) -> i8 {
    if x + offset * xsign > 18
        || x + offset * xsign < 0
        || y + offset * ysign > 18
        || y + offset * ysign < 0
    {
        return 1;
    }
    return 0;
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
