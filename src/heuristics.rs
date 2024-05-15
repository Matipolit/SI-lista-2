use crate::halma::{BoardState, Coords, Player, DIRECTIONS, PLAYER_BLACK_BASE, PLAYER_WHITE_BASE};
use rand::prelude::*;

pub trait Heuristic {
    fn evaluate(
        &mut self,
        board_state: &BoardState,
        evaluating_player: Player,
        round_number: u32,
    ) -> f32;
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

#[derive(Clone)]
pub struct HeuristicDiscourageStart {
    pub other_power: f32,
    pub discourage_power: f32,
}

#[derive(Clone)]
pub struct HeuristicComplex {
    pub single_power: f32,
    pub multi_power: f32,
    pub discourage_power: f32,
}

impl Heuristic for HeuristicRandom {
    fn evaluate(&mut self, board_state: &BoardState, evaluating_player: Player, _: u32) -> f32 {
        let mut score: f32 = self.rng.gen_range(-100.0..100.0);
        match evaluating_player {
            Player::Black => {
                for coord in board_state.black_coords {
                    if PLAYER_BLACK_BASE.contains(&coord) {
                        score -= 2.;
                    }
                }
            }
            Player::White => {
                for coord in board_state.white_coords {
                    if PLAYER_WHITE_BASE.contains(&coord) {
                        score -= 2.;
                    }
                }
            }
        }
        return score.clamp(-100., 100.);
    }
    fn name(&self) -> String {
        return "Random".to_owned();
    }
}

impl Heuristic for HeuristicProximity {
    fn evaluate(
        &mut self,
        board_state: &BoardState,
        evaluating_player: Player,
        round_number: u32,
    ) -> f32 {
        // let mut score: f32 = if round_number > 200 {
        //     self.rng.gen_range(-0.1..0.1)
        // } else {
        //     0.
        // };
        let mut score = 0.;
        match evaluating_player {
            Player::Black => {
                for coord in board_state.black_coords {
                    score += PLAYER_BLACK_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    score += dont_block_other_player_in_base(
                        &coord,
                        &PLAYER_WHITE_BASE,
                        &board_state.white_coords,
                    );
                    score += discourage_edges(&coord);
                }
            }
            Player::White => {
                for coord in board_state.white_coords {
                    score += PLAYER_WHITE_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    score += dont_block_other_player_in_base(
                        &coord,
                        &PLAYER_BLACK_BASE,
                        &board_state.black_coords,
                    );
                    score += discourage_edges(&coord);
                }
            }
        }
        return (score * self.power).clamp(-100.0, 100.0);
    }

    fn name(&self) -> String {
        return "Proximity table".to_owned();
    }
}

impl Heuristic for HeuristicProximityWithSingle {
    fn evaluate(
        &mut self,
        board_state: &BoardState,
        evaluating_player: Player,
        round_number: u32,
    ) -> f32 {
        // let mut score: f32 = if round_number > 200 {
        //     self.rng.gen_range(-0.1..0.1)
        // } else {
        //     0.
        // };
        let mut score = 0.;
        let mut max: f32 = f32::NEG_INFINITY;
        match evaluating_player {
            Player::Black => {
                for coord in board_state.black_coords {
                    let tile_ev =
                        PLAYER_BLACK_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    score += tile_ev;
                    if tile_ev > max {
                        max = tile_ev;
                    }
                    score += dont_block_other_player_in_base(
                        &coord,
                        &PLAYER_WHITE_BASE,
                        &board_state.white_coords,
                    );
                    score += discourage_edges(&coord);
                }
            }
            Player::White => {
                for coord in board_state.white_coords {
                    let tile_ev =
                        PLAYER_WHITE_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    score += tile_ev;
                    if tile_ev > max {
                        max = tile_ev;
                    }
                    score += dont_block_other_player_in_base(
                        &coord,
                        &PLAYER_BLACK_BASE,
                        &board_state.black_coords,
                    );
                    score += discourage_edges(&coord);
                }
            }
        }
        score = score * self.multi_power + max * self.single_power;
        return score.clamp(-100.0, 100.0);
    }
    fn name(&self) -> String {
        return "Proximity table with leading piece".to_owned();
    }
}

impl Heuristic for HeuristicDiscourageStart {
    fn evaluate(
        &mut self,
        board_state: &BoardState,
        evaluating_player: Player,
        round_number: u32,
    ) -> f32 {
        // let mut score: f32 = if round_number > 200 {
        //     self.rng.gen_range(-0.1..0.1)
        // } else {
        //     0.
        // };
        let mut score = 0.;
        match evaluating_player {
            Player::Black => {
                for coord in board_state.black_coords {
                    score += PLAYER_BLACK_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    if PLAYER_BLACK_BASE.contains(&coord) {
                        score -= self.discourage_power;
                    }

                    score += dont_block_other_player_in_base(
                        &coord,
                        &PLAYER_WHITE_BASE,
                        &board_state.white_coords,
                    );
                    score += discourage_edges(&coord);
                }
            }
            Player::White => {
                for coord in board_state.white_coords {
                    score += PLAYER_WHITE_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    if PLAYER_WHITE_BASE.contains(&coord) {
                        score -= self.discourage_power;
                    }
                    score += dont_block_other_player_in_base(
                        &coord,
                        &PLAYER_BLACK_BASE,
                        &board_state.black_coords,
                    );
                    score += discourage_edges(&coord);
                }
            }
        }
        return (score * self.other_power).clamp(-100.0, 100.0);
    }

