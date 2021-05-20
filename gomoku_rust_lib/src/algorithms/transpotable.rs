use crate::data_struct::State;
use crate::data_struct::Transpotablenode;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub static mut TRANSPOTABLENEGA: Vec<Transpotablenode> = vec![];
pub static mut TRANSPOTABLESCOUT: Vec<Transpotablenode> = vec![];

pub fn transposition_table_push(
    state: &State,
    depth: i32,
    transpo_table: &mut Vec<Transpotablenode>,
) {
    let mut hash = DefaultHasher::new();
    state.bitboards.hash(&mut hash);
    let state_hash: u64 = hash.finish();
    let new_table_node = Transpotablenode {
        hash: state_hash,
        depth: depth,
        value: state.heuristic,
    };
    transpo_table.push(new_table_node);
}

pub unsafe fn transposition_table_search(
    state: &State,
    transpo_table: &Vec<Transpotablenode>,
) -> (bool, i32, i64) {
    let mut hash = DefaultHasher::new();
    state.bitboards.hash(&mut hash);
    let state_hash: u64 = hash.finish();
    let len = transpo_table.len();
    for node in 0..len {
        if transpo_table[node].hash == state_hash {
            return (true, transpo_table[node].depth, transpo_table[node].value);
        }
    }
    return (false, 0, 0);
}
