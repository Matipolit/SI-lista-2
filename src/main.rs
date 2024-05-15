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
    let finish: Option<(DecisionTreeNode, u32, u32)> = match function_str {
        "minimax" => minimax(
            &mut node,
            max_depth,
            heuristics,
            rounds_limit,
            &log_level,
        ),
        _ => alfa_beta(
            &mut node,
            max_depth,
            heuristics,
            rounds_limit,
            &log_level,
        ),
    };
    let elapsed = time.elapsed();

    println!("{} game finished", test_name);
    match finish {
        Some((node, skipped, rounds)) => {
            println!("Took {} rounds", &rounds + skipped);
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
        power: 1.05,
        rng: rng.clone(),
    };

    let mut heuristic_proximity_hybrid = heuristics::HeuristicProximityWithSingle {
        multi_power: 0.87,
        single_power: 2.6,
        rng: rng.clone(),
    };

    match parsed_board {
        Ok(board) => {
            let first_node =
                DecisionTreeNode::new(board, halma::GameState::Start(halma::Player::White));
            let mut now = Instant::now();
            let mut new_proximity = heuristic_proximity.clone();
            /*
            run_test(
                &function,
                (&mut heuristic_random, &mut new_proximity),
                &mut now,
                first_node.clone(),
                max_depth,
                &log_level,
                None,
                "Random (white) vs proximity (black)".to_owned(),
            );
            run_test(
                &function,
                (&mut heuristic_proximity, &mut heuristic_proximity_hybrid),
                &mut now,
                first_node.clone(),
                max_depth,
                &log_level,
                None,
                "Proximity (white) vs proximity-hybrid (black)".to_owned(),
            );
            */
            run_test(
                &function,
                (&mut heuristic_proximity_hybrid, &mut heuristic_proximity),
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
