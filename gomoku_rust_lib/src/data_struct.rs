/// Bitboards representation of goban
#[derive(Copy, Clone, Hash, Debug)]
pub struct Bitboards {
    pub white_board: [u64; 6],
    pub black_board: [u64; 6],
}

/// Transpo table Node
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Transpotablenode {
    pub hash: u64,
    pub depth: i32,
    pub value: i64,
    pub flag: Flag,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Flag {
    EXACT,
    LOWERBOUND,
    UPPERBOUND,
}

/// State struct corresponding to an instant board value for a given player.
#[derive(Clone, Debug)]
pub struct State {
    pub bitboards: Bitboards,
    pub available_move: Vec<State>,
    pub current_player: i8,
    pub total_white_captured_stone: i8,
    pub total_black_captured_stone: i8,
    pub all_depth_white_captured_stone_value: i64,
    pub all_depth_black_captured_stone_value: i64,
    pub heuristic: i64,
    pub is_playable: i8,
    pub win_state: (usize, i8),
    pub current_move_pos: usize,
    pub board_info: BoardStateInfo,
    pub axes: [[u16; 4]; 2],
    pub max_eat_next_move: i8,
}

/// Board state information for computing heuristic and validate move

#[derive(Copy, Clone, Debug)]
pub struct BoardStateInfo {
    pub player: i8,
    pub is_wrong_move: i8,
    pub stone_captured: i8,
    pub capturable: bool,
    pub capturing: bool,
    pub captured_pattern_blocking_value: i64,
    pub pattern_value: i64,
    pub blocker_value: i64,
    pub is_winning: (usize, i8),
    pub axe_free_value: [bool; 4],
    pub pattern_axe: [(usize, usize); 4],
    pub blocker_axe: [(usize, usize); 4],
}
