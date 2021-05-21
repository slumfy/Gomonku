use crate::data_struct::State;
use crate::data_struct::Transpotablenode;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref TT_STATES: RwLock::<HashMap<u64, Transpotablenode>> = RwLock::new(HashMap::new());
}
pub fn tt_insert(state: &State, depth: i32) {
	let mut hash = DefaultHasher::new();
    state.bitboards.hash(&mut hash);
    let state_hash: u64 = hash.finish();
	let new_table_node = Transpotablenode {
        hash: state_hash,
        depth: depth,
        value: state.heuristic,
    };
	TT_STATES.write().unwrap().insert(state_hash, new_table_node);
}

pub fn tt_search(state: &State, depth: i32) {
	let mut hash = DefaultHasher::new();
    state.bitboards.hash(&mut hash);
    let state_hash: u64 = hash.finish();
	if TT_STATES.read().unwrap().contains_key(&state_hash) {
		println!("FOUND");
	}
}