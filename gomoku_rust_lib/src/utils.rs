use crate::state::State;

pub fn create_axes_from_stone_position(
    state: &State,
    x: isize,
    y: isize,
    player: i8,
) -> Vec<Vec<i8>> {
    let board = &state.board;
    let box_half_size: isize = 5;
    let player = player;
    let stone_x = x;
    let stone_y = y;
    let mut axes: Vec<Vec<i8>> = Vec::new();

    axes.push(vec![player]);
    axes.push(vec![player]);
    axes.push(vec![player]);
    axes.push(vec![player]);

    for i in 1..box_half_size {
        // Vertical x
        if stone_x - i >= 0 {
            axes[0].insert(0, board[(stone_x - i) as usize][stone_y as usize]);
        } else {
            axes[0].insert(0, -2);
        }
        if stone_x + i < 19 {
            axes[0].push(board[(stone_x + i) as usize][stone_y as usize]);
        } else {
            axes[0].push(-2);
        }

        // Horizontal y
        if stone_y - i >= 0 {
            axes[1].insert(0, board[(stone_x) as usize][(stone_y - i) as usize]);
        } else {
            axes[1].insert(0, -2);
        }
        if stone_y + i < 19 {
            axes[1].push(board[stone_x as usize][(stone_y + i) as usize]);
        } else {
            axes[1].push(-2);
        }

        // Diagonal top left
        if stone_y + i < 19 && stone_x + i < 19 {
            axes[2].push(board[(stone_x + i) as usize][(stone_y + i) as usize]);
        } else {
            axes[2].push(-2);
        }
        if stone_y - i >= 0 && stone_x - i >= 0 {
            axes[2].insert(0, board[(stone_x - i) as usize][(stone_y - i) as usize]);
        } else {
            axes[2].insert(0, -2);
        }

        // Diagonal top right
        if stone_y + i < 19 && stone_x - i >= 0 {
            axes[3].push(board[(stone_x - i) as usize][(stone_y + i) as usize]);
        } else {
            axes[3].push(-2);
        }
        if stone_y - i >= 0 && stone_x + i < 19 {
            axes[3].insert(0, board[(stone_x + i) as usize][(stone_y - i) as usize]);
        } else {
            axes[3].insert(0, -2);
        }
    }
    return axes;
}
