// use crate::algorithms;
// use crate::bitboard_operations::apply_bit;
// use crate::data_struct;
// use crate::data_struct::Bitboards;
// use crate::global_var;
// use crate::heuristic_ratios;
// use crate::state;
// use std::time::Instant;

// pub fn test_negamax_benchmark() {
    // let mut depth = 1;

    // let mut bitboards: Bitboards = Bitboards {
    //     white_board: [0, 0, 0, 0, 0, 0],
    //     black_board: [0, 0, 0, 0, 0, 0],
    // };
    // let pos = 180;
    // apply_bit(&mut bitboards, 0, global_var::PLAYER_WHITE_NB);
    // apply_bit(&mut bitboards, 1, global_var::PLAYER_BLACK_NB);
    // apply_bit(&mut bitboards, 19, global_var::PLAYER_WHITE_NB);
    // apply_bit(&mut bitboards, 120, global_var::PLAYER_BLACK_NB);
    // apply_bit(&mut bitboards, 181, global_var::PLAYER_WHITE_NB);
    // apply_bit(&mut bitboards, 25, global_var::PLAYER_BLACK_NB);
    // apply_bit(&mut bitboards, pos, global_var::PLAYER_WHITE_NB);

    // let mut state: data_struct::State = state::create_new_state(
    //     &mut bitboards,
    //     global_var::PLAYER_WHITE_NB,
    //     pos,
    //     0,
    //     0,
    //     (0, 0),
    //     0,
    // );

    // // Try with depth 1
    // println!("DEPTH {} :", depth);
    // let start_time = Instant::now();
    // algorithms::negamax(
    //     &mut state,
    //     depth,
    //     heuristic_ratios::HEURISTIC_MIN_VALUE,
    //     heuristic_ratios::HEURISTIC_MAX_VALUE,
    //     1,
    // );
    // let ai_move = algorithms::return_move(&mut state);
    // let end_time = Instant::now();
    // println!("time to process {:?}", end_time.duration_since(start_time));
    // println!();

    // // Try with depth 2
    // depth += 1;

    // println!("DEPTH {} :", depth);
    // let start_time = Instant::now();
    // algorithms::negamax(
    //     &mut state,
    //     depth,
    //     heuristic_ratios::HEURISTIC_MIN_VALUE,
    //     heuristic_ratios::HEURISTIC_MAX_VALUE,
    //     1,
    // );
    // let ai_move = algorithms::return_move(&mut state);
    // let end_time = Instant::now();
    // println!("time to process {:?}", end_time.duration_since(start_time));
    // println!();

    // // Try with depth 3
    // depth += 1;

    // println!("DEPTH {} :", depth);
    // let start_time = Instant::now();
    // algorithms::negamax(
    //     &mut state,
    //     depth,
    //     heuristic_ratios::HEURISTIC_MIN_VALUE,
    //     heuristic_ratios::HEURISTIC_MAX_VALUE,
    //     1,
    // );
    // let ai_move = algorithms::return_move(&mut state);
    // let end_time = Instant::now();
    // println!("time to process {:?}", end_time.duration_since(start_time));
    // println!();

    // // Try with depth 4
    // depth += 1;
    // println!("DEPTH {} :", depth);
    // let start_time = Instant::now();
    // algorithms::negamax(
    //     &mut state,
    //     depth,
    //     heuristic_ratios::HEURISTIC_MIN_VALUE,
    //     heuristic_ratios::HEURISTIC_MAX_VALUE,
    //     global_var::PLAYER_BLACK_NB,
    // );
    // let ai_move = algorithms::return_move(&mut state);
    // let end_time = Instant::now();
    // println!("time to process {:?}", end_time.duration_since(start_time));
    // println!();

    // // Try with depth 5
    // depth += 1;
    // println!("DEPTH {} :", depth);
    // let start_time = Instant::now();
    // algorithms::negamax(
    //     &mut state,
    //     depth,
    //     heuristic_ratios::HEURISTIC_MIN_VALUE,
    //     heuristic_ratios::HEURISTIC_MAX_VALUE,
    //     global_var::PLAYER_BLACK_NB,
    // );
    // let ai_move = algorithms::return_move(&mut state);
    // let end_time = Instant::now();
    // println!("time to process {:?}", end_time.duration_since(start_time));
    // println!();

    // // Try with depth 6
    // depth += 1;
    // println!("DEPTH {} :", depth);
    // let start_time = Instant::now();
    // algorithms::negamax(
    //     &mut state,
    //     depth,
    //     heuristic_ratios::HEURISTIC_MIN_VALUE,
    //     heuristic_ratios::HEURISTIC_MAX_VALUE,
    //     global_var::PLAYER_BLACK_NB,
    // );
    // let ai_move = algorithms::return_move(&mut state);
    // let end_time = Instant::now();
    // println!("time to process {:?}", end_time.duration_since(start_time));
    // println!();
// }