    fn name(&self) -> String {
        return "Proximity table discourage start".to_owned();
    }
}

impl Heuristic for HeuristicComplex {
    fn evaluate(
        &mut self,
        board_state: &BoardState,
        evaluating_player: Player,
        _round_number: u32,
    ) -> f32 {
        // let mut score: f32 = if round_number > 200 {
        //     self.rng.gen_range(-0.1..0.1)
        // } else {
        //     0.
        // };
        let mut score = 0.;
        let mut max: f32 = f32::NEG_INFINITY;
        match evaluating_player {
            Player::Black => {
                for coord in board_state.black_coords {
                    let mut tile_ev =
                        PLAYER_BLACK_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    // if PLAYER_BLACK_BASE.contains(&coord) {
                    //     tile_ev -= self.discourage_power;
                    // }
                    score += tile_ev;
                    if tile_ev > max {
                        max = tile_ev;
                    }
                    score += dont_block_other_player_in_base(
                        &coord,
                        &PLAYER_WHITE_BASE,
                        &board_state.white_coords,
                    );
                    score += discourage_edges(&coord);
                }
            }
            Player::White => {
                for coord in board_state.white_coords {
                    let mut tile_ev =
                        PLAYER_WHITE_HEURISTIC_PROXIMITY[coord.y as usize][coord.x as usize];
                    // if PLAYER_WHITE_BASE.contains(&coord) {
                    //     tile_ev -= self.discourage_power;
                    // }
                    score += tile_ev;
                    if tile_ev > max {
                        max = tile_ev;
                    }
                    score += dont_block_other_player_in_base(
                        &coord,
                        &PLAYER_BLACK_BASE,
                        &board_state.black_coords,
                    );
                    score += discourage_edges(&coord);
                }
            }
        }
        return (score * self.multi_power + max * self.single_power).clamp(-100., 100.);
    }

