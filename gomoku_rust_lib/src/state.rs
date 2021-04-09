use crate::bitboards::Bitboards;
use crate::check::check_alignment_for_given_pos;
use crate::check::check_is_in_table;
use crate::heuristic::heuristic;

#[path = "search_space.rs"]
mod search_space;
// use crate::state::search_space::get_box;
use crate::state::search_space::get_search_box;

pub struct State {
    pub board: Vec<Vec<i8>>,
    pub bitboards: Bitboards,
    pub available_move: Vec<State>,
    pub current_player: i8,
    pub white_captured_stone: i8,
    pub black_captured_stone: i8,
    pub heuristic: i32,
    pub win_state: i32,
    pub win_move: Vec<((isize, isize), i8)>,
    pub current_move: (isize, isize),
    pub bit_current_move_pos: i16,
}

pub fn create_new_state(
    board: &mut Vec<Vec<i8>>,
    bitboards: &mut Bitboards,
    player: i8,
    current_move: (isize, isize),
    bit_current_move_pos: i16,
    white_captured_stone: i8,
    black_captured_stone: i8,
    win_move: Vec<((isize, isize), i8)>,
) -> State {
    let mut new_state = State {
        board: board.to_vec(),
        bitboards: bitboards.clone(),
        current_player: player,
        white_captured_stone: white_captured_stone,
        black_captured_stone: black_captured_stone,
        available_move: vec![],
        heuristic: 0,
        win_move: win_move,
        win_state: 0,
        current_move: current_move,
        bit_current_move_pos: bit_current_move_pos,
    };
    if new_state.win_move.len() > 0 {
        for winmove in 0..new_state.win_move.len() {
            if new_state.win_move[winmove].1 == new_state.current_player {
                new_state.win_state = 1;
            } else {
                new_state.win_state = -1;
            }
            break;
        }
    }
    new_state.heuristic = heuristic(&mut new_state);
    return new_state;
}

pub fn create_child(state: &mut State) -> Vec<State> {
    let mut copy_board: Vec<Vec<i8>>;
    let mut copy_bitboards: Bitboards;
    let mut copy_win_pos: Vec<((isize, isize), i8)>;
    let mut childs_list: Vec<State>;
    let index_box: Vec<(usize, usize)> = get_search_box(state);
    let len = index_box.len();
    childs_list = Vec::new();
    for pos in 0..len {
        let x = index_box[pos].0;
        let y = index_box[pos].1;
        copy_board = state.board.clone();
        copy_bitboards = state.bitboards.clone();
        copy_win_pos = state.win_move.clone();
        let bit_current_move_pos: i16 = (x * 19 + y) as i16;

        let mut child = create_new_state(
            &mut copy_board,
            &mut copy_bitboards,
            -state.current_player,
            (x as isize, y as isize),
            bit_current_move_pos,
            state.white_captured_stone,
            state.black_captured_stone,
            copy_win_pos,
        );
        if child.heuristic > -1000 {
            apply_state_move(&mut child, 8);
            childs_list.push(child);
        }
    }
    return childs_list;
}

fn capture_stone(board: &mut Vec<Vec<i8>>, pos_list: Vec<(isize, isize)>) -> i8 {
    let mut count_capture: i8 = 0;
    for (x, y) in pos_list {
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
    let mut pos_list = Vec::new();
    let mut count_capture: i8 = 0;
    if check_is_in_table(x, y, xsign, ysign, 1) == 0
        && board[(x + 1 * xsign) as usize][(y + 1 * ysign) as usize] != player
        && board[(x + 1 * xsign) as usize][(y + 1 * ysign) as usize] != 0
    {
        if check_is_in_table(x, y, xsign, ysign, 2) == 0
            && board[(x + 2 * xsign) as usize][(y + 2 * ysign) as usize] != 0
        {
            if check_is_in_table(x, y, xsign, ysign, 3) == 0
                && board[(x + 3 * xsign) as usize][(y + 3 * ysign) as usize] == player
            {
                pos_list.push((x + 1 * xsign, y + 1 * ysign));
                pos_list.push((x + 2 * xsign, y + 2 * ysign));
                count_capture += capture_stone(board, pos_list);
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
    let player = state.current_player;
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
        state.current_player;
    if stone_captured > 0 {
        apply_capturing_stone(state, stone_captured);
    }
}

pub fn state_is_terminated(state: &mut State) -> bool {
    if state.current_player == 1 {
        if state.white_captured_stone >= 10 {
            return true;
        }
    } else {
        if state.black_captured_stone >= 10 {
            return true;
        }
    }
    if state.win_move.len() > 0 && state.win_state == 1 {
        if check_alignment_for_given_pos(
            &state,
            state.win_move[0].0 .0,
            state.win_move[0].0 .1,
            state.win_move[0].1,
        ) == true
        {
            return true;
        }
    }
    return false;
}
