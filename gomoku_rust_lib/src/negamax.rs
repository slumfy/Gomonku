use crate::state::State;
use crate::state::create_child;

pub fn negamax(mut state:&mut State, depth: i32, color: i32) -> ((i32,i32),i32) {
	if depth != 0 { 
		state.available_move = create_child(&mut state);
	}
	println!("current state: {:?}", state.current_move);
	if depth == 0 || state.available_move.len() == 0 {
		return ((state.current_move),state.heuristic * color);
	}
	let mut value:((i32,i32),i32) = ((-1,-1),-1000);
	for child in 0..state.available_move.len() {
		value = max(value, negamax(&mut state.available_move[child], depth - 1, -color))
	}
	return value;
}

fn max(value: ((i32,i32),i32),childvalue: ((i32,i32),i32)) -> ((i32,i32),i32) {
	if value.1 > childvalue.1 {
		return value;
	}
	return childvalue;
	}
// function negamax(node, depth, color) is
//     if depth = 0 or node is a terminal node then
//         return color × the heuristic value of node
//     value := −∞
//     for each child of node do
//         value := max(value, −negamax(child, depth − 1, −color))
//     return value


//example of negamax alpha beta pruning 
// function negamax(node, depth, α, β, color)
// 	is if depth = 0 or node is a terminal node then
// 		return color × the heuristic value of node

// 	childNodes := generateMoves(node)
// 	childNodes := orderMoves(childNodes)
// 	value := −∞
// 	foreach child in childNodes do
// 		value := max(value, −negamax(child, depth − 1, −β, −α, −color))
// 		α := max(α, value)
// 		if α ≥ β then
// 			break (* cut-off *)
// 	return value Negamax
	
// first call
// negamax(rootNode, depth, −∞, +∞, 1) 


//negascout

// function pvs(node, depth, α, β, color) is
//     if depth = 0 or node is a terminal node then
//         return color × the heuristic value of node
//     for each child of node do
//         if child is first child then
//             score := −pvs(child, depth − 1, −β, −α, −color)
//         else
//             score := −pvs(child, depth − 1, −α − 1, −α, −color) (* search with a null window *)
//             if α < score < β then
//                 score := −pvs(child, depth − 1, −β, −score, −color) (* if it failed high, do a full re-search *)
//         α := max(α, score)
//         if α ≥ β then
//             break (* beta cut-off *)
//     return α