use crate::bitboards::Bitboards;
use crate::bitboards::remove_bit;
use crate::print_bitboards;
use crate::global_var::AXE_MOUVEMENT_VALUE;
// patern need to sort by order of check
static PATTERN: [(u8, usize, &str); 8] = [
    (0xF8, 6, "five"),         // five XXXXX...
    (0x74, 7, "split four 3"), // split four 3 .XXX.X..
    (0x6C, 7, "split four 2"), // split four 2 .XX.XX..
    (0x5C, 7, "split four 1"), // split four 1 .X.XXX..
    // (0xE8, 7, "close split four 3"),    // close split four 3 XXX.X...
    // (0xD8, 7, "close split four 2"),    // close split four 2 XX.XX...
    // (0xB8, 7, "close split four 1"),    // close split four 1 X.XXX...
    // (0xF0, 5, "close four"),            // close four XXXX....
    // (0xB0, 5, "close split three"),     // close split three X.XX....
    // (0xD0, 5, "close split three rev"), // close split three rev XX.X....
    // (0xE0, 4, "close three"),           // close three XXX.....
    (0x78, 6, "open four"),            // open four .XXXX...
    (0x68, 6, "open split three"),     // open split three .X.XX...
    (0x58, 6, "open split three rev"), // open split three rev .XX.X...
    (0x70, 5, "open three"),           // open three  .XXX....
];

static CAPTURE_PATTERN: [(u8, usize, &str); 2] = [
    (0x90, 5, "capturing pair"), // eat pair	  X..X....
    (0x60, 4, "open double"),    // open double .XX.....
];

static BLOCKER: [(u8, usize); 5] = [
    (0x82, 7), // X.....X.
    (0x84, 6), // X....X..
    (0x88, 5), // X...X...
    (0x90, 4), // X..X....
    (0xA0, 3), // X.X.....
];

pub fn pattern_axes_dispatcher(bitboards: &mut Bitboards, axes: &[[u16; 4]; 2], pos: usize, player: i8) {
    if player == 1 {
        println!("white player pattern in row:");
        // check and apply capture
        check_capture(bitboards, &axes[0], &axes[1], pos, player);
        check_flank(&axes[0], &axes[1]);
        pattern_axes_finder(&axes[0], &axes[1], pos);
    } else if player == -1 {
        println!("black player pattern in row:");
        check_capture(bitboards, &axes[1], &axes[0], pos, player);
        check_flank(&axes[1], &axes[0]);
        pattern_axes_finder(&axes[1], &axes[0], pos);
    }
}

fn check_capture(bitboards: &mut Bitboards, axes: &[u16; 4], blocker_axes: &[u16; 4], pos: usize, player: i8) {
    for axe in 0..axes.len() {
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        player_axe >>= 1;
        blocker_axe >>= 1;
        println!("player axe: {:016b}", player_axe);
        println!("block  axe: {:016b}", blocker_axe);
        let shift: [usize; 2] = [0, 3];
        for s in shift.iter() {
            let player_shifted = player_axe >> s;
            println!("player shifted: {:016b} l= {}", player_shifted, s);
            let blocker_shifted = blocker_axe >> s;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            if (player_casted & CAPTURE_PATTERN[0].0) == CAPTURE_PATTERN[0].0 {
                if (blocker_casted & CAPTURE_PATTERN[1].0) == CAPTURE_PATTERN[1].0 {
                    println!("captured");
                    println!(
                        "axe: {}, direction {}, pos {}",
                        AXE_MOUVEMENT_VALUE[axe], s, pos
                    );
                    if *s == 3 { apply_capture(bitboards, AXE_MOUVEMENT_VALUE[axe], -1, pos, player); }
                    else { apply_capture(bitboards, AXE_MOUVEMENT_VALUE[axe], 1, pos, player); }
                }
                print_bitboards(bitboards);
            }
        }
    }
}

fn apply_capture(bitboards: &mut Bitboards, axe: usize, s: isize, pos: usize, player: i8){
    let opponent = -player;
    for n in 1..3 {
        if s == -1 { remove_bit(bitboards, pos - (n * axe), opponent); }
        else { remove_bit(bitboards, pos + (n * axe), opponent);}
    }
}

