#[path = "check.rs"] mod check;
#[path = "state.rs"] mod state;

pub fn heuristic(board: &mut Vec<Vec<i32>>, state State) -> i32 {
	// score += alignement()
	// score += alignement_win()
	// score += _free_flank()
	// score += combinations()
	// score += patern()
} 