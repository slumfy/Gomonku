use crate::state::State;

// pub fn get_box(state: &mut State) -> ((isize, isize), (isize, isize)) {
//     let offset = 5;
//     let mut x_tuple: (isize, isize) = (0, 19);
//     let mut y_tuple: (isize, isize) = (0, 19);
//     if state.current_move.0 - offset >= 0 {
//         x_tuple.0 = state.current_move.0 - offset;
//     }
//     if state.current_move.0 + offset + 1 <= 19 {
//         x_tuple.1 = state.current_move.0 + offset + 1;
//     }
//     if state.current_move.1 - offset >= 0 {
//         y_tuple.0 = state.current_move.1 - offset;
//     }
//     if state.current_move.1 + offset + 1 <= 19 {
//         y_tuple.1 = state.current_move.1 + offset + 1;
//     }
//      return (x_tuple, y_tuple);
// }

pub fn get_search_box(state: &mut State) -> Vec<(usize,usize)> {
	let mut box_position = vec!();
	for x in 0..19 {
		for y in 0..19 {
			if state.board[x][y] != 0 {
				create_box_for_pos(&mut box_position, x ,y);
			}
		}
	}
	return box_position;
}

fn create_box_for_pos(box_position: &mut Vec<(usize,usize)>, x: usize, y: usize) {
	let offset = 2;
	let xmin = if x < offset { 0 } else { x - offset};
	let xmax = if  x + offset + 1 > 19 { y } else { x + offset + 1};
	let ymin = if  y < offset { 0 } else { y - offset};
	let ymax = if  y + offset + 1 > 19 { y } else { y + offset + 1};
	for idx in xmin..xmax {
		for idy in ymin..ymax{
			if check_is_in_pos_list(box_position,idx,idy) == false {
				box_position.push((idx,idy));
			}
		}
	}
}

fn check_is_in_pos_list(box_position: &mut Vec<(usize,usize)>, x: usize, y: usize) -> bool {
	let len = box_position.len();
	for pos in 0..len {
		if box_position[pos].0 == x && box_position[pos].1 == y{
			return true;
		}
	}
	return false;
}
