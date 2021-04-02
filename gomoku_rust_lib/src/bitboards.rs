#[derive(Copy, Clone)]
pub struct Bitboards {
    pub white_board: [i64; 6],
    pub black_board: [i64; 6],
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
            if board[x][y] == 1 {
                let mask = 1 << bit_pos;
                new_bitboards.white_board[bitboards_index] =
                    new_bitboards.white_board[bitboards_index] | mask;
            } else if board[x][y] == -1 {
                let mask = 1 << bit_pos;
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
