use crate::bitboard_operations::apply_bit;
use crate::bitboard_operations::remove_bit;
use crate::check_move::check_pattern_is_not_capturable;
use crate::data_struct::Bitboards;
use crate::global_var;
// use crate::print::print_board_from_bitboard;

pub fn test_check_pattern_is_not_capturable_five() {
    let mut bitboards: Bitboards = Bitboards {
        white_board: [0, 0, 0, 0, 0, 0],
        black_board: [0, 0, 0, 0, 0, 0],
    };
    let mut pos = 0;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 0, 3, 1, 5),
        true
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in colone first pos
    let mut pos = 19;
    apply_bit(&mut bitboards, 0, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 19, 3, 1, 5),
        false
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in colone second pos
    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, 1, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 19, 3, 1, 5),
        false
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in colone third pos
    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, 2, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 19, 3, 1, 5),
        false
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in colone fourth pos
    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, 3, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 19, 3, 1, 5),
        false
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];
    // Can capture in colone last pos
    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, 4, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 19, 3, 1, 5),
        false
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 19, 3, 1, 5),
        true
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Can capture in diagonal
    let mut pos = 20;
    apply_bit(&mut bitboards, 0, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 20, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 20, 3, 1, 5),
        false
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 20;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 20, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 20, 3, 1, 5),
        true
    );

    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    // Try diagonal alignment
    let mut pos = 180;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 20;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 180, 3, 1, 5),
        true
    );

    // Can capture in colone
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 180;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;
    apply_bit(&mut bitboards, pos - 19, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);

    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 180, 0, 1, 5),
        false
    );

    // Cannot capture in colone because there is already a black stone
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 180;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;
    apply_bit(&mut bitboards, pos - 19, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 19, global_var::PLAYER_WHITE_NB);
    apply_bit(&mut bitboards, pos + 38, global_var::PLAYER_BLACK_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 20;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);

    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 180, 0, 1, 5),
        true
    );

    // Cannot capture in line because bigger than 2 whites stone
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 180;
    let mut black_pos = pos - 1;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    black_pos += 19;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    black_pos += 19;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    black_pos += 19;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    black_pos += 19;
    apply_bit(&mut bitboards, black_pos, global_var::PLAYER_BLACK_NB);
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos += 1;

    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);

    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, 180, 0, 1, 5),
        true
    );

    // Reverse check is unblockable five in two ways
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 6;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 1;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    remove_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, pos, 3, 1, 5),
        true
    );
    apply_bit(&mut bitboards, 7, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, pos, 3, 1, 5),
        true
    );

    // Reverse check is unblockable five in two ways
    bitboards.white_board = [0, 0, 0, 0, 0, 0];
    bitboards.black_board = [0, 0, 0, 0, 0, 0];

    let mut pos = 300;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    pos -= 19;
    apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    remove_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, pos, 3, 1, 5),
        true
    );
    apply_bit(&mut bitboards, 319, global_var::PLAYER_WHITE_NB);
    assert_eq!(
        check_pattern_is_not_capturable(&mut bitboards, pos, 3, 1, 5),
        true
    );
}
