use crate::state::State;

fn create_lists_from_stone_position(state: &State) -> Vec<Vec<i8>> {
    let board = &state.board;
    let box_half_size: isize = 5;
    let player = state.player_to_play;
    let stone_x = state.current_move.0;
    let stone_y = state.current_move.1;
    let mut lists: Vec<Vec<i8>> = vec![];

    lists.push(vec![player]);
    lists.push(vec![player]);
    lists.push(vec![player]);
    lists.push(vec![player]);

    for i in 1..box_half_size {
        // Vertical x
        if stone_x - i >= 0 {
            lists[0].insert(0, board[(stone_x - i) as usize][stone_y as usize]);
        }
        if stone_x + i < 19 {
            lists[0].push(board[(stone_x + i) as usize][stone_y as usize]);
        }
        // Horizontal y
        if stone_y - i >= 0 {
            lists[1].insert(0, board[(stone_x) as usize][(stone_y - i) as usize]);
        }
        if stone_y + i < 19 {
            lists[1].push(board[stone_x as usize][(stone_y + i) as usize]);
        }
        // Diagonal top left
        if stone_y + i < 19 && stone_x + i < 19 {
            lists[2].push(board[(stone_x + i) as usize][(stone_y + i) as usize]);
        }
        if stone_y - i >= 0 && stone_x - i >= 0 {
            lists[2].insert(0, board[(stone_x - i) as usize][(stone_y - i) as usize]);
        }
        // Diagonal top right
        if stone_y + i < 19 && stone_x - i >= 0 {
            lists[3].push(board[(stone_x - i) as usize][(stone_y + i) as usize]);
        }
        if stone_y - i >= 0 && stone_x + i < 19 {
            lists[3].insert(0, board[(stone_x + i) as usize][(stone_y - i) as usize]);
        }
    }
    return lists;
}

pub fn count_biggest_alignment(state: State) -> i32 {
    let lists = create_lists_from_stone_position(&state);
    let player = state.player_to_play;
    let opponent = -player;
    println!("vertical x = {:?}", lists[0]);
    println!("horizontal y = {:?}", lists[1]);
    println!("diagonal top left = {:?}", lists[2]);
    println!("diagonal top right = {:?}", lists[3]);

    return 0;
}
