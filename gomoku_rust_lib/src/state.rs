use crate::bitboards::Bitboards;
use crate::heuristic::heuristic;

#[path = "search_space.rs"]
use crate::search_space::get_search_box_bitboard;

pub struct State {
    pub bitboards: Bitboards,
    pub available_move: Vec<State>,
    pub current_player: i8,
    pub white_captured_stone: i8,
    pub black_captured_stone: i8,
    pub heuristic: i32,
    pub win_state: i32,
    pub win_move: Vec<(usize, i8)>,
    pub bit_current_move_pos: usize,
}

pub fn create_new_state(
    bitboards: &mut Bitboards,
    player: i8,
    bit_current_move_pos: usize,
    white_captured_stone: i8,
    black_captured_stone: i8,
    win_move: Vec<(usize, i8)>,
) -> State {
    let mut new_state = State {
        bitboards: bitboards.clone(),
        current_player: player,
        white_captured_stone: white_captured_stone,
        black_captured_stone: black_captured_stone,
        available_move: vec![],
        heuristic: 0,
        win_move: win_move,
        win_state: 0,
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
    let mut copy_bitboards: Bitboards;
    let mut copy_win_pos: Vec<(usize, i8)>;
    let mut childs_list: Vec<State>;
    let index_box: Vec<usize> = get_search_box_bitboard(&state.bitboards);
    let len = index_box.len();
    childs_list = Vec::new();
    for pos in 0..len {
        copy_bitboards = state.bitboards.clone();
        copy_win_pos = state.win_move.clone();
        let bit_current_move_pos: usize = index_box[pos];

        let mut child = create_new_state(
            &mut copy_bitboards,
            -state.current_player,
            bit_current_move_pos,
            state.white_captured_stone,
            state.black_captured_stone,
            copy_win_pos,
        );
        if child.heuristic > -1000 {
            apply_state_move(&mut child);
            childs_list.push(child);
        }
    }
    return childs_list;
}

pub fn apply_state_move(state: &mut State) {}

pub fn state_is_terminated(state: &mut State) -> bool {
    return false;
}
