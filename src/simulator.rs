use crate::board::CellState;
use crate::board_arr::BoardArr;
use crate::bot::{HEIGHT, WIDTH};
use crate::manager::{ManagerError, StepOutcome};
use crate::setting::SETTINGS;
use crate::tetromino::{self, Position, TetrisAction, Tetromino, TetrominoState, TetrominoType};

struct SimulationResult {
    tetromino: Tetromino,
    position: Position,
    last_step_outcome: Option<StepOutcome>,
}

/// A light-weight GameManager
pub struct Simulator<'a> {
    actions: &'a Vec<TetrisAction>,
    time: u64,
    gravity: u64,
    gravity_decrease: u64,
    lock_delay: u64,
    board: BoardArr<'a>,
    curr_pos: Position,
    curr_tetromino: Tetromino,
}

impl <'a> Simulator<'a> {
    pub fn new(
        actions: &'a Vec<TetrisAction>,
        board: &'a [[CellState; WIDTH]; HEIGHT],
        time: u64,
        position: &Position,
        tetromino: &Tetromino
    ) -> Self {
        
        Self {
            actions,
            time,
            gravity: SETTINGS.gravity,
            gravity_decrease: SETTINGS.gravity_decrease,
            lock_delay: SETTINGS.lock_delay,
            board: BoardArr::from(board),
            curr_pos: position.clone(),
            curr_tetromino: tetromino.clone(),
        }
    }

    /// return None if cannot finish all actions (except for soft/hard drop)
    /// return Some(SimulationResult) otherwise
    pub fn simulate(&mut self) -> Option<SimulationResult> {
        let mut last_step_outcome = None;
        for action in self.actions {
            if action == &TetrisAction::Q {
                panic!("Should not pass TetrisAction::Q into simulator.")
            }

            if action == &TetrisAction::S {
                self.show_with_tetromino();
                continue;
            }

            let Ok(step_result) = self.step(action.clone()) else {
                return None;
            };

            last_step_outcome = Some(step_result);
        }

        Some(SimulationResult{
            tetromino: self.curr_tetromino.clone(),
            position: self.curr_pos,
            last_step_outcome
        })
    }

    /// For debugging
    fn show_with_tetromino(&self) {
        let cells = SETTINGS.state(&self.curr_tetromino.ttype, &self.curr_tetromino.rotation).0
            .iter()
            .map(|offset| *offset + self.curr_pos)
            .collect::<Vec<Position>>();

        for r in (0..HEIGHT).rev() {
            for c in 0..WIDTH {
                if cells.contains(&Position(r as isize, c as isize)) 
                || self.board.0.get(r).unwrap().get(c).unwrap() == &CellState::Filled {
                    print!("o")
                }
            }
            println!()
        }
    }

    /// Advances the game by exactly one frame for the given action and return report. 
    //  For accomodating Multiplayer! 
    /// A coordinator can `step` several boards in lockstep and react to 
    // `lines_cleared` / `topped_out` between frames.
    pub fn step(&mut self, action: TetrisAction) -> Result<StepOutcome, ManagerError> {

        // (2) Perform the action for this frame. 
        // Soft (D) lowers gravity & cont; 
        // Hard (H) moves piece to the floor and returns None.
        let soft_drop = action == TetrisAction::D;
        if self.movement(action).is_none() {
            return self.simulate_lock_piece();
        }

        // (3) Advance gravity / lock delay.
        self.time += 1;

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
            return self.simulate_lock_piece();
        }

        Ok(StepOutcome::default())
    }

    /// True if the current piece can fall one more row without hitting the
    /// floor or a filled cell.
    fn can_move_down(&self) -> bool {
        self.is_valid_state(self.curr_tetromino.state(), Position(0, -1))
    }

    /// Simulate lock piece, but don't modify board
    // `Ok(true)` if the player topped out (game over)
    // Otherwise clears lines, draws the next piece and returns `Ok(false)`.
    fn simulate_lock_piece(&mut self) -> Result<StepOutcome, ManagerError> {
        let cells: Vec<Position> = self
            .curr_tetromino.state().0.iter()
            .map(|point| self.curr_pos + point.clone())
            .collect();

        let mut topped_out = false;
        for cell in cells {
            match self.board.simulate_insert(cell) {
                Ok(within_field) => topped_out |= !within_field,
                // A cell off the grid or already filled means the piece jammed
                // into the stack: treat it as a top-out.
                Err(_) => topped_out = true,
            }
        }

        if topped_out {
            return Ok(StepOutcome { topped_out: true, lines_cleared: 0, locked: true });
        }

        let lines_cleared = self.board.eliminated_lines();

        Ok(StepOutcome { topped_out: false, lines_cleared, locked: true })
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
                let offsets = SETTINGS.wall_kick(&self.curr_tetromino.ttype, &self.curr_tetromino.rotation.clockwise());

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
            },
            TetrisAction::W => {
                let offsets = SETTINGS.wall_kick(&self.curr_tetromino.ttype, &self.curr_tetromino.rotation.counter_clockwise());

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
            },
            TetrisAction::D => {
                // Soft drop shouldn't make the piece move down directly; it only
                // lowers gravity for this frame (in run function!). 
                // Old impl made D into an instant 1-row drop.
                Some(true)
            },
            TetrisAction::L => {
                let offset = Position(-1, 0);
                if self.is_valid_state(&self.curr_tetromino.state(), offset) {
                    self.curr_pos += offset;
                    return Some(true);
                }
                Some(false)
            },
            TetrisAction::R => {
                let offset = Position(1, 0);

                if self.is_valid_state(&self.curr_tetromino.state(), offset) {
                    self.curr_pos += offset;
                    return Some(true);
                }
                Some(false)
            },
            TetrisAction::E => Some(true),
            TetrisAction::H => {
                let mut offset = Position(0, -1);
                while self.is_valid_state(&self.curr_tetromino.state(), offset) {
                    offset += Position(0, -1);
                }
                self.curr_pos += offset + Position(0, 1);
                None
            },
            TetrisAction::S => panic!(),
            TetrisAction::Q => panic!()
        }
    }
}
