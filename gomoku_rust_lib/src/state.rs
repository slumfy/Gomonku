#[path = "check.rs"]
mod check;

pub struct State {
    board: Vec<Vec<i32>>,
    available_move: Vec<(i32, i32)>,
    player_to_play: i32,
    winstate: i32,
    heuristic: i32,
}

pub fn create_new_state(board: &mut Vec<Vec<i32>>, player: i32) -> State {
    let mut new_state = State {
        board: board.to_vec(),
        player_to_play: player,
        available_move: vec![],
        winstate: 0,
        heuristic: 0,
    };
    for x in 0..new_state.board.len() {
        for y in 0..new_state.board[x].len() {
            if check::check_wrong_position(&mut new_state.board, player, x as i32, y as i32) == 0 {
                new_state.available_move.push((x as i32, y as i32));
            }
        }
    }
    return new_state;
}

pub fn print_state(state: State) {
    println!("board {:?}", state.board);
    println!("availlable move {:?}", state.available_move);
    println!("player {:?}", state.player_to_play);
    println!("win  {:?}", state.winstate);
    println!("heuristic  {:?}", state.heuristic);
}
