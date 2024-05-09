use std::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Black,
    White,
}
pub type Board = [[Tile; 16]; 16];

pub fn board_to_string(board: &Board) -> String {
    let mut base_string = "".to_owned();
    for row in board {
        for cell in row {
            match cell {
                Tile::Empty => base_string += "0",
                Tile::Black => base_string += "1",
                Tile::White => base_string += "2",
            }
        }
        base_string += "\n"
    }
    return base_string;
}
pub fn board_from_str(board_str: &String) -> Result<Board, Box<dyn Error>> {
    let split_str = board_str.split('\n');
    let mut board = [[Tile::Empty; 16]; 16];
    split_str.enumerate().for_each(|(row_idx, line)| {
        line.chars()
            .enumerate()
            .for_each(|(char_idx, char)| match char {
                '0' => {}
                '1' => {
                    board[row_idx][char_idx] = Tile::Black;
                }
                '2' => {
                    board[row_idx][char_idx] = Tile::White;
                }
                _ => {}
            });
    });
    Ok(board)
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

#[derive(Clone, Copy)]
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

pub static PLAYER_BLACK_WINNING: [Coords; 19] = [
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

pub static PLAYER_WHITE_WINNING: [Coords; 19] = [
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
