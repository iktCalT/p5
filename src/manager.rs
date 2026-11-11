use crate::board::Board;
use crate::tetromino::{Position, TetrisAction, Tetromino, TetrominoType};

use crate::bag_randomizer::BagRandomizer;

pub enum ManagerError {
    PositionNotOnBoard,
    InsertIntoOccupiedPosition,
    RemoveFromEmptyPosition,
    ActionError
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

    // Check if current tetromino on valid state
    fn is_valid_state(&self) -> bool {

    }

    pub fn movement(&self, action: TetrisAction) {
        match action {
            TetrisAction::C => {
                self.curr_tetromino.rotate(action);
                if !self.is_valid_state() {
                    self.curr_tetromino.rotate(TetrisAction::W);
                }
            },
            TetrisAction::W => {
                self.curr_tetromino.rotate(action);
                if !self.is_valid_state() {
                    self.curr_tetromino.rotate(TetrisAction::C);
                }
            },
            Te
        }
    }
}

