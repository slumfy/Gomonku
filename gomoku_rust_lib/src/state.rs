struct State {
    board: [[i32; 19]; 19],
    available_move: Vec,
    player_to_play: i32,
    winstate: i32,
}
