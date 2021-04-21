/// Bitboards representation of goban
#[derive(Copy, Clone, Hash)]
pub struct Bitboards {
    pub white_board: [u64; 6],
    pub black_board: [u64; 6],
}

/// State struct corresponding to an instant board value for a given player.
pub struct State {
    pub bitboards: Bitboards,
    pub available_move: Vec<State>,
    pub current_player: i8,
    pub white_captured_stone: i8,
    pub black_captured_stone: i8,
    pub white_move_to_win: i8,
    pub black_move_to_win: i8,
    pub heuristic: i32,
    pub is_playable: i8,
    pub win_state: (usize, i8),
    pub bit_current_move_pos: usize,
	pub board_info: BoardStateInfo
}

/// Board state information for computing heuristic and validate move

#[derive(Copy, Clone, Debug)]
pub struct BoardStateInfo {
	pub player: i8,
    pub is_wrong_move: i8,
    pub stone_captured: i8,
    pub capturable: bool,
    pub capturing: bool,
    pub pattern_value: i32,
    pub blocker_value: i32,
    pub is_winning: (usize, i8),
    pub nb_move_to_win: i8,
	pub	pattern_axe: [(usize, usize); 4],
	pub	blocker_axe: [(usize, usize); 4],
}