    fn name(&self) -> String {
        return "Complex".to_owned();
    }
}

#[inline]
fn dont_block_other_player_in_base(
    piece_coords: &Coords,
    other_player_base: &[Coords; 19],
    other_player_pieces: &[Coords; 19],
) -> f32 {
    let mut score = 0.;
    for direction in DIRECTIONS {
        let looking_at_coords = Coords {
            x: piece_coords.x + direction.x,
            y: piece_coords.y + direction.y,
        };
        if looking_at_coords.is_in_board() {
            if other_player_pieces.contains(&looking_at_coords) {
                if other_player_base.contains(&looking_at_coords) {
                    score -= 1.;
                }
            }
        }
    }
    return score;
}

#[inline]
fn discourage_edges(piece_coords: &Coords) -> f32 {
    if piece_coords.y == 0 || piece_coords.y == 15 {
        return -0.1;
    }
    return 0.;
}

const PLAYER_WHITE_HEURISTIC_PROXIMITY: [[f32; 16]; 16] = [
    [
        4.8, 4.8, 4.5, 4.2, 3.8, 3.5, 3.2, 2.8, 2.5, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2,
    ],
    [
        4.5, 4.5, 4.2, 3.8, 3.5, 3.2, 2.8, 2.5, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2,
    ],
    [
        4.2, 4.2, 3.8, 3.5, 3.2, 2.8, 2.5, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5,
    ],
    [
        3.8, 3.8, 3.5, 3.2, 2.8, 2.5, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8,
    ],
    [
        3.5, 3.5, 3.2, 2.8, 2.5, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2,
    ],
    [
        3.2, 3.2, 2.8, 2.5, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5,
    ],
    [
        2.8, 2.8, 2.5, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8,
    ],
    [
        2.5, 2.5, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2,
    ],
    [
        2.2, 2.2, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2, -2.5,
    ],
    [
        1.8, 1.8, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2, -2.5, -2.8,
    ],
    [
        1.5, 1.5, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2, -2.5, -2.8, -3.2,
    ],
    [
        1.2, 1.2, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2, -2.5, -2.8, -3.2, -3.5,
    ],
    [
        0.8, 0.8, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2, -2.5, -2.8, -3.2, -3.5, -3.8,
    ],
    [
        0.5, 0.5, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2, -2.5, -2.8, -3.2, -3.5, -3.8, -4.2,
    ],
    [
        0.2, 0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2, -2.5, -2.8, -3.2, -3.5, -3.8, -4.2,
        -4.5,
    ],
    [
        -0.2, -0.2, -0.5, -0.8, -1.2, -1.5, -1.8, -2.2, -2.5, -2.8, -3.2, -3.5, -3.8, -4.2, -4.5,
        -4.8,
    ],
];

const PLAYER_BLACK_HEURISTIC_PROXIMITY: [[f32; 16]; 16] = [
    [
        -4.8, -4.5, -4.2, -3.8, -3.5, -3.2, -2.8, -2.5, -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2,
        -0.2,
    ],
    [
        -4.5, -4.2, -3.8, -3.5, -3.2, -2.8, -2.5, -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2,
        0.2,
    ],
    [
        -4.2, -3.8, -3.5, -3.2, -2.8, -2.5, -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.5,
    ],
    [
        -3.8, -3.5, -3.2, -2.8, -2.5, -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 0.8,
    ],
    [
        -3.5, -3.2, -2.8, -2.5, -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.2,
    ],
    [
        -3.2, -2.8, -2.5, -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.5,
    ],
    [
        -2.8, -2.5, -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 1.8,
    ],
    [
        -2.5, -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.2,
    ],
    [
        -2.2, -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.5, 2.5,
    ],
    [
        -1.8, -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.5, 2.8, 2.8,
    ],
    [
        -1.5, -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.5, 2.8, 3.2, 3.2,
    ],
    [
        -1.2, -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.5, 2.8, 3.2, 3.5, 3.5,
    ],
    [
        -0.8, -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.5, 2.8, 3.2, 3.5, 3.8, 3.8,
    ],
    [
        -0.5, -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.5, 2.8, 3.2, 3.5, 3.8, 4.2, 4.2,
    ],
    [
        -0.2, 0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.5, 2.8, 3.2, 3.5, 3.8, 4.2, 4.5, 4.5,
    ],
    [
        0.2, 0.5, 0.8, 1.2, 1.5, 1.8, 2.2, 2.5, 2.8, 3.2, 3.5, 3.8, 4.2, 4.5, 4.8, 4.8,
    ],
];

pub fn print_new_table() {
    println!("[");
    for y in 0..16 {
        print!("[");
        for x in 0..16 {
            let mut score = PLAYER_BLACK_HEURISTIC_PROXIMITY[y][x];
            if PLAYER_WHITE_BASE.contains(&Coords {
                x: x as i8,
                y: y as i8,
            }) {
                score += 0.1;
            }
            print!("{:.1}, ", score);
        }
        println!("],")
    }
    println!("]")
}
