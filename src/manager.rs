use crate::board::Board;
use crate::tetromino::{Position, TetrisAction, Tetromino, TetrominoState, TetrominoType};

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

    // Check if next state valid
    fn is_valid_state(&self, new_state: TetrominoState, offset: Position) -> bool {
        todo!()
    }

    /// return Some(true) if move successfully
    /// return Some(false) if cannot move
    /// return None if reaches bottom
    pub fn movement(&mut self, action: TetrisAction) -> Option<bool> {
        match action {
            TetrisAction::C => {
                let offsets = TetrominoType::offsets(self.curr_tetromino.ttype, self.curr_tetromino.rotation.clockwise());

                for offset in offsets {
                    let Ok(new_state) = self.curr_tetromino.peek_rotate(action) else {
                        continue;
                    };
                    if self.is_valid_state(new_state, offset) {
                        self.curr_tetromino.rotate(action);
                        return Some(true);
                    }
                }
                Some(false)
            },
            TetrisAction::W => {
                let offsets = TetrominoType::offsets(self.curr_tetromino.ttype, self.curr_tetromino.rotation.counter_clockwise());

                for offset in offsets {
                    let Ok(new_state) = self.curr_tetromino.peek_rotate(action) else {
                        continue;
                    };
                    if self.is_valid_state(new_state, offset) {
                        self.curr_tetromino.rotate(action);
                        return Some(true);
                    }
                }
                Some(false)
            },
            TetrisAction::D => {
                let offsets = vec![Position(0, -1)];
                
                for offset in offsets {
                    let Ok(new_state) = self.curr_tetromino.peek_rotate(action) else {
                        continue;
                    };
                    if self.is_valid_state(new_state, offset) {
                        self.curr_tetromino.rotate(action);
                        return Some(true);
                    }
                }
                Some(false)
            },
            _ => todo!()
        }
    }
}

