pub static HEURISTIC_MAX_VALUE: i32 = i32::MAX;
pub static HEURISTIC_MIN_VALUE: i32 = i32::MIN;

pub static HEURISTIC_CAPTURE_TEN_STONE: i32 = HEURISTIC_MAX_VALUE;
pub static _HEURISTIC_CAPTURE_AN_OPPONENT_FIVE_IN_A_ROW: i32 =
    _HEURISTIC_PREVENT_OPPONENT_WIN_BY_CAPTURE * 5;
pub static _HEURISTIC_PREVENT_OPPONENT_WIN_BY_CAPTURE: i32 =
    HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW * 5;
pub static HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW: i32 = HEURISTIC_FIVE_IN_A_ROW * 5;
pub static HEURISTIC_FIVE_IN_A_ROW: i32 = HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW * 5;
pub static HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW: i32 = HEURISTIC_FREE_FOUR_IN_A_ROW * 5;
pub static HEURISTIC_FREE_FOUR_IN_A_ROW: i32 = HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW * 5;
pub static HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW: i32 = HEURISTIC_PREVENT_CAPTURE_STONE * 5;
pub static HEURISTIC_PREVENT_CAPTURE_STONE: i32 = HEURISTIC_FREE_THREE_IN_A_ROW * 5;
pub static HEURISTIC_FREE_THREE_IN_A_ROW: i32 = _HEURISTIC_FOUR_IN_A_ROW_WITH_ONE_BLOCKER * 5;
pub static _HEURISTIC_FOUR_IN_A_ROW_WITH_ONE_BLOCKER: i32 = HEURISTIC_SIMPLE_BLOCK_TWO_IN_A_ROW * 5;
pub static HEURISTIC_SIMPLE_BLOCK_TWO_IN_A_ROW: i32 = HEURISTIC_FREE_TWO_IN_A_ROW * 5;
pub static HEURISTIC_FREE_TWO_IN_A_ROW: i32 = 10;
pub static HEURISTIC_POSSIBLE_AXE_DEVELOPMENT: i32 = 1;
// TODO: This heuristic value is not sure
pub static HEURISTIC_CAPTURING_ONE_STONE: i32 = 15;

//ratio of pattern
// index 0 for no blocker or 2 blocked
// index 1 for 1 blocker or 1 blocked
pub static HEURISTIC_PATTERN: [[i32; 3]; 10] = [
    [
        HEURISTIC_FIVE_IN_A_ROW,
        HEURISTIC_FIVE_IN_A_ROW,
        HEURISTIC_FIVE_IN_A_ROW,
    ], // five XXXXX...
    [
        HEURISTIC_FREE_FOUR_IN_A_ROW,
        HEURISTIC_FREE_FOUR_IN_A_ROW / 5,
        0,
    ], // four .XXXX...
    [HEURISTIC_FREE_FOUR_IN_A_ROW, 0, 0], // split four 3 .XXX.X..
    [HEURISTIC_FREE_FOUR_IN_A_ROW, 800, 0], // split four 1 .X.XXX..
    [HEURISTIC_FREE_FOUR_IN_A_ROW, 600, 0], // split four 2 .XX.XX..
    [
        HEURISTIC_FREE_THREE_IN_A_ROW,
        HEURISTIC_PREVENT_CAPTURE_STONE,
        HEURISTIC_PREVENT_CAPTURE_STONE / 3,
    ], // three  .XXX....
    [
        HEURISTIC_FREE_THREE_IN_A_ROW,
        HEURISTIC_PREVENT_CAPTURE_STONE,
        HEURISTIC_PREVENT_CAPTURE_STONE / 3,
    ], // three2  .XXX....
    [HEURISTIC_FREE_THREE_IN_A_ROW, 0, 0], // split three .X.XX...
    [HEURISTIC_FREE_THREE_IN_A_ROW, 0, 0], // split three rev .XX.X...
    [
        HEURISTIC_FREE_TWO_IN_A_ROW,
        -HEURISTIC_PREVENT_CAPTURE_STONE,
        0,
    ], // double 	.XX.....
];

pub static HEURISTIC_BLOCKER: [[i32; 3]; 10] = [
    [0, 0, 0],                                        // five XXXXX...
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW],     // four .XXXX...
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW],     // split four 3 .XXX.X..
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW],     // split four 1 .X.XXX..
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW],     // split four 2 .XX.XX..
    [0, HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW, 0],    // three  .XXX....
    [0, HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW, 0],    // three2  .XXX....
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW / 2], // split three .X.XX...
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW / 2], // split three rev .XX.X...
    [0, HEURISTIC_SIMPLE_BLOCK_TWO_IN_A_ROW, 0],      // double 	.XX.....
];
