use crate::global_var;
use crate::utils::is_on_axe;
pub fn test_is_on_axe() {
    // Test line
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MIN_LIMITS,
            1,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MIN_LIMITS,
            1,
            -1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MAX_LIMITS,
            1,
            -1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MAX_LIMITS,
            1,
            1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[3],
            global_var::BOARD_MAX_LIMITS,
            1,
            -19
        ),
        false
    );

    // Test column
    assert_eq!(is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 0, 1, 1), true);
    assert_eq!(is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 0, 2, 1), true);
    assert_eq!(is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 0, 3, 1), true);
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[1],
            global_var::BOARD_MIN_LIMITS,
            4,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[1],
            global_var::BOARD_MIN_LIMITS,
            5,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[1],
            global_var::BOARD_MIN_LIMITS,
            1,
            -1
        ),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 1, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 2, -1),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 3, -1),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 4, -1),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 5, -1),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[1], 19, 6, -1),
        false
    );

    // Test diagonal right - left
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MIN_LIMITS,
            1,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MIN_LIMITS,
            2,
            1
        ),
        true
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MIN_LIMITS,
            1,
            -1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MAX_LIMITS,
            1,
            1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MAX_LIMITS,
            2,
            1
        ),
        false
    );
    assert_eq!(
        is_on_axe(
            global_var::AXE_MOUVEMENT_VALUE[0],
            global_var::BOARD_MAX_LIMITS,
            3,
            1
        ),
        false
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 1, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 2, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 3, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 4, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 5, 1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 1, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 2, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 3, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 4, -1),
        true
    );
    assert_eq!(
        is_on_axe(global_var::AXE_MOUVEMENT_VALUE[0], 180, 5, -1),
        true
    );
}
