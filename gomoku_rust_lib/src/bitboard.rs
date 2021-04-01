use std::fmt;

pub struct Bitboard {
	whiteboard: [i64; 6],
	blackboard: [i64; 6],
}

pub fn create_bitboard_from_vec(board: &Vec<Vec<i8>>) -> Bitboard {
	let mut new_bitboard = Bitboard {
		whiteboard: [0,0,0,0,0,0],
		blackboard: [0,0,0,0,0,0],
	};
	for x in 0..19 {
		for y in 0..19 {
			let realpos = (x * 19 + y) % 64;
			let bitpos = 63 - realpos;
			let boardidx = (x * 19 + y) / 64;
			if board[x][y] == 1 {
			let mask = 1 << bitpos;
			new_bitboard.whiteboard[boardidx] = (new_bitboard.whiteboard[boardidx] | mask);
			}
			else if board[x][y] == -1 {
			let mask = 1 << bitpos;
			new_bitboard.blackboard[boardidx] = (new_bitboard.blackboard[boardidx] | mask);
			}
		}
	}
	for x in 0..6 {
	println!("whiteboard[{}]: {:064b}",x, new_bitboard.whiteboard[x]);
	println!("blackboard[{}]: {:064b}",x, new_bitboard.blackboard[x]);
	}
	return new_bitboard;
}

pub fn create_vec_from_bitboard(bitboard: &Bitboard) -> Vec<Vec<i8>> {
	let mut board: Vec<Vec<i8>> = vec![];
	for x in 0..19 {
		board.push(vec![]);
		for y in 0..19 {
			let realpos = (x * 19 + y) % 64;
			let bitpos = 63 - realpos;
			let boardidx = (x * 19 + y) / 64;
			let mask = 1 << bitpos;
			if bitboard.whiteboard[boardidx] & mask != 0 {
				board[x].push(1);
			}
			else if bitboard.blackboard[boardidx] & mask != 0 {
				board[x].push(-1);
			}
			else {
				board[x].push(0);
			}
		}
	}
	println!("vecboard: {:?}", board);
	return board;
}