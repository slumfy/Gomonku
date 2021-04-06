use crate::bitboards::Bitboards;
// patern need to sort by order of check
static PATTERN: [(u8, usize, &str); 12] = [
    (0xF8, 5, "five"),                  // five XXXXX...
    (0x74, 6, "split four 3"),          // split four 3 .XXX.X..
    (0x6C, 6, "split four 2"),          // split four 2 .XX.XX..
    (0x5C, 6, "split four 1"),          // split four 1 ..X.XXX.
    (0xF0, 5, "close four"),            // close four XXXX....
    (0xB0, 5, "close split three"),     // close split three X.XX....
    (0xD0, 5, "close split three rev"), // close split three rev XX.X....
    (0xE0, 4, "close three"),           // close three XXX.....
    (0x78, 6, "open four"),             // open four .XXXX...
    (0x68, 6, "open split three"),      // open split three .X.XX...
    (0x58, 6, "open split three rev"),  // open split three rev .XX.X...
    (0x70, 5, "open three"),
]; // open three .XXX....

pub fn pattern_dispatcher(bitboards: &Bitboards, pos: usize, player: i8) {
    if player == 1 {
        println!("white player pattern in row:");
        pattern_finder(&bitboards.white_board, pos);
    } else if player == -1 {
        println!("black player pattern in row:");
        pattern_finder(&bitboards.black_board, pos);
    }
}

fn pattern_finder(bitboard: &[u64; 6], pos: usize) {
    println!("pos: {}", pos);
    println!("x idx: {}", pos / 19);
    println!("y idx: {}", pos % 19);
    println!("int idx: {}", pos / 64);
    let y = pos % 19;
    let mut int_to_check = create_row(bitboard, pos);
    println!("int checked: {:064b}", int_to_check);
    int_to_check <<= 5;
    for l in 0..19 {
        let mut int_shifted = int_to_check >> l;
        // println!("int checked shifted: {:064b}", int_shifted);
        let casted_int = int_shifted as u8;

        for p in 0..PATTERN.len() {
            // println!("int checked: {:08b}", casted_int);
            // println!("partern checked: {:08b}", PATTERN[p].0);
            // println!("& bit operator: {:08b}", (casted_int & PATTERN[p].0));
            if (casted_int & PATTERN[p].0) == PATTERN[p].0 {
                println!("{} found", PATTERN[p].2);
            }
        }
    }
}

fn create_row(bitboard: &[u64; 6], pos: usize) -> u64 {
    let mask: [u64; 24] = [
        0xFFFFE00000000000,
        0x1FFFFC000000,
        0x3FFFF80,
        0x7F,
        0xFFFFE00000000,
        0x1FFFFC000,
        0x3FFF,
        0x7FFFF0000000000,
        0xFFFFE00000,
        0x1FFFFC,
        0x3,
        0x7FFFF0000000,
        0xFFFFE00,
        0x1FF,
        0x3FFFF800000000,
        0x7FFFF0000,
        0xFFFF,
        0x1FFFFC0000000000,
        0x3FFFF800000,
        0xFFF0000000000000,
        0xF800000000000000,
        0xFFFF800000000000,
        0xFFC0000000000000,
        0xE000000000000000,
    ];
    let row_idx = pos / 19;
    let int_idx = pos / 64;
    let shift = (row_idx * 19) % 64;
    let mut row: u64;
    if (row_idx == 3 || row_idx == 6 || row_idx == 10 || row_idx == 13 || row_idx == 16) {
        row = (((bitboard[int_idx] & mask[row_idx]) << shift)
            | ((bitboard[int_idx + 1] & mask[19 + int_idx]) >> (64 - shift)));
    } else {
        row = (bitboard[int_idx] & mask[row_idx]) << shift;
    }
    return row >> 45;
}
