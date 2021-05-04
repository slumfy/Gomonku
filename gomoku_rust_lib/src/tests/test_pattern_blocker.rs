use crate::bitboard_operations::apply_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::bitpattern::pattern_axes_finder;
use crate::data_struct::Bitboards;
use crate::global_var;

pub fn test_pattern_blockers() {
    test_blocker_doubles();
    test_blocker_triples();
    test_blocker_four();
}

fn test_blocker_doubles() {
    //double 1 blocker left
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("\nwhite_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 1)]);
    //double 1 blocker right
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 3, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 3,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("\nwhite_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 1)]);
    //double 2 blocker
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 3, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 2)]);
    
}
fn test_blocker_triples() {
    //triple one blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 1)]);

    //triple two with hole blocker
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 5, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 5,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 2)]);
    //triple two blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 4, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 4,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 2)]);

	//triple two blocker opposite
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 2)]);

    //triple triple blocker
    apply_bit(&mut bitboards, pos+5, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos+5,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (0, 3)]);

     //split triple rev 1blocker
     let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 1)]);

    //split triple rev 2 blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos+3, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 3,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (8, 2)]);

    //split triple 1blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos+5, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos+5, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos+5,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 1)]);

    //split triple 2 blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos+2, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 2,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (7, 2)]);
}

fn test_blocker_four() {
    //open split four2 2 blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 3, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 3,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (4, 2)]);

    //open split four2 1 blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 1)]);

    //open split four1 1 blocker left
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (0, 3)]);

    //open split four1 1 blocker right
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos+6, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos+6, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos+6,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 1)]);

    //open split four1 2 blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos+2, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos+2, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos+2,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (3, 2)]);

    //open split four3 1 blocker left
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 1)]);

    //open split four3 1 blocker right
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos+6, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos+6, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos+6,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (0, 3)]);

    //open split four3 2 blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos+4, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos+4, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos+4,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (2, 2)]);

    //open four 1 blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos+5, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos+5, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos+5,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (1, 1)]);

    //open four 2 blocker
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos+5, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (1, 2)]);
}
