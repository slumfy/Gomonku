// Search algorithm global vars
pub static DEEP: i32 = 1;

// Capturing stone count
pub static mut WHITE_CAPTURED_STONE: i8 = 0;
pub static mut BLACK_CAPTURED_STONE: i8 = 0;

// Player color
pub static PLAYER_WHITE_NB: i8 = 1;
pub static PLAYER_BLACK_NB: i8 = -1;

// Heuristic global var
pub static HEURISTIC_MAX_VALUE: i32 = 1000;
pub static HEURISTIC_MIN_VALUE: i32 = -1000;

// Axes global var
pub static AXE_MOUVEMENT_VALUE: [usize; 4] = [20, 19, 18, 1];

// BOARD VAR

pub static BOARD_MIN_LIMITS: usize = 0;
pub static BOARD_MAX_LIMITS: usize = 360;

// Move validation check
pub static VALID_MOVE: i8 = 0;
pub static OUT_OF_BOARD_MOVE: i8 = -1;
pub static OVERLAPPING_STONE_MOVE: i8 = -2;
