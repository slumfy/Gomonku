use crate::bitboards::Bitboards;
use crate::heuristic::heuristic;

use crate::search_space::get_search_box_bitboard;

pub struct State {
    pub bitboards: Bitboards,
    pub available_move: Vec<State>,
    pub current_player: i8,
    pub white_captured_stone: i8,
    pub black_captured_stone: i8,
    pub heuristic: i32,
    pub win_state: (usize, i8),
    pub bit_current_move_pos: usize,
}

pub fn create_new_state(
    bitboards: &mut Bitboards,
    player: i8,
    bit_current_move_pos: usize,
    white_captured_stone: i8,
    black_captured_stone: i8,
) -> State {
    let new_state = State {
        bitboards: bitboards.clone(),
        current_player: player,
        white_captured_stone: white_captured_stone,
        black_captured_stone: black_captured_stone,
        available_move: vec![],
        heuristic: 0,
        win_state: (0, 0),
        bit_current_move_pos: bit_current_move_pos,
    };
    return new_state;
}

pub fn create_child(state: &mut State) -> Vec<State> {
    let mut copy_bitboards: Bitboards;
    let mut childs_list: Vec<State>;
    let index_box: Vec<usize> = get_search_box_bitboard(&state.bitboards);
    let len = index_box.len();
    childs_list = Vec::new();
    for pos in 0..len {
        copy_bitboards = state.bitboards.clone();
        let bit_current_move_pos: usize = index_box[pos];

        let mut child = create_new_state(
            &mut copy_bitboards,
            -state.current_player,
            bit_current_move_pos,
            state.white_captured_stone,
            state.black_captured_stone,
        );
        child.heuristic = heuristic(&mut child);
        if child.heuristic > 0 {
            println!("childheur {}", child.heuristic);
        }
        if child.heuristic >= 0 {
            childs_list.push(child);
        }
    }
    return childs_list;
}

pub fn state_is_terminated(state: &mut State) -> bool {
    if state.white_captured_stone >= 10 || state.black_captured_stone >= 10 {
        return true;
    }
    if state.heuristic == 100000 {
        return true;
    }
    return false;
}
