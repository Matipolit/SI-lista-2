use std::{
    env::{self, SplitPaths},
    error::Error,
};

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Black,
    White,
}
type Board = [[Tile; 16]; 16];

enum Player {
    Black,
    White,
}

enum MoveResult {
    NotPossible,
    Move,
    Jump(u8, u8),
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

enum GameState {
    Start,
    Moved(Player),
    Won(Player),
}

struct Move {
    from: (u8, u8),
    to: (u8, u8),
}

struct DecisionTreeNode {
    board: Board,
    gameState: GameState,
    children: Vec<(Move, DecisionTreeNode)>,
}

struct Coords {
    x: i8,
    y: i8
}

static DIRECTIONS: [Coords; 8] = [
    (-1, -1),
    (-1, 0),
    (0, -1),
    (1, -1),
    (-1, 1),
    (0, 1),
    (1, 0),
    (1, 1),
];

static 

impl DecisionTreeNode {
    // for a given state of the board, generate all possible moves and set them as children
    fn generateChildren(&mut self, playerMoving: Player) {
        for y in 0..15 {
            for x in 0..15 {
                if(self.tile_is_mine(x, y, playerMoving)){
                    for direction in DIRECTIONS.into_iter(){
                        let move_to = (x as i8 + direction.0, y as i8 + direction.1);
                        if DecisionTreeNode::coords_in_board(move_to.0, move_to.1){
                            if self.tile_is_empty(move_to.0 as usize, move_to.1 as usize) {
                                let mut new_board = self.board.clone();
                                DecisionTreeNode::move_tile(&mut new_board, (x, y), (move_to.0 as usize, move_to.1 as usize))
                                self.children.push(DecisionTreeNode{board: new_board, })
                            }
                            
                        }
                        
                    }
                }
            }
        }
    }
    #[inline]
    fn coords_in_board(x: i8, y: i8) -> bool{
        (x < 16) && (x > -1) && (y < 16) && (y > -1)
    }

    
    #[inline]
    fn tile_is_empty(&self, x: usize, y: usize) -> bool {
        matches!(self.board[y][x], Tile::Empty)
    }

    #[inline]
    fn tile_is_mine(&self, x: usize,y: usize, playerMoving: Player) -> bool{
        match playerMoving{
            Player::Black => matches!(self.board[y][x], Tile::Black)
            Player::White => matches!(self.board[y][x], Tile::White)
        }
    }

    #[inline]
    fn move_tile(board: &mut Board, from: (usize, usize), to: (usize, usize)){
        let tile = board[from.1][from.0];
        board[from.1][from.0] = Tile::Empty;
        board[to.1][to.0] = tile;
    }

    #[inline]
    fn check_game_state(&self){
        
    }
}

// generate all the moves for the game
// first player is always black
// get the first state of the board, generate a decision tree node for it, then for all its children and so on, until either player wins
fn generateTree() -> DecisionTreeNode {}

fn board_from_str(board_str: &String) -> Result<Board, Box<dyn Error>> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments! Usage: cargo run --release -- <board string>");
    }
    let board = board_from_str(&args[1]);
}
