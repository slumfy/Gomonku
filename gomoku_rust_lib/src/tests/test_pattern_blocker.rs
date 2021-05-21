use crate::bitboard_operations::apply_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::bitpattern::pattern_axes_finder;
use crate::data_struct::Bitboards;
use crate::global_var;

pub fn test_blockers() {
    println!("test_blocker_doubles_border");
    test_blocker_doubles_border();

    println!("test_blocker_doubles_1_blocker_left");
    test_blocker_doubles_1_blocker_left();

    println!("test_blocker_doubles_1_blocker_right");
    test_blocker_doubles_1_blocker_right();

    println!("test_blocker_doubles_2_blocker_left");
    test_blocker_doubles_2_blocker_left();

    println!("test_blocker_doubles_2_blocker_right");
    test_blocker_doubles_2_blocker_right();

    println!("test_blocker_triple_border");
    test_blocker_triple_border();

    println!("test_blocker_triple_1_blocker_left");
    test_blocker_triple_1_blocker_left();

    println!("test_blocker_triple_1_blocker_right");
    test_blocker_triple_1_blocker_right();

    println!("test_blocker_triple_2_blocker_right");
    test_blocker_triple_2_blocker_right();

    println!("test_blocker_triple_2_blocker_left");
    test_blocker_triple_2_blocker_left();

    println!("test_blocker_triple_2_with_hole_blocker_left");
    test_blocker_triple_2_with_hole_blocker_left();

    println!("test_blocker_triple_2_with_hole_blocker_right");
    test_blocker_triple_2_with_hole_blocker_right();

    println!("test_blocker_triple_3_blocker_right");
    test_blocker_triple_3_blocker_right();

    println!("test_blocker_triple_3_blocker_middle");
    test_blocker_triple_3_blocker_middle();

    println!("test_blocker_triple_3_blocker_left");
    test_blocker_triple_3_blocker_left();

    println!("test_blocker_split_triple_rev_1_blocker");
    test_blocker_split_triple_rev_1_blocker();

    println!("test_blocker_split_triple_rev_2_blocker");
    test_blocker_split_triple_rev_2_blocker();

    println!("test_blocker_split_triple_rev_2_blocker_wrong");
    test_blocker_split_triple_rev_2_blocker_wrong();

    println!("test_blocker_split_triple_1_blocker");
    test_blocker_split_triple_1_blocker();

    println!("test_blocker_split_triple_2_blocker");
    test_blocker_split_triple_2_blocker();

    println!("test_blocker_split_triple_2_blocker_wrong");
    test_blocker_split_triple_2_blocker_wrong();

    println!("test_blocker_four_1_blocker");
    test_blocker_four_1_blocker();

    println!("test_blocker_four_2_blocker");
    test_blocker_four_2_blocker();

    println!("test_blocker_split_four2_1_blocker");
    test_blocker_split_four2_1_blocker();

    println!("test_blocker_split_four2_2_blocker");
    test_blocker_split_four2_2_blocker();

    println!("test_blocker_split_four1_1_blocker");
    test_blocker_split_four1_1_blocker();

    println!("test_blocker_split_four1_1_blocker_wrong");
    test_blocker_split_four1_1_blocker_wrong();

    println!("test_blocker_split_four1_2_blocker");
    test_blocker_split_four1_2_blocker();

    println!("test_blocker_split_four1_2_blocker_wrong");
    test_blocker_split_four1_2_blocker_wrong();

    println!("test_blocker_split_four3_1_blocker");
    test_blocker_split_four3_1_blocker();

    println!("test_blocker_split_four3_1_blocker_wrong");
    test_blocker_split_four3_1_blocker_wrong();

    println!("test_blocker_split_four3_2_blocker");
    test_blocker_split_four3_2_blocker();

    println!("test_blocker_split_four3_2_blocker_wrong");
    test_blocker_split_four3_2_blocker_wrong();
}

fn test_blocker_doubles_border() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 0;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 2, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 2,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("\nwhite_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 2)]);
}

fn test_blocker_doubles_1_blocker_left() {
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
}

fn test_blocker_doubles_1_blocker_right() {
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
}

fn test_blocker_doubles_2_blocker_left() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_BLACK_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 2)]);
}

fn test_blocker_doubles_2_blocker_right() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 2)]);
}

fn test_blocker_triple_border() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 0;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
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
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 2)]);
}

fn test_blocker_triple_1_blocker_left() {
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
}

fn test_blocker_triple_1_blocker_right() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 1)]);
}

fn test_blocker_triple_2_blocker_right() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
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
}

fn test_blocker_triple_2_blocker_left() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_BLACK_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 2)]);
}

fn test_blocker_triple_2_with_hole_blocker_left() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
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
}

fn test_blocker_triple_2_with_hole_blocker_right() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_BLACK_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (6, 2)]);
}

fn test_blocker_triple_3_blocker_right() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (0, 3)]);
}

fn test_blocker_triple_3_blocker_middle() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 1, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 1,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (0, 3)]);
}

fn test_blocker_triple_3_blocker_left() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_BLACK_NB);
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
}

fn test_blocker_split_triple_rev_1_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
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
}

fn test_blocker_split_triple_rev_2_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (8, 2)]);
}

fn test_blocker_split_triple_rev_2_blocker_wrong() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_BLACK_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (0, 3)]);
}

fn test_blocker_split_triple_1_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (9, 1)]);
}

fn test_blocker_split_triple_2_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 2, &mut bitboards);
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

fn test_blocker_split_triple_2_blocker_wrong() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_BLACK_NB);
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
}

fn test_blocker_four_1_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (1, 1)]);
}

fn test_blocker_four_2_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_BLACK_NB);
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

fn test_blocker_split_four2_1_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
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
}

fn test_blocker_split_four2_2_blocker() {
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
}

fn test_blocker_split_four1_1_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 6, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 6, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 6,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 1)]);
}

fn test_blocker_split_four1_1_blocker_wrong() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
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
}

fn test_blocker_split_four1_2_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 2, &mut bitboards);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (3, 2)]);
}

fn test_blocker_split_four1_2_blocker_wrong() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos + 6, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 6, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 6,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (5, 2)]);
}

fn test_blocker_split_four3_1_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
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
}

fn test_blocker_split_four3_1_blocker_wrong() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 6, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 6, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 6,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (0, 3)]);
}

fn test_blocker_split_four3_2_blocker() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
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
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (2, 2)]);
}

fn test_blocker_split_four3_2_blocker_wrong() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 5, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos + 6, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos + 6, &mut bitboards);
    let returned_blocker = pattern_axes_finder(
        &mut bitboards,
        &axes[1],
        &axes[0],
        pos + 6,
        global_var::PLAYER_BLACK_NB,
    )[1];
    println!("white_axes {:016b}", axes[0][3]);
    println!("black_axes {:016b}", axes[1][3]);
    println!("return_pattern {:?}", returned_blocker);
    assert_eq!(returned_blocker, [(0, 3), (0, 3), (0, 3), (0, 3)]);
}
