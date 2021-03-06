pub static HEURISTIC_MAX_VALUE: i32 = i32::MAX;
pub static HEURISTIC_MIN_VALUE: i32 = i32::MIN;

pub static HEURISTIC_CAPTURE_TEN_STONE: i32 = HEURISTIC_MAX_VALUE;
pub static _HEURISTIC_CAPTURE_AN_OPPONENT_FIVE_IN_A_ROW: i32 =
    _HEURISTIC_PREVENT_OPPONENT_WIN_BY_CAPTURE * 5;
pub static _HEURISTIC_PREVENT_OPPONENT_WIN_BY_CAPTURE: i32 = _HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW * 5;
pub static _HEURISTIC_UNBLOCKABLE_FIVE_IN_A_ROW: i32 = _HEURISTIC_FIVE_IN_A_ROW * 5;
pub static _HEURISTIC_FIVE_IN_A_ROW: i32 = _HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW * 5;
pub static _HEURISTIC_DOUBLE_BLOCK_FOUR_IN_A_ROW: i32 = _HEURISTIC_FREE_FOUR_IN_A_ROW * 5;
pub static _HEURISTIC_FREE_FOUR_IN_A_ROW: i32 = _HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW * 5;
pub static _HEURISTIC_SIMPLE_BLOCK_THREE_IN_A_ROW: i32 = _HEURISTIC_PREVENT_CAPTURE_STONE * 5;
pub static _HEURISTIC_PREVENT_CAPTURE_STONE: i32 = _HEURISTIC_FREE_THREE_IN_A_ROW * 5;
pub static _HEURISTIC_FREE_THREE_IN_A_ROW: i32 = _HEURISTIC_FOUR_IN_A_ROW_WITH_ONE_BLOCKER * 5;
pub static _HEURISTIC_FOUR_IN_A_ROW_WITH_ONE_BLOCKER: i32 = _HEURISTIC_SIMPLE_BLOCK_TWO_IN_A_ROW * 5;
pub static _HEURISTIC_SIMPLE_BLOCK_TWO_IN_A_ROW: i32 = _HEURISTIC_FREE_TWO_IN_A_ROW * 5;
pub static _HEURISTIC_FREE_TWO_IN_A_ROW: i32 = 10;
pub static _HEURISTIC_POSSIBLE_AXE_DEVELOPMENT: i32 = 1;

//ratio of pattern
// index 0 for no blocker or 2 blocked
// index 1 for 1 blocker or 1 blocked
pub static HEURISTIC_PATTERN: [[i32; 2]; 10] = [
    [1000, 1000], // five XXXXX...
    [900, 900],   // four .XXXX...
    [800, 800],   // split four 3 .XXX.X..
    [800, 800],   // split four 1 .X.XXX..
    [600, 600],   // split four 2 .XX.XX..
    [500, 500],   // three  .XXX....
	[500, 500],   // three2  .XXX....
    [300, 300],   // split three .X.XX...
    [300, 300],   // split three rev .XX.X...
    [100, 100],   // double 	.XX.....
];
