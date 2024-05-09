use crate::decision_tree::DecisionTreeNode;
use crate::halma::{board_to_string, Board, GameState, Player};

use std::cmp::{max, min};
use std::ops::{Add, AddAssign};

use crate::heuristics::{Heuristic, HeuristicProximity};

pub struct MinMaxResult {
    final_node: DecisionTreeNode,
    rounds: u32,
    visited_nodes: u128,
    time_millis: u128,
}

enum MinMaxInnerResult {
    HeuristicEval(i32, DecisionTreeNode),
    Leaf(DecisionTreeNode),
}

pub fn minmax<'a>(
    max_depth: u32,
    starting_board: Board,
    heuristic: &mut impl Heuristic,
    round_limit: Option<u32>,
) -> Option<MinMaxResult> {
    fn minmax_inner<'a>(
        current_depth: u32,
        max_depth: u32,
        node: &'a mut DecisionTreeNode,
        heuristic: &mut impl Heuristic,
        visited_children: &mut u128,
    ) -> MinMaxInnerResult {
        //println!("minmax inner");
        visited_children.add_assign(1);

        let player = match node.game_state {
            GameState::Start(player) => player,
            GameState::Moved(player) => player,
            GameState::Won(player) => player,
        };

        let mut max_eval: (i32, DecisionTreeNode) = (
            match player {
                Player::Black => -100,
                Player::White => 100,
            },
            node.clone(),
        );
        if matches!(node.game_state, GameState::Won(_)) {
            return MinMaxInnerResult::Leaf(node.clone());
        } else if current_depth == 0 {
            let value = heuristic.evaluate(&node.board, player);
            //println!("Depth is 0, evaluating for player: {:?}", current_player);
            return MinMaxInnerResult::HeuristicEval(value, node.clone());
        } else {
            //println!("Depth: {}, player: {:?}", current_depth, current_player);
            node.generate_children(player.other());
            for mut child in &mut node.children {
                let child_minmax = minmax_inner(
                    current_depth - 1,
                    max_depth,
                    &mut child,
                    heuristic,
                    visited_children,
                );
                match child_minmax {
                    MinMaxInnerResult::HeuristicEval(eval, child_node) => match player {
                        Player::Black => {
                            if eval > max_eval.0 {
                                println!("Found better eval on level {}.\n Previous: {}\n{}\nCurrent: {}\n{}",
                                     current_depth,
                                     max_eval.0, max_eval.1,
                                     eval, child_node
                                 );
                                max_eval = (eval, child_node);
                            }
                        }
                        Player::White => {
                            if eval < max_eval.0 {
                                max_eval = (eval, child_node);
                            }
                        }
                    },
                    MinMaxInnerResult::Leaf(node) => return MinMaxInnerResult::Leaf(node),
                }
            }
            return MinMaxInnerResult::HeuristicEval(max_eval.0, max_eval.1);
        }
    }

    let mut first_node = DecisionTreeNode::new(starting_board, GameState::Start(Player::White));
    let mut visited_nodes: u128 = 0;
    let mut rounds: u32 = 0;
    let mut current_result = MinMaxInnerResult::HeuristicEval(-100, first_node.clone());

    loop {
        if round_limit.is_some() {
            if rounds > round_limit.unwrap() {
                break;
            }
        }
        match current_result {
            MinMaxInnerResult::Leaf(final_node) => {
                return Some(MinMaxResult {
                    final_node,
                    rounds,
                    visited_nodes,
                    time_millis: 0,
                })
            }
            MinMaxInnerResult::HeuristicEval(eval, node) => {
                println!(
                    "round: {}, eval: {}, game_state: {:?}",
                    &rounds, &eval, &node.game_state
                );
                println!("board:\n{}", board_to_string(&node.board));
                current_result = minmax_inner(
                    max_depth,
                    max_depth,
                    &mut node.clone(),
                    heuristic,
                    &mut visited_nodes,
                );
                rounds += 1;
            }
        }
    }
    return None;
}
