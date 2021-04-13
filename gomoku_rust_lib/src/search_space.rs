use crate::bitboards::Bitboards;

#[allow(dead_code)]
pub fn get_search_box_bitboard(bitboard: &Bitboards) -> Vec<usize> {
    let mut box_position: Vec<usize> = vec![];
    for x in 0..19 {
        for y in 0..19 {
            let real_pos = (x * 19 + y) % 64;
            let bit_pos = 63 - real_pos;
            let bitboard_index = (x * 19 + y) / 64;
            let mask = 1 << bit_pos;
            if bitboard.white_board[bitboard_index] & mask != 0 {
                create_box_for_bitpos(x * 19 + y, &mut box_position);
            } else if bitboard.black_board[bitboard_index] & mask != 0 {
                create_box_for_bitpos(x * 19 + y, &mut box_position);
            }
            // println!("pos_box : {:?}",box_position);
        }
    }
    return box_position;
}

#[allow(dead_code)]
fn create_box_for_bitpos(bitpos: usize, box_position: &mut Vec<usize>) {
    let box_size = 1;
    let y = bitpos % 19;
    let x = bitpos / 19;
    let xmin = if x < box_size { 0 } else { x - box_size };
    let xmax = if x + box_size >= 18 { 18 } else { x + box_size };
    let ymin = if y < box_size { 0 } else { y - box_size };
    let ymax = if y + box_size >= 18 { 18 } else { y + box_size };
    for idx in xmin..xmax + 1 {
        for idy in ymin..ymax + 1 {
            if check_is_in_bitpos_list(box_position, idx * 19 + idy) == false {
                box_position.push(idx * 19 + idy);
            }
        }
    }
}

fn check_is_in_bitpos_list(box_position: &mut Vec<usize>, bitpos: usize) -> bool {
    let len = box_position.len();
    for pos in 0..len {
        if box_position[pos] == bitpos {
            return true;
        }
    }
    return false;
}


#[allow(dead_code)]
pub fn unwrap_bitlist(box_position: Vec<usize>) -> Vec<(usize, usize)> {
    let len = box_position.len();
    let mut unwrap = vec![];
    for pos in 0..len {
        unwrap.push((box_position[pos] / 19, box_position[pos] % 19));
    }
    return unwrap;
}
