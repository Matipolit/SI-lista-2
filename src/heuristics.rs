use crate::halma::{BoardState, Player};
use rand::prelude::*;

pub trait Heuristic {
    fn evaluate(&mut self, board_state: &BoardState, evaluating_player: Player, round_number: u32) -> f32;
    fn name(&self) -> String;
}

#[derive(Clone)]
pub struct HeuristicRandom {
    pub rng: ThreadRng,
}

#[derive(Clone)]
pub struct HeuristicProximity {
    pub power: f32,
    pub rng: ThreadRng,
}

#[derive(Clone)]
pub struct HeuristicProximityWithSingle {
    pub single_power: f32,
    pub multi_power: f32,
    pub rng: ThreadRng,
}

impl Heuristic for HeuristicRandom {
    fn evaluate(&mut self, _: &BoardState, _evaluating_player: Player, _: u32) -> f32 {
        return self.rng.gen_range(-100.0..100.0);
    }
    fn name(&self) -> String{
        return "Random".to_owned();
    }
}

impl Heuristic for HeuristicProximity {
    fn evaluate(&mut self, board_state: &BoardState, evaluating_player: Player, round_number: u32) -> f32 {
        // let mut score: f32 = if round_number > 200 {
        //     self.rng.gen_range(-0.1..0.1)
        // } else {
        //     0.
        // };
        let mut score = 0.;
        match evaluating_player {
            Player::Black => {
                for coord in board_state.black_coords{
                    score += PLAYER_BLACK_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                }
            }
            Player::White => {
                for coord in board_state.white_coords{
                    score += PLAYER_WHITE_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                }
            }
        }
        return (score * self.power).clamp(-100.0, 100.0);
    }

    fn name(&self) -> String{
        return "Proximity table".to_owned()
    }
}

impl Heuristic for HeuristicProximityWithSingle {
    fn evaluate(&mut self, board_state: &BoardState, evaluating_player: Player, round_number: u32) -> f32 {
        // let mut score: f32 = if round_number > 200 {
        //     self.rng.gen_range(-0.1..0.1)
        // } else {
        //     0.
        // };
        let mut score = 0.;
        let mut max: f32 = f32::NEG_INFINITY;
        match evaluating_player {
            Player::Black => {
                 for coord in board_state.black_coords{
                    let tile_ev = PLAYER_BLACK_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    score += tile_ev;
                    if tile_ev > max {
                        max = tile_ev;
                    }
                }
                
            }
            Player::White => {
                  for coord in board_state.black_coords{
                    let tile_ev = PLAYER_WHITE_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    score += tile_ev;
                    if tile_ev > max {
                        max = tile_ev;
                    }
                } 
            }
        }
        score = score * self.multi_power + max * self.single_power;
        return score.clamp(-100.0, 100.0);
    }
    fn name(&self) -> String {
        return "Proximity table with leading piece".to_owned()
    }
}

const PLAYER_WHITE_HEURISTIC_PROXIMITY: [[f32; 16]; 16] = [
    [
        4.7, 4.7, 4.3, 4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0,
    ],
    [
        4.3, 4.3, 4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3,
    ],
    [
        4.0, 4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7,
    ],
    [
        3.7, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0,
    ],
    [
        3.3, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3,
    ],
    [
        3.0, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7,
    ],
    [
        2.7, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0,
    ],
    [
        2.3, 2.3, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3,
    ],
    [
        2.0, 2.0, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3, -2.7,
    ],
    [
        1.7, 1.7, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3, -2.7, -3.0,
    ],
    [
        1.3, 1.3, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3, -2.7, -3.0, -3.3,
    ],
    [
        1.0, 1.0, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3, -2.7, -3.0, -3.3, -3.7,
    ],
    [
        0.7, 0.7, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3, -2.7, -3.0, -3.3, -3.7, -4.0,
    ],
    [
        0.3, 0.3, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3, -2.7, -3.0, -3.3, -3.7, -4.0, -4.3,
    ],
    [
        0.0, 0.0, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3, -2.7, -3.0, -3.3, -3.7, -4.0, -4.3,
        -4.7,
    ],
    [
        -0.3, -0.3, -0.7, -1.0, -1.3, -1.7, -2.0, -2.3, -2.7, -3.0, -3.3, -3.7, -4.0, -4.3, -4.7,
        -5.0,
    ],
];

const PLAYER_BLACK_HEURISTIC_PROXIMITY: [[f32; 16]; 16] = [
    [
        -5.0, -4.7, -4.3, -4.0, -3.7, -3.3, -3.0, -2.7, -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3,
        -0.3,
    ],
    [
        -4.7, -4.3, -4.0, -3.7, -3.3, -3.0, -2.7, -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0,
        0.0,
    ],
    [
        -4.3, -4.0, -3.7, -3.3, -3.0, -2.7, -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.3,
    ],
    [
        -4.0, -3.7, -3.3, -3.0, -2.7, -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 0.7,
    ],
    [
        -3.7, -3.3, -3.0, -2.7, -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.0,
    ],
    [
        -3.3, -3.0, -2.7, -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.3,
    ],
    [
        -3.0, -2.7, -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 1.7,
    ],
    [
        -2.7, -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.0,
    ],
    [
        -2.3, -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.3, 2.3,
    ],
    [
        -2.0, -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 2.7,
    ],
    [
        -1.7, -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 3.0, 3.0,
    ],
    [
        -1.3, -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 3.0, 3.3, 3.3,
    ],
    [
        -1.0, -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 3.0, 3.3, 3.7, 3.7,
    ],
    [
        -0.7, -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 3.0, 3.3, 3.7, 4.0, 4.0,
    ],
    [
        -0.3, 0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 3.0, 3.3, 3.7, 4.0, 4.3, 4.3,
    ],
    [
        0.0, 0.3, 0.7, 1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 3.0, 3.3, 3.7, 4.0, 4.3, 4.7, 4.7,
    ],
];
