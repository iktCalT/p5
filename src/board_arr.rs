use std::fmt::{Display, Write};

use crate::{board::CellState, bot::{HEIGHT, WIDTH}, manager::ManagerError, tetromino::{self, Position, Tetromino}};

/// Number of extra rows above the playfield used as a spawn buffer.
pub const SPAWN_BUFFER: usize = 2;

#[derive(Debug, Clone)]
pub struct BoardArr<'a> (
    pub &'a [[CellState; WIDTH]; HEIGHT]
);

impl<'a> BoardArr<'a> {
    pub fn from(board: &'a [[CellState; WIDTH]; HEIGHT]) -> Self {
        Self(board)
    }

    fn get(&self, position: &Position) -> Option<&CellState> {
        if position.0 < 0 || position.1 < 0 {
            return None;
        }
        let Some(row) = self.0.get(position.1 as usize) else {
            return None;
        };
        row.get(position.0 as usize)
    }

    /// return true if it is a empty position (including topped out)
    /// return false if it is non-empty or the position doesn't exist
    pub fn is_empty(&self, position: &Position) -> bool {
        let Some(pos) = self.get(position) else {
            return position.1 >= HEIGHT as isize;
        };
        *pos == CellState::Empty
    }

    /// Return the number of lines eliminated
    /// But doesn't really modify the board
    pub fn eliminated_lines(&self) -> usize {
        self.0
            .iter()
            .filter(|row| row
                .iter()
                .all(|point| *point == CellState::Filled))
            .count()
    }

    /// True if any filled cell sits at or above row `height` (a top-out).
    /// Locked cells never reach the spawn buffer during normal play, so this
    /// only becomes true after garbage shifts the stack too high.
    pub fn is_topped_out(&self, cells: &Vec<Position>) -> bool {
        for position in cells {
            if position.1 >= HEIGHT as isize {
                return true;
            }
        }
        false
    }

    /// Simulate insertion, but don't modify board
    // `Ok(false)` for if the cell lies above the playfield (a top-out), 
    // `Ok(true)` for normal Fill
    /// Error if the cell is off-grid or already occupied.
    pub fn simulate_insert(&mut self, position: Position) -> Result<bool, ManagerError> {
        if position.1 >= HEIGHT as isize {
            return Ok(false);
        }

        let Some(pos) = self.get(&position) else {
            return Err(ManagerError::PositionNotOnBoard);
        };
        if *pos == CellState::Filled {
            return Err(ManagerError::InsertIntoOccupiedPosition);
        }
        Ok(true)
    }
}
