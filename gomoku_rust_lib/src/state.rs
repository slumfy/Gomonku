#[path = "check.rs"]
mod check;
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
    // new_state.heuristic = heuristic::heuristic(board, new_state);
    return new_state;
}
