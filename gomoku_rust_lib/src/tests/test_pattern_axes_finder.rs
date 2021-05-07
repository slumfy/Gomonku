use crate::bitboard_operations::apply_bit;
// use crate::bitboard_operations::remove_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::bitpattern::pattern_axes_finder;
use crate::print::print_board_from_bitboard;
use crate::data_struct::Bitboards;
use crate::global_var;

pub fn test_pattern_axes_finder() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    // Alignement of 1 blocked left
    let pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 3)]);

    // Alignement of 2 blocked left 0XX
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
	// print_board_from_bitboard(&bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (9, 1)]);

    // Alignement of 3 blocked left 0XXX...
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (5, 1)]);

    // Alignement of 4 blocked left
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (1, 1)]);
    test_alignment_of_five();
    test_four();
    test_three()
}

fn test_alignment_of_five() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    let pos = 19;

    // Alignement of 5 undefeatable
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);
    // Alignement of 5 undefeatable in up left corner
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    let pos = 0;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 20, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 60, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 80, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);

    // Alignement of 5 undefeatable in up right corner
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    let pos = 18;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 18, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 36, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 54, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 72, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);

    // Alignement of 5 undefeatable in down right corner
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 360;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 20, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 60, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 80, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);

    //alignement of 5 undefeatable in collum
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    let pos = 3;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 38, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 57, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 76, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);
}

fn test_four() {
    //alignement of 4 in collum
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 3;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 38, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 57, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (1, 1), (0, 3), (0, 3)]);

    // split four three
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (2, 0)]);

    // split four three diagonal right
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 20, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 80, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(2, 1), (0, 3), (0, 3), (0, 3)]);

    // split four three diagonal left
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 24;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 18, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 36, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 72, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (2, 0), (0, 3)]);

    // four next to border 
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 42;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 18, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 36, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 54, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
	// print_board_from_bitboard(&bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (1, 0), (0, 3)]);

    // split four one border left
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    print_board_from_bitboard(&bitboards);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (3, 0)]);

    // split four one
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (3, 0)]);

    // split four two
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (4, 0)]);

    // split four two in up left corner
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 0;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 20, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 60, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 80, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(4, 1), (0, 3), (0, 3), (0, 3)]);
}


fn test_three() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    // three2
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 4, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (6, 0)]);

    // split three
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (7, 0)]);

    // split three in up left corner
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 0;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 60, global_var::PLAYER_WHITE_NB);
    print_board_from_bitboard(&bitboards);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(7, 1), (0, 3), (0, 3), (0, 3)]);

    // split three rev
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 2;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (8, 0)]);

// split three rev rigth border
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 17;
    apply_bit(&mut bitboards, pos , global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 2, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 3, global_var::PLAYER_WHITE_NB);
    print_board_from_bitboard(&bitboards);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (8, 0)]);

    // split three rev down right corner
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let pos = 360;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 40, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 60, global_var::PLAYER_WHITE_NB);
    print_board_from_bitboard(&bitboards);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(8, 1), (0, 3), (0, 3), (0, 3)]);
}

//     // Alignement of 5 defeatable
//     apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
//     apply_bit(&mut bitboards, pos - 19, global_var::PLAYER_BLACK_NB);
//     let axes = create_bits_axes_from_pos(pos, &mut bitboards);
//     let returned_pattern = pattern_axes_finder(
//         &mut bitboards,
//         &axes[0],
//         &axes[1],
//         pos,
//         global_var::PLAYER_WHITE_NB,
//     )[0];
//     assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

//     // Alignement of 5 undefeatable
//     apply_bit(&mut bitboards, pos + 38, global_var::PLAYER_WHITE_NB);
//     let axes = create_bits_axes_from_pos(pos, &mut bitboards);
//     let returned_pattern = pattern_axes_finder(
//         &mut bitboards,
//         &axes[0],
//         &axes[1],
//         pos,
//         global_var::PLAYER_WHITE_NB,
//     )[0];
//     assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);

//     // Alignement of 5 defeatable
//     apply_bit(&mut bitboards, pos + 21, global_var::PLAYER_WHITE_NB);
//     let axes = create_bits_axes_from_pos(pos, &mut bitboards);
//     let returned_pattern = pattern_axes_finder(
//         &mut bitboards,
//         &axes[0],
//         &axes[1],
//         pos,
//         global_var::PLAYER_WHITE_NB,
//     )[0];
//     assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

//     // Alignement of 5 defeatable testing if pattern match in any pos
//     let axes = create_bits_axes_from_pos(pos + 1, &mut bitboards);
//     let returned_pattern = pattern_axes_finder(
//         &mut bitboards,
//         &axes[0],
//         &axes[1],
//         pos + 1,
//         global_var::PLAYER_WHITE_NB,
//     )[0];
//     assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

//     // Alignement of 5 defeatable testing if pattern match in any pos

//     let axes = create_bits_axes_from_pos(pos + 2, &mut bitboards);
//     let returned_pattern = pattern_axes_finder(
//         &mut bitboards,
//         &axes[0],
//         &axes[1],
//         pos + 2,
//         global_var::PLAYER_WHITE_NB,
//     )[0];
//     assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

//     // Alignement of 5 defeatable testing if pattern match in any pos
//     let axes = create_bits_axes_from_pos(pos + 3, &mut bitboards);
//     let returned_pattern = pattern_axes_finder(
//         &mut bitboards,
//         &axes[0],
//         &axes[1],
//         pos + 3,
//         global_var::PLAYER_WHITE_NB,
//     )[0];
//     assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

//     // Alignement of 5 defeatable testing if pattern match in any pos
//     let axes = create_bits_axes_from_pos(pos + 4, &mut bitboards);
//     let returned_pattern = pattern_axes_finder(
//         &mut bitboards,
//         &axes[0],
//         &axes[1],
//         pos + 4,
//         global_var::PLAYER_WHITE_NB,
//     )[0];
//     assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

//     // Alignement of 5 undefeatable
//     apply_bit(&mut bitboards, pos + 41, global_var::PLAYER_WHITE_NB);

//     let axes = create_bits_axes_from_pos(pos, &mut bitboards);
//     let returned_pattern = pattern_axes_finder(
//         &mut bitboards,
//         &axes[0],
//         &axes[1],
//         pos,
//         global_var::PLAYER_WHITE_NB,
//     )[0];
//     assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);
// }
