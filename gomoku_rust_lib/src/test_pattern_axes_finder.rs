use crate::bitboard_operations::apply_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::bitpattern::pattern_axes_finder;
use crate::data_struct::Bitboards;
use crate::global_var;

pub fn test_pattern_axes_finder() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };

    // Alignement of 1
    let pos = 19;
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 3)]);

    // Alignement of 2
    apply_bit(&mut bitboards, pos + 1, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 3)]);

    // Alignement of 3
    apply_bit(&mut bitboards, pos + 2, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (5, 1)]);

    // Alignement of 4
    apply_bit(&mut bitboards, pos + 3, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (1, 1)]);
    test_alignment_of_five();
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
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);

    // Alignement of 5 defeatable
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos - 19, global_var::PLAYER_BLACK_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

    // Alignement of 5 undefeatable
    apply_bit(&mut bitboards, pos + 38, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);

    // Alignement of 5 defeatable
    apply_bit(&mut bitboards, pos + 21, global_var::PLAYER_WHITE_NB);
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

    // Alignement of 5 defeatable testing if pattern match in any pos
    let axes = create_bits_axes_from_pos(pos + 1, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos + 1,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

    // Alignement of 5 defeatable testing if pattern match in any pos

    let axes = create_bits_axes_from_pos(pos + 2, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos + 2,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

    // Alignement of 5 defeatable testing if pattern match in any pos
    let axes = create_bits_axes_from_pos(pos + 3, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos + 3,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

    // Alignement of 5 defeatable testing if pattern match in any pos
    let axes = create_bits_axes_from_pos(pos + 4, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos + 4,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 0)]);

    // Alignement of 5 undefeatable
    apply_bit(&mut bitboards, pos + 41, global_var::PLAYER_WHITE_NB);

    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 5), (0, 5), (0, 5), (0, 5)]);
}
