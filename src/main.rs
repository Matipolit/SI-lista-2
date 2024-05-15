mod decision_tree;
mod halma;
mod heuristics;
mod minimax;

use halma::board_state_from_str;

use heuristics::Heuristic;
use minimax::LogLevel;

use std::{env, fs, time::Instant};

use crate::{
    decision_tree::DecisionTreeNode,
    minimax::{alfa_beta, minimax},
};

fn run_test<A: Heuristic, B: Heuristic>(
    function_str: &str,
    heuristics: (&mut A, &mut B),
    time: &mut Instant,
    first_node: DecisionTreeNode,
    max_depth: u32,
    log_level: &LogLevel,
    rounds_limit: Option<u32>,
) {
    *time = Instant::now();
    let mut node = first_node.clone();
    let test_name = format!("{} vs {}", heuristics.0.name(), heuristics.1.name());

    println!("Playing {}", test_name);
    let finish: Option<(DecisionTreeNode, u32, u32)> = match function_str {
        "minimax" => minimax(&mut node, max_depth, heuristics, rounds_limit, &log_level),
        _ => alfa_beta(&mut node, max_depth, heuristics, rounds_limit, &log_level),
    };
    let elapsed = time.elapsed();

    println!("\n\n\n\n{} game finished", test_name);
    match finish {
        Some((node, skipped, rounds)) => {
            println!("Took {} rounds", &rounds + skipped);
            println!("{}", node);
        }
        None => println!("Finished without winner"),
    }
    println!("Took {:.2?} seconds", elapsed.as_secs_f32());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Wrong number of arguments! Usage: cargo run --release -- <board filename> <log level [none, round, all]> <function [minimax, alfabeta]> <max depth>");
    }
    let board_file = &args[1];
    let board_string = fs::read_to_string(board_file)
        .expect(format!("Could not read board file: {}", board_file).as_str());

    let parsed_board = board_state_from_str(&board_string);

    let log_level: LogLevel = match args[2].as_str() {
        "none" => LogLevel::None,
        "all" => LogLevel::All,
        _ => LogLevel::RoundNum,
    };

    let function = &args[3];

    let max_depth: u32 = args[4].parse().unwrap();

    let rng = rand::thread_rng();
    let mut heuristic_random = heuristics::HeuristicRandom { rng: rng.clone() };

    let mut heuristic_proximity = heuristics::HeuristicProximity {
        power: 1.08,
        rng: rng.clone(),
    };

    let mut heuristic_leading = heuristics::HeuristicProximityWithSingle {
        multi_power: 0.9,
        single_power: 2.5,
        rng: rng.clone(),
    };

    let mut heuristic_discourage = heuristics::HeuristicDiscourageStart {
        other_power: 1.1,
        discourage_power: 1.0,
    };

    let mut heuristic_complex = heuristics::HeuristicComplex {
        single_power: 2.5,
        multi_power: 0.9,
        discourage_power: 1.0,
    };

    //heuristics::print_new_table();

    match parsed_board {
        Ok(board) => {
            let first_node =
                DecisionTreeNode::new(board, halma::GameState::Start(halma::Player::Black));
            let mut now = Instant::now();
            // run_test(
            //     &function,
            //     (&mut heuristic_random, &mut heuristic_leading),
            //     &mut now,
            //     first_node.clone(),
            //     max_depth,
            //     &log_level,
            //     None,
            // );
            // let mut new_proximity = heuristic_proximity.clone();

            // run_test(
            //     &function,
            //     (&mut heuristic_random, &mut heuristic_discourage),
            //     &mut now,
            //     first_node.clone(),
            //     max_depth,
            //     &log_level,
            //     None,
            // );

            // run_test(
            //     &function,
            //     (&mut heuristic_proximity, &mut heuristic_proximity_hybrid),
            //     &mut now,
            //     first_node.clone(),
            //     max_depth,
            //     &log_level,
            //     None,
            // );
            // run_test(
            //     &function,
            //     (&mut heuristic_proximity, &mut heuristic_discourage),
            //     &mut now,
            //     first_node.clone(),
            //     max_depth,
            //     &log_level,
            //     None,
            // );

            // run_test(
            //     &function,
            //     (&mut heuristic_proximity_hybrid, &mut heuristic_discourage),
            //     &mut now,
            //     first_node.clone(),
            //     max_depth,
            //     &log_level,
            //     None,
            // );

            run_test(
                &function,
                (&mut heuristic_leading, &mut heuristic_complex),
                &mut now,
                first_node.clone(),
                max_depth,
                &log_level,
                None,
            );
        }
        Err(error) => println!("Could not parse board: {}", error),
    }
}
