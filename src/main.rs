mod decision_tree;
mod halma;
mod heuristics;
mod minimax_new;
//mod minmax;

use halma::board_from_str;

use minimax_new::LogLevel;

use std::{env, fs, time::Instant};

use crate::{
    decision_tree::DecisionTreeNode,
    minimax_new::{alfa_beta, minimax},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Wrong number of arguments! Usage: cargo run --release -- <board filename> <log level [none, round, all]> <function [minimax, alfabeta]> <max depth>");
    }
    let board_file = &args[1];
    let board_string = fs::read_to_string(board_file)
        .expect(format!("Could not read board file: {}", board_file).as_str());

    let parsed_board = board_from_str(&board_string);

    let log_level: LogLevel = match args[2].as_str() {
        "none" => LogLevel::None,
        "all" => LogLevel::All,
        _ => LogLevel::RoundNum,
    };

    let function = &args[3];

    let max_depth: u32 = args[4].parse().unwrap();

    let rng = rand::thread_rng();
    let mut heuristic_random = heuristics::HeuristicRandom { rng: rng.clone() };

    let mut heuristic_proximity = heuristics::HeuristicProximity { power: 1.05 };

    let mut heuristic_proximity_hybrid = heuristics::HeuristicProximityWithSingle {
        multi_power: 0.87,
        single_power: 2.6,
    };

    match parsed_board {
        Ok(board) => {
            /*
            let mut test_white_node = DecisionTreeNode {
                board,
                game_state: halma::GameState::Start,
                children: vec![],
                selected: false,
            };
            test_white_node.generate_children(halma::Player::White);
            for child in test_white_node.children {
                println!("{}", board_to_string(&child.board));
            }
            */
            /*
            let mut graph = Graph::<DecisionTreeNode, u64>::with_capacity(1000, 1000);
            let mut test_tree = generate_tree(Some(1), board, &mut Some(&mut graph));
            //println!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

            println!("In total: {} children", graph.node_count());
            */
            // for (index, child) in test_tree.children.into_iter().enumerate() {
            //     println!("Child {}", index);
            //     display_board(&child.board)
            // }

            let first_node =
                DecisionTreeNode::new(board, halma::GameState::Start(halma::Player::Black));
            let mut now = Instant::now();
            let mut node = first_node.clone();

            let finish_random = match function.as_str() {
                "minimax" => minimax(
                    &mut node,
                    max_depth,
                    &mut heuristic_random,
                    None,
                    &log_level,
                    halma::Player::Black,
                ),
                _ => alfa_beta(
                    &mut node,
                    max_depth,
                    &mut heuristic_random,
                    None,
                    &log_level,
                    halma::Player::Black,
                ),
            };
            let elapsed = now.elapsed();

            println!("Random Heuristic game finished");
            match finish_random {
                Some((node, skipped, rounds)) => {
                    println!("Child at depth: {}\n{}", &skipped, &node);
                    println!("Took {} rounds", &rounds);
                }
                None => println!("Finished without winner"),
            }
            println!("Took {:.2?} seconds", elapsed.as_secs_f32());

            now = Instant::now();
            node = first_node.clone();

            let finish_prox = match function.as_str() {
                "minimax" => minimax(
                    &mut node,
                    max_depth,
                    &mut heuristic_proximity,
                    None,
                    &log_level,
                    halma::Player::Black,
                ),
                _ => alfa_beta(
                    &mut node,
                    max_depth,
                    &mut heuristic_proximity,
                    None,
                    &log_level,
                    halma::Player::Black,
                ),
            };
            let elapsed = now.elapsed();

            println!("Proximity Heuristic game finished");
            match finish_prox {
                Some((node, skipped, rounds)) => {
                    println!("Child at depth: {}\n{}", &skipped, &node);
                    println!("Took {} rounds", &rounds);
                }
                None => println!("Finished without winner"),
            }
            println!("Took {:.2?} seconds", elapsed.as_secs_f32());

            now = Instant::now();
            node = first_node.clone();

            let finish_prox_hybrid = match function.as_str() {
                "minimax" => minimax(
                    &mut node,
                    max_depth,
                    &mut heuristic_proximity_hybrid,
                    None,
                    &log_level,
                    halma::Player::Black,
                ),
                _ => alfa_beta(
                    &mut node,
                    max_depth,
                    &mut heuristic_proximity_hybrid,
                    None,
                    &log_level,
                    halma::Player::Black,
                ),
            };
            let elapsed = now.elapsed();

            println!("Proximity Heuristic game finished");
            match finish_prox_hybrid {
                Some((node, skipped, rounds)) => {
                    println!("Child at depth: {}\n{}", &skipped, &node);
                    println!("Took {} rounds", &rounds);
                }
                None => println!("Finished without winner"),
            }
            println!("Took {:.2?} seconds", elapsed.as_secs_f32());
        }
        Err(error) => println!("Could not parse board: {}", error),
    }
}
