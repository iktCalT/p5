use std::sync::mpsc::{Receiver, Sender};

use crate::board::{Board, CellState};
use crate::bot::BoardState;
use crate::setting::SETTINGS;
use crate::tetromino::{Position, TetrisAction, Tetromino, TetrominoState, TetrominoType};

use crate::bag_randomizer::BagRandomizer;

#[derive(Debug, PartialEq)]
pub enum ManagerError {
    PositionNotOnBoard,
    InsertIntoOccupiedPosition,
    RemoveFromEmptyPosition,
    ActionError,
    ReceiveError,
}

/// What happened during a single Step.
#[derive(Debug, Clone, Copy, Default)]
pub struct StepOutcome {
    /// Number of full rows cleared when the piece locked this frame (0 if none).
    pub lines_cleared: usize,
    /// True if the player topped out this frame; the game is over.
    pub topped_out: bool,
    /// True if a piece locked this frame.
    pub locked: bool,
}

pub struct GameManager {
    rx: Receiver<TetrisAction>,
    tx: Sender<(BoardState, StepOutcome)>,
    /// Frames elapsed since the current piece was last forced down by gravity.
    time: u64,
    gravity: u64,
    /// Amount subtracted from `gravity` on a frame where a soft drop is held.
    gravity_decrease: u64,
    /// Frames a piece may rest on the ground before it locks into place.
    lock_delay: u64,
    pub board: Board,
    prev_outcome: StepOutcome,
    bag_randomizer: BagRandomizer<TetrominoType>,
    curr_pos: Position,
    curr_tetromino: Tetromino,
    lines_cleared: usize,
}

impl GameManager {
    pub fn new(
        gravity: u64,
        gravity_decrease: u64,
        lock_delay: u64,
        rx: Receiver<TetrisAction>,
        tx: Sender<(BoardState, StepOutcome)>,
        board: Board,
        mut bag_randomizer: BagRandomizer<TetrominoType>,
    ) -> Self {
        let curr_pos = Self::spawn_position(&board); // refactored this bc is used again
        let curr_tetromino = Tetromino::new(*bag_randomizer.cur());
        // The first piece is now in play, so move past it in the bag.
        bag_randomizer.next();

        Self {
            rx,
            tx,
            time: 0,
            gravity,
            gravity_decrease,
            lock_delay,
            board,
            prev_outcome: StepOutcome{lines_cleared: 0, topped_out: false, locked: false},
            bag_randomizer,
            curr_pos,
            curr_tetromino,
            lines_cleared: 0
        }
    }

    /// Column/row a new piece spawns at Position of horizontally centered, at the top of
    /// the playfield (row `height`).
    fn spawn_position(board: &Board) -> Position {
        Position(((board.width - 1) / 2) as isize, board.height as isize)
    }

    pub fn show(&self) {
        self.board.print();
    }

    /// Only for debugging
    pub fn show_with_tetromino(&self) {
        self.board
            .print_with_tetromino(&self.curr_tetromino, &self.curr_pos);
    }

    pub fn show_as_string(&self) -> String {
        self.board
            .as_string_with_tetromino(&self.curr_tetromino, &self.curr_pos)
    }

    pub fn run(&mut self) -> Result<usize, ManagerError> {
        loop {
            // TODO input action
            let Ok(action) = self.rx.recv() else {
                return Err(ManagerError::ReceiveError);
            };

            if action == TetrisAction::Q {
                break;
            }

            if action == TetrisAction::S {
                // self.show_with_tetromino();
                self.tx.send((self.to_board_state(), self.prev_outcome)).unwrap();
                continue;
            }

            let rst = self.step(action)?;
            self.lines_cleared += rst.lines_cleared;
            self.prev_outcome = rst;
            self.prev_outcome.lines_cleared = self.lines_cleared;
            if rst.topped_out {
                return Ok(self.lines_cleared);
            }
        }
        Ok(self.lines_cleared)
    }

    /// Advances the game by exactly one frame for the given action and return report.
    //  For accomodating Multiplayer!
    /// A coordinator can `step` several boards in lockstep and react to
    // `lines_cleared` / `topped_out` between frames.
    pub fn step(&mut self, action: TetrisAction) -> Result<StepOutcome, ManagerError> {
        // if self.movement(action).is_none() {
        //     for point in self.curr_tetromino.state().0.iter() {
        //         if !self.board.insert(self.curr_pos + point.clone()).unwrap() {
        //             // Game Over
        //             return Ok(());
        //         }
        //     }
        //     self.board.elimination();
        //     self.bag_randomizer.next();
        //     self.curr_tetromino = Tetromino::new(self.bag_randomizer.cur().clone());
        //     self.curr_pos = Position(((self.board.width - 1) / 2) as isize, (self.board.height - 1) as isize);
        //     self.time = 0;
        //     continue;
        // }

        // (2) Perform the action for this frame.
        // Soft (D) lowers gravity & cont;
        // Hard (H) moves piece to the floor and returns None.
        let soft_drop = action == TetrisAction::D;
        if self.movement(action).is_none() {
            return self.lock_piece();
        }

        // (3) Advance gravity / lock delay.
        self.time += 1;

        // if self.time % self.gravity == 0 {
        //     // Move 1 step down
        //     if self.movement(TetrisAction::D).is_none() {
        //         for point in self.curr_tetromino.state().0.iter() {
        //             if !self.board.insert(self.curr_pos + point.clone()).unwrap() {
        //                 return Ok(());
        //             }
        //         }
        //         self.board.elimination();
        //         self.bag_randomizer.next();
        //         self.curr_tetromino = Tetromino::new(self.bag_randomizer.cur().clone(), );
        //         self.curr_pos = Position(((self.board.width - 1) / 2) as isize, (self.board.height - 1) as isize);
        //         self.time = 0;
        //     }
        // }

        let effective_gravity = if soft_drop {
            self.gravity.saturating_sub(self.gravity_decrease).max(1)
        } else {
            self.gravity
        };

        let can_down = self.can_move_down();
        if can_down && self.time >= effective_gravity {
            self.curr_pos += Position(0, -1);
            self.time = 0;
        } else if !can_down && self.time >= self.lock_delay {
            // The piece has rested on the ground for `lock_delay` frames.
            return self.lock_piece();
        }

        Ok(StepOutcome::default())
    }

