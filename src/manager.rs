use crate::board::Board;
use crate::tetromino::{Position, TetrisAction, Tetromino, TetrominoType};

use crate::bag_randomizer::BagRandomizer;

pub enum ManagerError {
    PositionNotOnBoard,
    InsertIntoOccupiedPosition,
    RemoveFromEmptyPosition
}

struct GameManager {
    time: u64,
    gravity: u64,
    board: Board,
    bag_randomizer: BagRandomizer<TetrominoType>,
    curr_pos: Position,
    curr_tetromino: Tetromino
}

impl GameManager {
    pub fn new(gravity: u64, board: Board, mut bag_randomizer: BagRandomizer<TetrominoType>) -> Self {
        let curr_tetromino = Tetromino::new(bag_randomizer.cur().clone());
        bag_randomizer.next();
        Self { 
            time: 0, 
            gravity,
            board,
            bag_randomizer,
            curr_pos: Position(0, 0),
            curr_tetromino
        }
    }

    pub fn run(&self) {
        loop {

        }
    }

    pub fn movement(&self, action: TetrisAction) {

    }
}

