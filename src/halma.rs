use std::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Black,
    White,
}
pub type Board = [[Tile; 16]; 16];

pub type PlayerCoords = [Coords; 19];

#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    pub black_coords: PlayerCoords,
    pub white_coords: PlayerCoords,
}

pub fn board_state_to_string(board_state: &BoardState) -> String {
    let mut base_string = "".to_owned();
    for y in 0..16 {
        for x in 0..16 {
            if board_state.black_coords.contains(&Coords { x, y }) {
                base_string += "1";
            } else if board_state.white_coords.contains(&Coords { x, y }) {
                base_string += "2";
            } else {
                base_string += "0";
            }
        }
        base_string += "\n";
    }
    return base_string;
}

pub fn board_state_from_str(board_str: &String) -> Result<BoardState, Box<dyn Error>> {
    let split_str = board_str.split('\n');
    let mut black_coords: Vec<Coords> = Vec::with_capacity(19);
    let mut white_coords: Vec<Coords> = Vec::with_capacity(19);
    split_str.enumerate().for_each(|(row_idx, line)| {
        line.chars()
            .enumerate()
            .for_each(|(char_idx, char)| match char {
                '0' => {}
                '1' => {
                    black_coords.push(Coords {
                        x: char_idx as i8,
                        y: row_idx as i8,
                    });
                }
                '2' => {
                    white_coords.push(Coords {
                        x: char_idx as i8,
                        y: row_idx as i8,
                    });
                }
                _ => {}
            });
    });
    Ok(BoardState {
        black_coords: black_coords.as_slice().try_into()?,
        white_coords: white_coords.as_slice().try_into()?,
    })
}

#[inline]
pub fn index_board(board: &Board, coords: Coords) -> Tile {
    board[coords.y as usize][coords.x as usize]
}

#[derive(Clone, Copy, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Start(Player),
    Moved(Player),
    Won(Player),
}

#[derive(Debug, Clone, Copy)]
pub struct Coords {
    pub x: i8,
    pub y: i8,
}

impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coords {
    #[inline]
    pub fn is_in_board(&self) -> bool {
        (self.x < 16) && (self.x > -1) && (self.y < 16) && (self.y > -1)
    }
}

pub static DIRECTIONS: [Coords; 8] = [
    Coords { x: -1, y: -1 },
    Coords { x: -1, y: 0 },
    Coords { x: 0, y: -1 },
    Coords { x: 1, y: -1 },
    Coords { x: -1, y: 1 },
    Coords { x: 0, y: 1 },
    Coords { x: 1, y: 0 },
    Coords { x: 1, y: 1 },
];

pub static PLAYER_WHITE_BASE: [Coords; 19] = [
    Coords { x: 14, y: 11 },
    Coords { x: 15, y: 11 },
    Coords { x: 13, y: 12 },
    Coords { x: 14, y: 12 },
    Coords { x: 15, y: 12 },
    Coords { x: 12, y: 13 },
    Coords { x: 13, y: 13 },
    Coords { x: 14, y: 13 },
    Coords { x: 15, y: 13 },
    Coords { x: 11, y: 14 },
    Coords { x: 12, y: 14 },
    Coords { x: 13, y: 14 },
    Coords { x: 14, y: 14 },
    Coords { x: 15, y: 14 },
    Coords { x: 11, y: 15 },
    Coords { x: 12, y: 15 },
    Coords { x: 13, y: 15 },
    Coords { x: 14, y: 15 },
    Coords { x: 15, y: 15 },
];

pub static PLAYER_BLACK_BASE: [Coords; 19] = [
    Coords { x: 0, y: 0 },
    Coords { x: 1, y: 0 },
    Coords { x: 2, y: 0 },
    Coords { x: 3, y: 0 },
    Coords { x: 4, y: 0 },
    Coords { x: 0, y: 1 },
    Coords { x: 1, y: 1 },
    Coords { x: 2, y: 1 },
    Coords { x: 3, y: 1 },
    Coords { x: 4, y: 1 },
    Coords { x: 0, y: 2 },
    Coords { x: 1, y: 2 },
    Coords { x: 2, y: 2 },
    Coords { x: 3, y: 2 },
    Coords { x: 0, y: 3 },
    Coords { x: 1, y: 3 },
    Coords { x: 2, y: 3 },
    Coords { x: 0, y: 4 },
    Coords { x: 1, y: 4 },
];
