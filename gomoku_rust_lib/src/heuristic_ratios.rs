pub static HEURISTIC_MAX_VALUE: i64 = i64::MAX;
pub static HEURISTIC_MIN_VALUE: i64 = i64::MIN + 1;

pub static HEURISTIC_PREVENT_OPPONENT_WIN_BY_CAPTURE: i64 = HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW * 5;
pub static HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW: i64 = HEURISTIC_FIVE_IN_A_ROW * 5;
pub static HEURISTIC_FIVE_IN_A_ROW: i64 = HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW * 5;
pub static HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW: i64 = HEURISTIC_FREE_FOUR_IN_A_ROW * 5;
pub static HEURISTIC_FREE_FOUR_IN_A_ROW: i64 = HEURISTIC_DOUBLE_BLOCK_SPLIT_THREE_IN_A_ROW * 5;
pub static HEURISTIC_DOUBLE_BLOCK_SPLIT_THREE_IN_A_ROW: i64 =
    HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW * 5;
pub static HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW: i64 = HEURISTIC_PREVENT_CAPTURE_STONE * 5;
pub static HEURISTIC_PREVENT_CAPTURE_STONE: i64 = HEURISTIC_FREE_THREE_IN_A_ROW * 5;
pub static HEURISTIC_FREE_THREE_IN_A_ROW: i64 = HEURISTIC_SIMPLE_BLOCK_TWO_IN_A_ROW * 5;
pub static HEURISTIC_SIMPLE_BLOCK_TWO_IN_A_ROW: i64 = HEURISTIC_FREE_TWO_IN_A_ROW * 5;
pub static HEURISTIC_FREE_TWO_IN_A_ROW: i64 = HEURISTIC_FOUR_IN_A_ROW_WITH_ONE_BLOCKER * 5;
pub static HEURISTIC_FOUR_IN_A_ROW_WITH_ONE_BLOCKER: i64 = 10;
pub static HEURISTIC_THREE_IN_A_ROW_ONE_BLOCKER: i64 = HEURISTIC_POSSIBLE_AXE_DEVELOPMENT * 5;
pub static HEURISTIC_POSSIBLE_AXE_DEVELOPMENT: i64 = 1;

// Combinative heuristic blocker
pub static HEURISTIC_SIMPLE_BLOCK_THREE_AND_TWO: i64 = HEURISTIC_FREE_THREE_IN_A_ROW * 5;
pub static HEURISTIC_BLOCK_A_DOUBLE_THREE: i64 = -(HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW * 2);
// Combinative heuristic blocker
pub static HEURISTIC_FOUR_AND_OPEN_THREE: i64 = HEURISTIC_FREE_FOUR_IN_A_ROW / 2;

//ratio of pattern
// index 0 for no blocker or 2 blocked
// index 1 for 1 blocker or 1 blocked
pub static HEURISTIC_PATTERN: [[i64; 3]; 9] = [
    [
        HEURISTIC_FIVE_IN_A_ROW,
        HEURISTIC_FIVE_IN_A_ROW,
        HEURISTIC_FIVE_IN_A_ROW,
    ], // five XXXXX...
    [HEURISTIC_FREE_FOUR_IN_A_ROW, 0, 0],      // four .XXXX...
    [HEURISTIC_FREE_THREE_IN_A_ROW / 3, 0, 0], // split four 3 .XXX.X..
    [HEURISTIC_FREE_THREE_IN_A_ROW / 3, 0, 0], // split four 1 .X.XXX..
    [HEURISTIC_FREE_THREE_IN_A_ROW / 3, 0, 0], // split four 2 .XX.XX..
    [
        HEURISTIC_FREE_THREE_IN_A_ROW,
        HEURISTIC_PREVENT_CAPTURE_STONE,
        HEURISTIC_PREVENT_CAPTURE_STONE / 3,
    ], // three  .XXX....
    [HEURISTIC_FREE_THREE_IN_A_ROW / 2, 0, 0], // split three .X.XX...
    [HEURISTIC_FREE_THREE_IN_A_ROW / 2, 0, 0], // split three rev .XX.X...
    [
        HEURISTIC_FREE_TWO_IN_A_ROW,
        -HEURISTIC_PREVENT_CAPTURE_STONE,
        0,
    ], // double 	.XX.....
];

pub static HEURISTIC_BLOCKER: [[i64; 3]; 9] = [
    [0, 0, 0],                                           // five XXXXX...
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW],        // four .XXXX...
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW],        // split four 3 .XXX.X..
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW],        // split four 1 .X.XXX..
    [0, 0, HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW],        // split four 2 .XX.XX..
    [0, HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW, 0],       // three  .XXX....
    [0, 0, HEURISTIC_DOUBLE_BLOCK_SPLIT_THREE_IN_A_ROW], // split three .X.XX...
    [0, 0, HEURISTIC_DOUBLE_BLOCK_SPLIT_THREE_IN_A_ROW], // split three rev .XX.X...
    [0, HEURISTIC_SIMPLE_BLOCK_TWO_IN_A_ROW, 0],         // double 	.XX.....
];

pub fn exponential_heuristic_prevent_capture_stone_calculator(opponent_stone_captured: i8) -> i64 {
    // let mut multiplier = opponent_stone_captured / 2;
    // let mut value = HEURISTIC_PREVENT_CAPTURE_STONE;
    if opponent_stone_captured >= 8 {
        return HEURISTIC_PREVENT_OPPONENT_WIN_BY_CAPTURE;
    }
    // while multiplier > 1 {
    //     value = value * 2;
    //     multiplier -= 1;
    // }
    return HEURISTIC_PREVENT_CAPTURE_STONE;
}
