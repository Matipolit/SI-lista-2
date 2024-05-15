use crate::decision_tree::DecisionTreeNode;
use crate::halma::{GameState, Player};

use crate::heuristics::Heuristic;

#[derive(Clone)]
enum MinMaxResult {
    Eval(f32, usize),
    Leaf(DecisionTreeNode, u32),
}

pub enum LogLevel {
    None,
    RoundNum,
    All,
}

pub fn minimax<A: Heuristic, B: Heuristic>(
    node: &mut DecisionTreeNode,
    max_depth: u32,
    heuristics: (&mut A, &mut B),
    rounds_limit: Option<u32>,
    log_level: &LogLevel,
) -> Option<(DecisionTreeNode, u32, u32)> {
    fn minimax_inner(
        node: &mut DecisionTreeNode,
        current_depth: u32,
        heuristic: &mut impl Heuristic,
        node_index: usize,
        maximizing: bool,
        player: Player,
        round_number: u32,
    ) -> MinMaxResult {
        if current_depth == 0 {
            return MinMaxResult::Eval(
                heuristic.evaluate(&node.board_state, player, round_number),
                node_index,
            );
        }

        let next_player = match node.game_state {
            GameState::Start(player) => player,
            GameState::Moved(player) => player.other(),
            GameState::Won(_) => {
                return MinMaxResult::Leaf(node.clone(), current_depth);
            }
        };

        let mut max_eval: f32 = match maximizing {
            true => f32::NEG_INFINITY,
            false => f32::INFINITY,
        };

        if !node.generated {
            node.generate_children(next_player);
        }

        if node.children.len() == 0 {
            println!("Node with no children found! {}", &node);
            println!("Ensuring generation");
            node.generate_children(next_player);
            panic!("{}", &node);
        }

        let mut max_child_index = 0;
        let mut child_index = 0;

        for mut child in &mut node.children {
            let minmax_inner_result = minimax_inner(
                &mut child,
                current_depth - 1,
                heuristic,
                child_index,
                !maximizing,
                player,
                round_number,
            );
            match minmax_inner_result {
                MinMaxResult::Eval(child_eval, _) => match maximizing {
                    false => {
                        if max_eval > child_eval {
                            max_eval = child_eval;
                            max_child_index = child_index;
                        }
                    }
                    true => {
                        if max_eval < child_eval {
                            max_eval = child_eval;
                            max_child_index = child_index;
                        }
                    }
                },
                MinMaxResult::Leaf(_, _) => return minmax_inner_result,
            }

            child_index += 1;
        }

        return MinMaxResult::Eval(max_eval, max_child_index);
    }

    let mut result: Option<MinMaxResult> = None;
    let mut rounds: u32 = 0;
    let mut player = match node.game_state{
        GameState::Start(player) => player,
        GameState::Moved(player) => player,
        GameState::Won(_) => panic!("Passed a winning node to minimax, probably a mistake!"),
    };
    let mut current_heuristic = false;

    loop {
        match log_level {
            LogLevel::None => {}
            _ => {
                println!("Playing round {}", rounds);
            }
        }
        match rounds_limit {
            Some(limit) => {
                if rounds == limit {
                    return None;
                }
            }
            None => {}
        }
        match result {
            Some(some_result) => match some_result {
                MinMaxResult::Eval(eval, eval_node) => {
                    *node = node.children.swap_remove(eval_node);
                    if matches!(log_level, LogLevel::All) {
                        println!("Evaluated node is: {} with score: {}", eval_node, eval);
                        println!("Children of node: {}", &node.children.len());
                        println!("{}", &node);
                    }

                    player = player.other();
                    current_heuristic = !current_heuristic;

                    result = match current_heuristic {
                        true => Some(minimax_inner(
                            node,
                            max_depth,
                            heuristics.0,
                            0,
                            true,
                            player,
                            rounds,
                        )),
                        false => Some(minimax_inner(
                            node,
                            max_depth,
                            heuristics.1,
                            0,
                            true,
                            player,
                            rounds,
                        )),
                    }
                }
                MinMaxResult::Leaf(final_node, from_depth) => {
                    return Some((final_node, max_depth - from_depth, rounds))
                }
            },
            None => {
                if matches!(log_level, LogLevel::All) {
                    println!("First node:\n{}", &node);
                }
                result = Some(minimax_inner(
                    node,
                    max_depth,
                    heuristics.0,
                    0,
                    true,
                    player,
                    rounds,
                ))
            }
        }
        rounds += 1;
    }
}
pub fn alfa_beta<A: Heuristic, B: Heuristic>(
    node: &mut DecisionTreeNode,
    max_depth: u32,
    heuristics: (&mut A, &mut B),
    rounds_limit: Option<u32>,
    log_level: &LogLevel,
) -> Option<(DecisionTreeNode, u32, u32)> {
    fn alfa_beta_inner(
        node: &mut DecisionTreeNode,
        current_depth: u32,
        heuristic: &mut impl Heuristic,
        node_index: usize,
        maximizing: bool,
        mut alfa: f32,
        mut beta: f32,
        player: Player,
        round_number: u32,
    ) -> MinMaxResult {
        let next_player = match node.game_state {
            GameState::Start(game_player) => game_player,
            GameState::Moved(game_player) => game_player.other(),
            GameState::Won(_) => {
                return MinMaxResult::Leaf(node.clone(), current_depth);
            }
        };

        if current_depth == 0 {
            return MinMaxResult::Eval(
                heuristic.evaluate(&node.board_state, player, round_number),
                node_index,
            );
        }

        let mut max_eval: f32 = match maximizing {
            true => f32::NEG_INFINITY,
            false => f32::INFINITY,
        };

        if !node.generated {
            node.generate_children(next_player);
        }

        let mut max_child_index = 0;
        let mut child_index = 0;
        if node.children.len() == 0 {
            println!("Node with no children found! {}", &node);
            println!("Ensuring generation");
            node.generate_children(next_player);
            panic!("{}", &node);
        }
        for mut child in &mut node.children {
            let minmax_inner_result = alfa_beta_inner(
                &mut child,
                current_depth - 1,
                heuristic,
                child_index,
                !maximizing,
                alfa,
                beta,
                player,
                round_number,
            );
            match minmax_inner_result {
                MinMaxResult::Eval(child_eval, _) => match maximizing {
                    false => {
                        if max_eval > child_eval {
                            max_eval = child_eval;
                            max_child_index = child_index;
                        }
                        if beta > child_eval {
                            beta = child_eval;
                        }
                    }
                    true => {
                        if max_eval < child_eval {
                            max_eval = child_eval;
                            max_child_index = child_index;
                        }
                        if alfa < child_eval {
                            alfa = child_eval;
                        }
                    }
                },

                MinMaxResult::Leaf(_, _) => return minmax_inner_result,
            }

            if beta <= alfa {
                break;
            }
            child_index += 1;
        }

        return MinMaxResult::Eval(max_eval, max_child_index);
    }

    let mut result: Option<MinMaxResult> = None;
    let mut rounds: u32 = 0;

    let mut current_heuristic = false;
    let mut player = match node.game_state{
        GameState::Start(player) => player,
        GameState::Moved(player) => player,
        GameState::Won(_) => panic!("Passed a winning node to alfabeta, probably a mistake!"),
    };

    loop {
        match log_level {
            LogLevel::None => {}
            _ => {
                println!("Playing round {}", rounds);
            }
        }
        match rounds_limit {
            Some(limit) => {
                if rounds == limit {
                    return None;
                }
            }
            None => {}
        }
        match result {
            Some(some_result) => match some_result {
                MinMaxResult::Eval(eval, eval_node) => {
                    *node = node.children.swap_remove(eval_node);

                    if matches!(log_level, LogLevel::All) {
                        println!("Evaluated node is: {} with score: {}", eval_node, eval);
                        println!("Children of node: {}", &node.children.len());
                        println!("{}", &node);
                    }

                    player = player.other();
                    current_heuristic = !current_heuristic;

                    result = match current_heuristic {
                        true => Some(alfa_beta_inner(
                            node,
                            max_depth,
                            heuristics.0,
                            0,
                            true,
                            f32::NEG_INFINITY,
                            f32::INFINITY,
                            player,
                            rounds,
                        )),
                        false => Some(alfa_beta_inner(
                            node,
                            max_depth,
                            heuristics.1,
                            0,
                            true,
                            f32::NEG_INFINITY,
                            f32::INFINITY,
                            player,
                            rounds,
                        )),
                    }
                }
                MinMaxResult::Leaf(final_node, from_depth) => {
                    return Some((final_node, max_depth - from_depth, rounds))
                }
            },
            None => {
                if matches!(log_level, LogLevel::All) {
                    println!("First node:\n{}", &node);
                }
                result = Some(alfa_beta_inner(
                    node,
                    max_depth,
                    heuristics.0,
                    0,
                    true,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    player,
                    rounds,
                ))
            }
        }
        rounds += 1;
    }
}
