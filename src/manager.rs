use std::sync::mpsc::Receiver;

use crate::board::Board;
use crate::setting::Settings;
use crate::tetromino::{Position, TetrisAction, Tetromino, TetrominoState, TetrominoType};

use crate::bag_randomizer::BagRandomizer;

#[derive(Debug, PartialEq)]
pub enum ManagerError {
    PositionNotOnBoard,
    InsertIntoOccupiedPosition,
    RemoveFromEmptyPosition,
    ActionError,
    ReceiveError
}

pub struct GameManager {
    settings: Settings,
    rx: Receiver<TetrisAction>,
    time: u64,
    gravity: u64,
    board: Board,
    bag_randomizer: BagRandomizer<TetrominoType>,
    curr_pos: Position,
    curr_tetromino: Tetromino,
}

impl GameManager {
    pub fn new(gravity: u64, settings: Settings, rx: Receiver<TetrisAction>, 
            board: Board, mut bag_randomizer: BagRandomizer<TetrominoType>) -> Self {
        let curr_tetromino = Tetromino::new(bag_randomizer.cur().clone());
        bag_randomizer.next();
        let curr_pos = Position(((board.width - 1) / 2) as isize, (board.height - 1) as isize);
        Self {
            settings,
            rx,
            time: 0,
            gravity,
            board,
            bag_randomizer,
            curr_pos,
            curr_tetromino,
        }
    }

    pub fn show(&self) {
        self.board.print();
    }

    /// Only for debugging
    pub fn show_with_tetromino(&self) {
        self.board.print_with_tetromino(&self.curr_tetromino, &self.curr_pos);
    }

    pub fn run(&mut self) -> Result<(), ManagerError> {
        loop {
            // TODO input action
            let Ok(action) = self.rx.recv() else {
                return Err(ManagerError::ReceiveError);
            };

            if action == TetrisAction::Q {
                break;
            }

            if self.movement(action).is_none() {
                for point in self.curr_tetromino.state.0.iter() {
                    self.board.insert(self.curr_pos + point.clone()).unwrap();
                }
                self.bag_randomizer.next();
                self.curr_tetromino = Tetromino::new(self.bag_randomizer.cur().clone());
                self.curr_pos = Position(((self.board.width - 1) / 2) as isize, (self.board.height - 1) as isize);
                self.time = 0;
                continue;
            }
            self.time += 1;

            if self.time % self.gravity == 0 {
                // Move 1 step down
                if self.movement(TetrisAction::D).is_none() {
                    for point in self.curr_tetromino.state.0.iter() {
                        self.board.insert(self.curr_pos + point.clone()).unwrap();
                    }
                    self.bag_randomizer.next();
                    self.curr_tetromino = Tetromino::new(self.bag_randomizer.cur().clone(), );
                    self.curr_pos = Position(((self.board.width - 1) / 2) as isize, (self.board.height - 1) as isize);
                    self.time = 0;
                }
            }
        }
        Ok(())
    }

    // Check if next state valid
    fn is_valid_state(&self, new_state: &TetrominoState, offset: Position) -> bool {
        let new_position = self.curr_pos + offset;
        for point in &new_state.0 {
            if !self.board.is_empty(&(new_position + point.clone())) {
                return false;
            }
        }
        true
    }

    /// return Some(true) if move successfully
    /// return Some(false) if cannot move
    /// return None if reaches bottom
    pub fn movement(&mut self, action: TetrisAction) -> Option<bool> {
        match action {
            TetrisAction::C => {
                let offsets = self.curr_tetromino.ttype.wall_kick(self.curr_tetromino.rotation.clockwise());

                for offset in offsets {
                    let Ok(new_state) = self.curr_tetromino.peek_rotate(action) else {
                        continue;
                    };
                    if self.is_valid_state(&new_state, offset) {
                        self.curr_tetromino.rotate(action).unwrap();
                        self.curr_pos += offset;
                        return Some(true);
                    }
                }
                Some(false)
            },
            TetrisAction::W => {
                let offsets = self.curr_tetromino.ttype.wall_kick(self.curr_tetromino.rotation.counter_clockwise());

                for offset in offsets {
                    let Ok(new_state) = self.curr_tetromino.peek_rotate(action) else {
                        continue;
                    };
                    if self.is_valid_state(&new_state, offset) {
                        self.curr_tetromino.rotate(action).unwrap();
                        self.curr_pos += offset;
                        return Some(true);
                    }
                }
                Some(false)
            },
            TetrisAction::D => {
                let offset = Position(0, -1);

                if self.is_valid_state(&self.curr_tetromino.state, offset) {
                    self.curr_pos += offset;
                    return Some(true);
                }
                None
            },
            TetrisAction::L => {
                let offset = Position(-1, 0);
                if self.is_valid_state(&self.curr_tetromino.state, offset) {
                    self.curr_pos += offset;
                    return Some(true);
                }
                Some(false)
            },
            TetrisAction::R => {
                let offset = Position(1, 0);

                if self.is_valid_state(&self.curr_tetromino.state, offset) {
                    self.curr_pos += offset;
                    return Some(true);
                }
                Some(false)
            },
            TetrisAction::E => Some(true),
            TetrisAction::H => {
                let mut offset = Position(0, -1);
                while self.is_valid_state(&self.curr_tetromino.state, offset) {
                    offset += Position(0, -1);
                }
                self.curr_pos += offset + Position(0, 1);
                None
            },
            TetrisAction::Q => None
        }
    }
}

