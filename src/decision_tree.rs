use crate::halma::{
    board_state_to_string, BoardState, Coords, GameState, Player, DIRECTIONS, PLAYER_BLACK_BASE,
    PLAYER_WHITE_BASE,
};

use petgraph::{prelude::*, Graph};

use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct DecisionTreeNode {
    pub board_state: BoardState,
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
            board_state_to_string(&self.board_state)
        )
    }
}

impl DecisionTreeNode {
    pub fn new(board_state: BoardState, game_state: GameState) -> Self {
        DecisionTreeNode {
            board_state,
            game_state,
            children: Vec::with_capacity(40),
            generated: false,
        }
    }
    // for a given state of the board, generate all possible moves and set them as children
    pub fn generate_children(&mut self, player_moving: Player) {
        self.generated = true;
        let my_coords = match player_moving {
            Player::Black => self.board_state.black_coords,
            Player::White => self.board_state.white_coords,
        };
        // for y in 0..16 {
        //     for x in 0..16 {
        for (from_coords_idx, from_coords) in my_coords.into_iter().enumerate() {
            let my_winning_coords = match player_moving {
                Player::Black => PLAYER_WHITE_BASE,
                Player::White => PLAYER_BLACK_BASE,
            };
            //check all 8 possible dirctions
            for direction in DIRECTIONS.into_iter() {
                let move_to = Coords {
                    x: from_coords.x + direction.x,
                    y: from_coords.y + direction.y,
                };

                if move_to.is_in_board() {
                    //normal moves
                    if self.is_tile_empty(move_to) {
                        if my_winning_coords.contains(&from_coords) {
                            if my_winning_coords.contains(&move_to) {
                                self.add_child_node(from_coords_idx, move_to, player_moving);
                            }
                        } else {
                            self.add_child_node(from_coords_idx, move_to, player_moving);
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
                                    self.add_child_node(from_coords_idx, jump_point, player_moving);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn add_child_node(&mut self, move_from_idx: usize, move_to: Coords, player_moved: Player) {
        let mut new_board_state = self.board_state.clone();
        Self::move_tile(&mut new_board_state, move_from_idx, move_to, player_moved);
        let new_game_state = if Self::is_game_won(&new_board_state, player_moved) {
            GameState::Won(player_moved)
        } else {
            GameState::Moved(player_moved)
        };
        self.children
            .push(Self::new(new_board_state, new_game_state))
    }

    fn generate_valid_jumps_for_point(
        &self,
        jump_points: &mut Vec<Coords>,
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
                                if !jump_points.contains(&jump_to_point) {
                                    jump_points.push(jump_to_point);
                                    self.generate_valid_jumps_for_point(
                                        jump_points,
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
        (!self.board_state.black_coords.contains(&coords))
            && (!self.board_state.white_coords.contains(&coords))
    }

    #[inline]
    fn move_tile(board_state: &mut BoardState, from_idx: usize, to: Coords, player: Player) {
        match player {
            Player::Black => {
                board_state.black_coords[from_idx] = to;
            }
            Player::White => {
                board_state.white_coords[from_idx] = to;
            }
        };
    }

    #[inline]
    fn is_game_won(board_state: &BoardState, player_moved: Player) -> bool {
        let winning_positions = match player_moved {
            Player::Black => PLAYER_WHITE_BASE,
            Player::White => PLAYER_BLACK_BASE,
        };
        let my_positions = match player_moved {
            Player::Black => &board_state.black_coords,
            Player::White => &board_state.white_coords,
        };

        for my_position in my_positions.iter() {
            if !winning_positions.contains(&my_position) {
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
    first_board_state: BoardState,
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
        board_state: first_board_state,
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
