pub struct Bitboard {
    pub white_board: [i64; 6],
    pub black_board: [i64; 6],
}

pub fn create_bitboard_from_vec(board: &Vec<Vec<i8>>) -> Bitboard {
    let mut new_bitboard = Bitboard {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    for x in 0..19 {
        for y in 0..19 {
            let real_pos = (x * 19 + y) % 64;
            let bit_pos = 63 - real_pos;
            let bitboard_index = (x * 19 + y) / 64;
            if board[x][y] == 1 {
                let mask = 1 << bit_pos;
                new_bitboard.white_board[bitboard_index] =
                    (new_bitboard.white_board[bitboard_index] | mask);
            } else if board[x][y] == -1 {
                let mask = 1 << bit_pos;
                new_bitboard.black_board[bitboard_index] =
                    (new_bitboard.black_board[bitboard_index] | mask);
            }
        }
    }
    for x in 0..6 {
        println!("white_board[{}]: {:064b}", x, new_bitboard.white_board[x]);
        println!("black_board[{}]: {:064b}", x, new_bitboard.black_board[x]);
    }
    return new_bitboard;
}

pub fn create_vec_from_bitboard(bitboard: &Bitboard) -> Vec<Vec<i8>> {
    let mut board: Vec<Vec<i8>> = vec![];
    for x in 0..19 {
        board.push(vec![]);
        for y in 0..19 {
            let real_pos = (x * 19 + y) % 64;
            let bit_pos = 63 - real_pos;
            let bitboard_index = (x * 19 + y) / 64;
            let mask = 1 << bit_pos;
            if bitboard.white_board[bitboard_index] & mask != 0 {
                board[x].push(1);
            } else if bitboard.black_board[bitboard_index] & mask != 0 {
                board[x].push(-1);
            } else {
                board[x].push(0);
            }
        }
    }
    println!("vecboard: {:?}", board);
    return board;
}
