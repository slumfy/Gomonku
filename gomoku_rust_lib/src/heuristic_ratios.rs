
pub static DEVELOPMENT_RATIO_DIVISER: i32 = 2;
//weight of potential capture pos
pub static CAPTURING_POS_SCORE: i32 = 100;
//weight of potential capturable pos
pub static CAPTURABLE_POS_SCORE: i32 = 200;
//multiplier for nb of stone captured
pub static CAPTURING_COUNT_RATIO_MULTIPLIER: i32 = 50;



// simulate inf for heuristic value
pub static HEURISTIC_MAX_VALUE: i32 = 100000;
pub static HEURISTIC_MIN_VALUE: i32 = -100000;

// blocker/pattern multiplier for better move weight
pub static PATTERN_MULTIPLIER: i32 = 2;
pub static BLOCKER_MULTIPLIER: i32 = 2;

//ratio of pattern
pub static HEURISTIC_PATTERN: [[i32;2]; 9] = [
    [1000,500],	// five XXXXX...
    [900,90],		// four .XXXX...
    [800,80],		// split four 3 .XXX.X..
    [800,80],		// split four 1 .X.XXX..
    [600,60],		// split four 2 .XX.XX..
    [500,50],		// three  .XXX....
    [300,30],		// split three .X.XX...
    [300,30],		// split three rev .XX.X...
	[100,10], 	// double 	.XX.....
];


//HEURISTIC ORDERING
// *CAPTURING
// *CAPTURABLE
// *PATTERN
// *EAT_STONE
// *FREE_SPACE
// *?
// *who is close to win.