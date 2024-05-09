use crate::halma::{
    board_to_string, index_board, Board, Coords, GameState, Player, Tile, DIRECTIONS,
    PLAYER_BLACK_WINNING, PLAYER_WHITE_WINNING,
};

use petgraph::{
    dot::{Config, Dot},
    prelude::*,
    Graph,
};

use std::{
    env::{self, SplitPaths},
    fmt::Display,
    fs,
};

#[derive(Clone, Debug)]
pub struct DecisionTreeNode {
    pub board: Board,
    pub game_state: GameState,
    pub children: Vec<DecisionTreeNode>,
    pub generated: bool,
}

impl Display for DecisionTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node:\nGame state: {:?}, children amount: {}, generated: {}\n{}",
            &self.game_state,
            &self.children.len(),
            &self.generated,
            board_to_string(&self.board)
        )
    }
}

impl DecisionTreeNode {
    pub fn new(board: Board, game_state: GameState) -> Self {
        DecisionTreeNode {
            board,
            game_state,
            children: Vec::with_capacity(40),
            generated: false,
        }
    }
    // for a given state of the board, generate all possible moves and set them as children
    pub fn generate_children(&mut self, player_moving: Player) {
        self.generated = true;
        for y in 0..16 {
            for x in 0..16 {
                let from_coords = Coords { x, y };
                let my_winning_coords = match player_moving {
                    Player::Black => PLAYER_BLACK_WINNING,
                    Player::White => PLAYER_WHITE_WINNING,
                };
                //only move the current player's tiles
                if self.is_tile_mine(from_coords, player_moving) {
                    //check all 8 possible dirctions
                    for direction in DIRECTIONS.into_iter() {
                        let move_to = Coords {
                            x: x + direction.x,
                            y: y + direction.y,
                        };

                        if move_to.is_in_board() {
                            //normal moves
                            if self.is_tile_empty(move_to) {
                                if my_winning_coords.contains(&from_coords) {
                                    if my_winning_coords.contains(&move_to) {
                                        self.add_child_node(from_coords, move_to, player_moving);
                                    }
                                } else {
                                    self.add_child_node(from_coords, move_to, player_moving);
                                }
                            //jumping
                            } else {
                                let jump_to = Coords {
                                    x: move_to.x + direction.x,
                                    y: move_to.y + direction.y,
                                };
                                if jump_to.is_in_board() {
                                    if self.is_tile_empty(jump_to) {
                                        let mut jump_points: Vec<Coords> = vec![jump_to];
                                        self.generate_valid_jumps_for_point(
                                            &mut jump_points,
                                            jump_to,
                                            direction,
                                        );
                                        for jump_point in jump_points {
                                            if my_winning_coords.contains(&from_coords) {
                                                if !my_winning_coords.contains(&jump_point) {
                                                    continue;
                                                }
                                            }
                                            self.add_child_node(
                                                from_coords,
                                                jump_point,
                                                player_moving,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn add_child_node(&mut self, move_from: Coords, move_to: Coords, player_moved: Player) {
        let mut new_board = self.board.clone();
        Self::move_tile(&mut new_board, move_from, move_to);
        let new_game_state = if Self::is_game_won(&new_board, player_moved) {
            GameState::Won(player_moved)
        } else {
            GameState::Moved(player_moved)
        };
        self.children.push(Self::new(new_board, new_game_state))
    }

    fn generate_valid_jumps_for_point(
        &self,
        jumps: &mut Vec<Coords>,
        point: Coords,
        previous_direction: Coords,
    ) {
        for direction in DIRECTIONS {
            if direction != previous_direction {
                let check_point = Coords {
                    x: point.x + direction.x,
                    y: point.y + direction.y,
                };
                if check_point.is_in_board() {
                    if !self.is_tile_empty(check_point) {
                        let jump_to_point = Coords {
                            x: check_point.x + direction.x,
                            y: check_point.y + direction.y,
                        };
                        if jump_to_point.is_in_board() {
                            if self.is_tile_empty(jump_to_point) {
                                if !jumps.contains(&jump_to_point) {
                                    jumps.push(jump_to_point);
                                    self.generate_valid_jumps_for_point(
                                        jumps,
                                        jump_to_point,
                                        direction,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    fn is_tile_empty(&self, coords: Coords) -> bool {
        matches!(
            self.board[coords.y as usize][coords.x as usize],
            Tile::Empty
        )
    }

    #[inline]
    fn is_tile_mine(&self, coords: Coords, player_moving: Player) -> bool {
        match player_moving {
            Player::Black => matches!(
                self.board[coords.y as usize][coords.x as usize],
                Tile::Black
            ),
            Player::White => matches!(
                self.board[coords.y as usize][coords.x as usize],
                Tile::White
            ),
        }
    }

    #[inline]
    fn move_tile(board: &mut Board, from: Coords, to: Coords) {
        let tile = board[from.y as usize][from.x as usize];
        board[from.y as usize][from.x as usize] = Tile::Empty;
        board[to.y as usize][to.x as usize] = tile;
    }

    #[inline]
    fn is_game_won(board: &Board, player_moved: Player) -> bool {
        let checked_positions = match player_moved {
            Player::Black => PLAYER_BLACK_WINNING,
            Player::White => PLAYER_WHITE_WINNING,
        };
        let checked_tile = match player_moved {
            Player::Black => Tile::Black,
            Player::White => Tile::White,
        };
        for position in checked_positions.into_iter() {
            if index_board(board, position) != checked_tile {
                return false;
            }
        }
        return true;
    }
}

// generate all the moves for the game
// first player is always black
// get the first state of the board, generate a decision tree node for it, then for all its children and so on, until either player wins
pub fn generate_tree(
    max_depth: Option<u64>,
    first_board: Board,
    graph: &mut Option<&mut Graph<DecisionTreeNode, u64>>,
) -> DecisionTreeNode {
    fn generate_tree_inner(
        node: &mut DecisionTreeNode,
        current_depth: u64,
        max_depth: Option<u64>,
        graph: &mut Option<&mut Graph<DecisionTreeNode, u64>>,
        parent_index: Option<NodeIndex>,
    ) {
        let player = match node.game_state {
            GameState::Start(player) => player,
            GameState::Moved(player) => player,
            GameState::Won(player) => player,
        };
        if max_depth.is_some() {
            if current_depth > max_depth.unwrap() {
                return;
            }
        }
        node.generate_children(player.other());
        for mut child in &mut node.children {
            let mut child_index: Option<NodeIndex> = None;
            if graph.is_some() {
                child_index = Some(graph.as_mut().unwrap().add_node(child.clone()));
                graph.as_mut().unwrap().add_edge(
                    parent_index.unwrap(),
                    child_index.unwrap(),
                    current_depth,
                );
            }

            if !matches!(child.game_state, GameState::Won(_)) {
                generate_tree_inner(&mut child, current_depth + 1, max_depth, graph, child_index);
            } else {
                //println!("Child won!");
                //println!("{}, {:?}", board_to_string(&child.board), child.game_state);
            }
        }
        return;
    }
    let mut first_node = DecisionTreeNode {
        board: first_board,
        game_state: GameState::Start(Player::White),
        children: Vec::with_capacity(40),
        generated: false,
    };
    let mut first_index: Option<NodeIndex> = None;
    if graph.is_some() {
        first_index = Some(graph.as_mut().unwrap().add_node(first_node.clone()));
    }
    generate_tree_inner(&mut first_node, 0, max_depth, graph, first_index);
    first_node
}
