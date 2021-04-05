#[derive(Copy, Clone)]
pub struct Bitboards {
    pub white_board: [u64; 6],
    pub black_board: [u64; 6],
}

pub fn create_bitboards_from_vec(board: &Vec<Vec<i8>>) -> Bitboards {
    let mut new_bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    for x in 0..19 {
        for y in 0..19 {
            let real_pos = (x * 19 + y) % 64;
            let bit_pos = 63 - real_pos;
            let bitboards_index = (x * 19 + y) / 64;
            let mask = 1 << bit_pos;
            if board[x][y] == 1 {
                new_bitboards.white_board[bitboards_index] =
                    new_bitboards.white_board[bitboards_index] | mask;
            } else if board[x][y] == -1 {
                new_bitboards.black_board[bitboards_index] =
                    new_bitboards.black_board[bitboards_index] | mask;
            }
        }
    }
    return new_bitboards;
}

pub fn create_vec_from_bitboards(bitboards: &Bitboards) -> Vec<Vec<i8>> {
    let mut board: Vec<Vec<i8>> = vec![];
    for x in 0..19 {
        board.push(vec![]);
        for y in 0..19 {
            let real_pos = (x * 19 + y) % 64;
            let bit_pos = 63 - real_pos;
            let bitboards_index = (x * 19 + y) / 64;
            let mask = 1 << bit_pos;
            if bitboards.white_board[bitboards_index] & mask != 0 {
                board[x].push(1);
            } else if bitboards.black_board[bitboards_index] & mask != 0 {
                board[x].push(-1);
            } else {
                board[x].push(0);
            }
        }
    }
    return board;
}

pub fn apply_bitmove(bitboards: &mut Bitboards, pos:usize, player: i8) {
	let real_pos = pos % 64;
	let bit_pos = 63 - real_pos;
	let bitboards_index = pos / 64;
	let mask = 1 << bit_pos;
	if player == 1 {
		bitboards.white_board[bitboards_index] =
                    bitboards.white_board[bitboards_index] | mask;
	}
	else {
		bitboards.black_board[bitboards_index] =
                    bitboards.black_board[bitboards_index] | mask;
	}
}