    /// True if the current piece can fall one more row without hitting the
    /// floor or a filled cell.
    fn can_move_down(&self) -> bool {
        self.is_valid_state(self.curr_tetromino.state(), Position(0, -1))
    }
    /// Fills the current piece into the board.
    // `Ok(true)` if the player topped out (game over)
    // Otherwise clears lines, draws the next piece and returns `Ok(false)`.
    fn lock_piece(&mut self) -> Result<StepOutcome, ManagerError> {
        let cells: Vec<Position> = self
            .curr_tetromino
            .state()
            .0
            .iter()
            .map(|point| self.curr_pos + point.clone())
            .collect();

        let mut topped_out = false;
        for cell in cells {
            match self.board.insert(cell) {
                Ok(within_field) => topped_out |= !within_field,
                // A cell off the grid or already filled means the piece jammed
                // into the stack: treat it as a top-out.
                Err(_) => topped_out = true,
            }
        }

        if topped_out {
            return Ok(StepOutcome { topped_out: true, lines_cleared: 0, locked: true });
        }

        let lines_cleared = self.board.elimination();
        self.spawn_next();
        Ok(StepOutcome { topped_out: false, lines_cleared, locked: true })
    }

    /// Draws the next tetromino from the bag and resets it to the spawn state.
    fn spawn_next(&mut self) {
        self.curr_tetromino = Tetromino::new(*self.bag_randomizer.cur());
        self.bag_randomizer.next();
        self.curr_pos = Self::spawn_position(&self.board);
        self.time = 0;
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
    /// return None if the piece reached the bottom and must lock (hard drop)
    pub fn movement(&mut self, action: TetrisAction) -> Option<bool> {
        match action {
            TetrisAction::C => {
                let offsets = SETTINGS.wall_kick(
                    &self.curr_tetromino.ttype,
                    &self.curr_tetromino.rotation.clockwise(),
                );

                for offset in offsets {
                    let Ok(new_state) = self.curr_tetromino.peek_rotate(action) else {
                        continue;
                    };
                    if self.is_valid_state(&new_state, *offset) {
                        self.curr_tetromino.rotate(action).unwrap();
                        self.curr_pos += *offset;
                        return Some(true);
                    }
                }
                Some(false)
            }
            TetrisAction::W => {
                let offsets = SETTINGS.wall_kick(
                    &self.curr_tetromino.ttype,
                    &self.curr_tetromino.rotation.counter_clockwise(),
                );

                for offset in offsets {
                    let Ok(new_state) = self.curr_tetromino.peek_rotate(action) else {
                        continue;
                    };
                    if self.is_valid_state(&new_state, *offset) {
                        self.curr_tetromino.rotate(action).unwrap();
                        self.curr_pos += *offset;
                        return Some(true);
                    }
                }
                Some(false)
            }
            TetrisAction::D => {
                // Soft drop shouldn't make the piece move down directly; it only
                // lowers gravity for this frame (in run function!).
                // Old impl made D into an instant 1-row drop.
                Some(true)
            }
            TetrisAction::L => {
                let offset = Position(-1, 0);
                if self.is_valid_state(&self.curr_tetromino.state(), offset) {
                    self.curr_pos += offset;
                    return Some(true);
                }
                Some(false)
            }
            TetrisAction::R => {
                let offset = Position(1, 0);

                if self.is_valid_state(&self.curr_tetromino.state(), offset) {
                    self.curr_pos += offset;
                    return Some(true);
                }
                Some(false)
            }
            TetrisAction::E => Some(true),
            TetrisAction::H => {
                let mut offset = Position(0, -1);
                while self.is_valid_state(&self.curr_tetromino.state(), offset) {
                    offset += Position(0, -1);
                }
                self.curr_pos += offset + Position(0, 1);
                None
            }
            TetrisAction::S => panic!(),
            TetrisAction::Q => panic!(),
        }
    }

    pub fn to_board_state(&self) -> crate::bot::BoardState {
        let mut grid = [[crate::bot::CellState::Empty; 10]; 20];
        for r in 0..20.min(self.board.height) {
            for c in 0..10.min(self.board.width) {
                if !self.board.is_empty(&Position(c as isize, r as isize)) {
                    grid[r][c] = crate::bot::CellState::Filled;
                }
            }
        }
        let tetromino = match self.curr_tetromino.ttype {
            TetrominoType::I => crate::bot::TetrominoType::I,
            TetrominoType::J => crate::bot::TetrominoType::J,
            TetrominoType::L => crate::bot::TetrominoType::L,
            TetrominoType::O => crate::bot::TetrominoType::O,
            TetrominoType::S => crate::bot::TetrominoType::S,
            TetrominoType::T => crate::bot::TetrominoType::T,
            TetrominoType::Z => crate::bot::TetrominoType::Z,
        };
        crate::bot::BoardState {
            tetromino_row: self.curr_pos.1 as usize,
            tetromino_col: self.curr_pos.0 as usize,
            tetromino_rotation: self.curr_tetromino.rotation as usize,
            tetromino,
            board: grid,
        }
    }
}
