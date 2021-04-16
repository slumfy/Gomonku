use crate::bitboard_operations::apply_bit;
use crate::bitboard_operations::remove_bit;
use crate::bitboards::create_bits_axes_from_pos;
use crate::bitboards::Bitboards;
use crate::bitpattern::pattern_axes_finder;
use crate::global_var;

pub fn test_pattern_axes_finder() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let mut pos = 0;
    let axes = create_bits_axes_from_pos(pos, &mut bitboards, global_var::PLAYER_WHITE_NB);
    let returned_pattern = pattern_axes_finder(
        &mut bitboards,
        &axes[0],
        &axes[1],
        pos,
        global_var::PLAYER_WHITE_NB,
    )[0];
    assert_eq!(returned_pattern, [(0, 3), (0, 3), (0, 3), (0, 3)])
}
