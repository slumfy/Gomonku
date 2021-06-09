// HEURISTIC IMMUABLE VALUE
pub static MAX_VALUE: i64 = i64::MAX;
pub static MIN_VALUE: i64 = i64::MIN;
pub static HEURISTIC_MULTIPLIER: i64 = 10;

// PATTERNS  AND BLOCKERS VALUE
pub static CAPTURE_TEN_STONE: i64 = UNBLOCKABLE_FIVE_IN_A_ROW * HEURISTIC_MULTIPLIER;

pub static UNBLOCKABLE_FIVE_IN_A_ROW: i64 = FIVE_IN_A_ROW * HEURISTIC_MULTIPLIER;

pub static FIVE_IN_A_ROW: i64 = DOUBLE_BLOCK_FOUR_IN_A_ROW * HEURISTIC_MULTIPLIER;

pub static DOUBLE_BLOCK_FOUR_IN_A_ROW: i64 = FREE_FOUR_IN_A_ROW * HEURISTIC_MULTIPLIER;

pub static FREE_FOUR_IN_A_ROW: i64 = FOUR_IN_A_ROW_WITH_ONE_BLOCKER * HEURISTIC_MULTIPLIER;

pub static FOUR_IN_A_ROW_WITH_ONE_BLOCKER: i64 = SIMPLE_BLOCK_THREE_IN_A_ROW * HEURISTIC_MULTIPLIER;

pub static SIMPLE_BLOCK_THREE_IN_A_ROW: i64 = CAPTURE_STONE * HEURISTIC_MULTIPLIER;

pub static CAPTURE_STONE: i64 = FREE_THREE_IN_A_ROW * HEURISTIC_MULTIPLIER;

pub static FREE_THREE_IN_A_ROW: i64 = SIMPLE_BLOCK_TWO_IN_A_ROW * HEURISTIC_MULTIPLIER;

pub static SIMPLE_BLOCK_TWO_IN_A_ROW: i64 = FREE_TWO_IN_A_ROW * HEURISTIC_MULTIPLIER;

pub static FREE_TWO_IN_A_ROW: i64 = POSSIBLE_AXE_DEVELOPMENT * HEURISTIC_MULTIPLIER;

pub static POSSIBLE_AXE_DEVELOPMENT: i64 = 1;

// Combinative heuristic blocker
pub static SIMPLE_BLOCK_THREE_AND_TWO: i64 = FREE_THREE_IN_A_ROW * 5;
pub static BLOCK_A_DOUBLE_THREE: i64 = -(SIMPLE_BLOCK_THREE_IN_A_ROW * 2);
pub static DOUBLE_BLOCK_SPLIT_THREE_IN_A_ROW: i64 = SIMPLE_BLOCK_THREE_IN_A_ROW * 5;

// Combinative heuristic blocker
pub static FOUR_AND_OPEN_THREE: i64 = FREE_FOUR_IN_A_ROW / 2;

//ratio of pattern
// index 0 for no blocker or 2 blocked
// index 1 for 1 blocker or 1 blocked
pub static PATTERN: [[i64; 3]; 9] = [
    [FIVE_IN_A_ROW, FIVE_IN_A_ROW, FIVE_IN_A_ROW], // five XXXXX...
    [FREE_FOUR_IN_A_ROW, 0, 0],                    // four .XXXX...
    [FREE_THREE_IN_A_ROW / 3, 0, 0],               // split four 3 .XXX.X..
    [FREE_THREE_IN_A_ROW / 3, 0, 0],               // split four 1 .X.XXX..
    [FREE_THREE_IN_A_ROW / 3, 0, 0],               // split four 2 .XX.XX..
    [FREE_THREE_IN_A_ROW, CAPTURE_STONE, CAPTURE_STONE / 3], // three  .XXX....
    [FREE_THREE_IN_A_ROW / 2, 0, 0],               // split three .X.XX...
    [FREE_THREE_IN_A_ROW / 2, 0, 0],               // split three rev .XX.X...
    [FREE_TWO_IN_A_ROW, -CAPTURE_STONE, 0],        // double 	.XX.....
];

pub static BLOCKER: [[i64; 3]; 9] = [
    [0, 0, 0],                                 // five XXXXX...
    [0, 0, DOUBLE_BLOCK_FOUR_IN_A_ROW],        // four .XXXX...
    [0, 0, DOUBLE_BLOCK_FOUR_IN_A_ROW],        // split four 3 .XXX.X..
    [0, 0, DOUBLE_BLOCK_FOUR_IN_A_ROW],        // split four 1 .X.XXX..
    [0, 0, DOUBLE_BLOCK_FOUR_IN_A_ROW],        // split four 2 .XX.XX..
    [0, SIMPLE_BLOCK_THREE_IN_A_ROW, 0],       // three  .XXX....
    [0, 0, DOUBLE_BLOCK_SPLIT_THREE_IN_A_ROW], // split three .X.XX...
    [0, 0, DOUBLE_BLOCK_SPLIT_THREE_IN_A_ROW], // split three rev .XX.X...
    [0, SIMPLE_BLOCK_TWO_IN_A_ROW, 0],         // double 	.XX.....
];

pub fn exponential_heuristic_capture_stone_calculator(opponent_stone_captured: i8) -> i64 {
    let mut multiplier = opponent_stone_captured / 2;
    let mut value = CAPTURE_STONE;
    if opponent_stone_captured >= 8 {
        return CAPTURE_TEN_STONE;
    }
    while multiplier > 1 {
        value = value * HEURISTIC_MULTIPLIER;
        multiplier -= 1;
    }
    // return CAPTURE_STONE;
	return value;
}