fn check_flank(axes: &[u16; 4], blocker_axes: &[u16; 4]) {
    for axe in 0..axes.len() {
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        player_axe >>= 1;
        blocker_axe >>= 1;
        println!("player axe: {:016b}", player_axe);
        println!("block  axe: {:016b}", blocker_axe);
        let shift: [usize; 2] = [1, 2];
        for s in shift.iter() {
            let player_shifted = player_axe >> s;
            let blocker_shifted = blocker_axe >> s;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            if (player_casted & CAPTURE_PATTERN[1].0) == CAPTURE_PATTERN[1].0 {
                if (blocker_casted & CAPTURE_PATTERN[0].0) == CAPTURE_PATTERN[0].0 {
                    println!("blocked");
                } else if (blocker_casted & CAPTURE_PATTERN[0].0) != 0 {
                    println!("flank");
                } else {
                    println!("free");
                }
            }
        }
    }
}

fn print_axe_value(axe: usize) {
    if axe == 0 {
        println!("DIAGONALE UPLEFT:")
    } else if axe == 1 {
        println!("COLONE:")
    } else if axe == 2 {
        println!("DIAGONALE UPRIGHT:")
    } else {
        println!("LIGNE:")
    }
}

fn check_in_map(
    axe_mouvement_value: i16,
    pattern_pos: i16,
    offset: i16,
    direction_sign: i16,
) -> bool {
    let calcul = (pattern_pos + axe_mouvement_value * offset * direction_sign);
    if calcul < 0 {
        return false;
    }
    let line_checked = calcul / 19;
    let line = pattern_pos / 19;
    //ligne
    if axe_mouvement_value == 1 {
        if line_checked != line {
            return false;
        }
    } else {
        println!("checkpos {} pos {}", calcul, pattern_pos);
        println!(
            "line check {} diff {}",
            line_checked,
            line + offset * direction_sign
        );
        if line_checked != line + offset * direction_sign {
            return false;
        }
    }
    return true;
}

fn check_border(pos: usize, l: usize, axe: usize, pattern_length: usize) -> bool {
    let axe_mouvement_value: i16 = AXE_MOUVEMENT_VALUE[axe] as i16;
    let pattern_pos: i16 = pos as i16 - l as i16 * axe_mouvement_value;
    if check_in_map(axe_mouvement_value, pattern_pos, 1, -1) == false {
        return false;
    }
    if check_in_map(axe_mouvement_value, pattern_pos, pattern_length as i16, 1) == false {
        return false;
    }
    return true;
}

fn pattern_axes_finder(axes: &[u16; 4], blocker_axes: &[u16; 4], pos: usize) {
    let y = pos % 19;
    println!("y= {}", y);
    for axe in 0..axes.len() {
        print_axe_value(axe);
        let mut player_axe = axes[axe];
        let mut blocker_axe = blocker_axes[axe];
        let mut found_pattern: (&str, usize) = ("", 0);
        player_axe >>= 1;
        blocker_axe >>= 1;
        println!("player axe: {:016b}", player_axe);
        for l in 0..4 {
            let mut player_shifted = player_axe >> l;
            println!("player shifted: {:016b} l= {}", player_shifted, l);
            let mut blocker_shifted = blocker_axe >> l;
            let player_casted = player_shifted as u8;
            let blocker_casted = blocker_shifted as u8;
            let mut is_bloked: usize = 0;
            for p in 0..PATTERN.len() {
                if (player_casted & PATTERN[p].0) == PATTERN[p].0 {
                    // println!("player casted: {:08b}", player_casted);
                    for b in 0..BLOCKER.len() {
                        if BLOCKER[b].1 == PATTERN[p].1 {
                            // println!("blocker  cast: {:08b}", blocker_casted);
                            // println!("blocked checked: {:08b}", blocker_casted);
                            // println!("pattern checked: {:08b}", BLOCKER[b].0);
                            let mut blocker_checker: u8 = (blocker_casted & BLOCKER[b].0);
                            // println!("BLOCKER CHECKER: {:08b}", blocker_checker);
                            // println!("l {} length {}", l , PATTERN[p].1);
                            if blocker_checker == BLOCKER[b].0 {
                                is_bloked = 2;
                            } else if blocker_checker != 0 {
                                is_bloked = 1;
                                if check_border(pos, l, axe, PATTERN[p].1) == false {
                                    is_bloked += 1
                                }
                            } else if check_border(pos, l, axe, PATTERN[p].1) == false {
                                is_bloked = 1;
                                if blocker_checker != 0 {
                                    is_bloked += 1
                                }
                            } else {
                                is_bloked = 0;
                            }
                        }
                    }
                    if is_bloked < 2 {
                        found_pattern = (PATTERN[p].2.clone(), is_bloked);
                        println!("{} found {} blocker", PATTERN[p].2, is_bloked);
                        break;
                    }
                }
                if found_pattern.0 != "" {
                    break;
                }
            }
        }
        println!(
            "PATTERN FOUND {} BLOCKER FOUND {}",
            found_pattern.0, found_pattern.1
        );
    }
}
