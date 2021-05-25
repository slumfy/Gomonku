/// Bitboards representation of goban
#[derive(Copy, Clone, Hash, Debug)]
pub struct Bitboards {
    pub white_board: [u64; 6],
    pub black_board: [u64; 6],
}

/// Transpo table Node
pub struct Transpotablenode {
    pub hash: u64,
    pub depth: i32,
    pub value: i64,
}

/// State struct corresponding to an instant board value for a given player.
#[derive(Clone, Debug)]
pub struct State {
    pub bitboards: Bitboards,
    pub available_move: Vec<State>,
    pub current_player: i8,
    pub total_white_captured_stone: i8,
    pub total_black_captured_stone: i8,
    pub parent_state_white_captured_stone: i8,
    pub parent_state_black_captured_stone: i8,
    pub white_move_to_win: i8,
    pub black_move_to_win: i8,
    pub heuristic: i64,
    pub is_playable: i8,
    pub win_state: (usize, i8),
    pub current_move_pos: usize,
    pub board_info: BoardStateInfo,
    pub axes: [[u16; 4]; 2],
<<<<<<< HEAD
	pub max_eat_next_move : i8,
=======
    pub stone_threaten: u32,
>>>>>>> d0608f0de0493e9458851ae51bfcdc5f959612b5
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
    pub nb_move_to_win: i8,
    pub axe_free_value: [bool; 4],
    pub pattern_axe: [(usize, usize); 4],
    pub blocker_axe: [(usize, usize); 4],
}
