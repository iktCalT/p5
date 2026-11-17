use std::fmt::{Display, Write};

use crate::{
    manager::ManagerError,
    tetromino::{Position, Tetromino},
};

/// Number of extra rows above the playfield used as a spawn buffer.
pub const SPAWN_BUFFER: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Filled,
    Empty,
}

impl Into<char> for &CellState {
    fn into(self) -> char {
        match self {
            CellState::Empty => '.',
            CellState::Filled => 'o',
        }
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    /// Height of the playing field. Rows at or above this are the spawn buffer.
    pub height: usize,
    grid: Vec<Vec<CellState>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![CellState::Empty; width]; height + SPAWN_BUFFER],
        }
    }

    pub fn print(&self) {
        print!("{}", self.as_string())
    }

    /// Is is an expensive function, only used for debugging
    pub fn print_with_tetromino(&self, tetromino: &Tetromino, position: &Position) {
        let mut copy = self.clone();
        for point in tetromino.state().0.iter() {
            if let Some(cell) = copy.get_mut(*position + point.clone()) {
                *cell = CellState::Filled;
            }
        }
        copy.print();
    }

    /// Is is an expensive function, only used for debugging
    pub fn as_string_with_tetromino(&self, tetromino: &Tetromino, position: &Position) -> String {
        let mut copy = self.clone();
        for point in tetromino.state().0.iter() {
            if let Some(cell) = copy.get_mut(*position + point.clone()) {
                *cell = CellState::Filled;
            }
        }
        copy.as_string()
    }

    pub fn as_string(&self) -> String {
        let mut string = String::new();
        for row in self.grid.iter().rev() {
            let s: String = row.iter().map(|s| Into::<char>::into(s)).collect();
            string.insert_str(string.len(), s.as_str());
            string.insert(string.len(), b'\n' as char);
        }
        string
    }

    fn get(&self, position: &Position) -> Option<&CellState> {
        if position.0 < 0 || position.1 < 0 {
            return None;
        }
        let Some(row) = self.grid.get(position.1 as usize) else {
            return None;
        };
        row.get(position.0 as usize)
    }

    fn get_mut(&mut self, position: Position) -> Option<&mut CellState> {
        let Some(row) = self.grid.get_mut(position.1 as usize) else {
            return None;
        };
        row.get_mut(position.0 as usize)
    }

    /// return true if it is a empty position
    /// return false if it is non-empty or the position doesn't exist
    pub fn is_empty(&self, position: &Position) -> bool {
        if position.0 < 0 || position.1 < 0 {
            return false;
        }
        let Some(pos) = self.get(position) else {
            return false;
        };
        *pos == CellState::Empty
    }

    /// Removes every filled row and return # rows cleared.
    pub fn elimination(&mut self) -> usize {
        // Restore against the grid length (height + SPAWN_BUFFER = 22), not playing field length
        let rows_before = self.grid.len();
        self.grid
            .retain(|row| row.iter().any(|point| *point == CellState::Empty));
        // let eliminated = self.height - self.grid.len();
        let eliminated = rows_before - self.grid.len();
        for _ in 0..eliminated {
            self.grid.push(vec![CellState::Empty; self.width]);
        }
        eliminated
    }

    /// Adds one garbage row at the bottom of the board.
    pub fn add_garbage_row(&mut self, hole_col: usize) {
        // Drop the top row, then push a new bottom row in.
        self.grid.pop();
        let mut row = vec![CellState::Filled; self.width];
        if hole_col < self.width {
            row[hole_col] = CellState::Empty;
        }
        self.grid.insert(0, row);
    }

    /// True if any filled cell sits at or above row `height` (a top-out).
    /// Locked cells never reach the spawn buffer during normal play, so this
    /// only becomes true after garbage shifts the stack too high.
    pub fn is_topped_out(&self) -> bool {
        self.grid
            .iter()
            .skip(self.height)
            .any(|row| row.iter().any(|cell| *cell == CellState::Filled))
    }

    /// Marks the cell at `position` as Filled.
    // `Ok(false)` for if the cell lies above the playfield (a top-out),
    // `Ok(true)` for normal Fill
    /// Error if the cell is off-grid or already occupied.
    pub fn insert(&mut self, position: Position) -> Result<bool, ManagerError> {
        if position.1 >= self.height as isize {
            return Ok(false);
        }

        let Some(pos) = self.get_mut(position) else {
            return Err(ManagerError::PositionNotOnBoard);
        };
        if *pos == CellState::Filled {
            return Err(ManagerError::InsertIntoOccupiedPosition);
        }
        *pos = CellState::Filled;
        Ok(true)
    }

    pub fn remove(&mut self, position: Position) -> Result<(), ManagerError> {
        let Some(pos) = self.get_mut(position) else {
            return Err(ManagerError::PositionNotOnBoard);
        };
        if *pos == CellState::Empty {
            return Err(ManagerError::RemoveFromEmptyPosition);
        }
        *pos = CellState::Empty;
        Ok(())
    }
}
