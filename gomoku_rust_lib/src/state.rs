#[path = "check.rs"]
mod check;
#[path = "heuristic.rs"]
mod heuristic;

pub struct State {
    pub board: Vec<Vec<i32>>,
    pub available_move: Vec<State>,
    pub player_to_play: i32,
    winstate: i32,
    pub heuristic: i32,
	pub current_move: (i32,i32)
}

pub fn create_new_state(board: &mut Vec<Vec<i32>>, player: i32, current_move: (i32,i32)) -> State {
    let mut new_state = State {
        board: board.to_vec(),
        player_to_play: player,
        available_move: vec![],
        winstate: 0,
        heuristic: 0,
		current_move: current_move
    };
	new_state.heuristic = heuristic::heuristic(board, &new_state);
    return new_state;
}

pub fn print_state(state: State) {
    println!("board {:?}", state.board);
    println!("availlable move {:?}", state.available_move.len());
    println!("player {:?}", state.player_to_play);
    println!("win  {:?}", state.winstate);
    println!("heuristic  {:?}", state.heuristic);
	println!("current_move  {:?}", state.current_move);
}

pub fn create_child(state: &mut State) -> Vec<State> {
	let mut cpyboard: Vec<Vec<i32>>;
	let mut childlist: Vec<State>;
	let indexbox: ((i32,i32),(i32,i32)) = get_box(state);
	childlist = Vec::new();
	for x in indexbox.0.0..indexbox.0.1 {
		for y in indexbox.1.0..indexbox.1.1 {
			if check::check_wrong_position(&mut state.board, state.player_to_play, x as i32, y as i32) == 0 {
				cpyboard = state.board.clone();
				cpyboard[x as usize][y as usize] == -state.player_to_play;
				let child = create_new_state(&mut cpyboard,-state.player_to_play,(x as i32, y as i32));
				childlist.push(child);
			}
		}
	}
	return childlist;
}

fn get_box(state: &mut State) -> ((i32,i32),(i32,i32)) {
	let mut x_tuple: (i32,i32) = (0,18);
	let mut y_tuple: (i32,i32) = (0,18);
	if state.current_move.0 - 5 >= 0 {
		x_tuple.0 = state.current_move.0 - 5;
	}
	if state.current_move.0 + 5 <=18 {
		x_tuple.1 = state.current_move.1 + 5;
	}
	if state.current_move.1 - 5 >= 0 {
		y_tuple.0 = state.current_move.0 - 5;
	}
	if state.current_move.1 + 5 <= 18 {
		y_tuple.1 = state.current_move.1 + 5;
	}
	println!("box {:?}",(x_tuple,y_tuple));
	return (x_tuple,y_tuple);
}