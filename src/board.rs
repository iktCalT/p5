use crate::{manager::ManagerError, tetromino::{self, Position, Tetromino}};

#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    grid: Vec<Vec<u8>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![b'.'; width]; height]
        }
    }

    pub fn print(&self) {
        for row in self.grid.iter().rev() {
            println!("{}", String::from_utf8(row.clone()).expect("Error row"));
        }
    }

    /// Is is an expensive function, only used for debugging
    pub fn print_with_tetromino(&self, tetromino: &Tetromino, position: &Position) {
        let mut copy = self.clone();
        for point in tetromino.state.0.iter() {
            copy.insert(position.clone() + point.clone()).unwrap();
        }
        copy.print();
    }

    fn get(&self, position: Position) -> Option<&u8> {
        let Some(row) = self.grid.get(position.1 as usize) else {
            return None;
        };
        row.get(position.0 as usize)
    }

    fn get_mut(&mut self, position: Position) -> Option<&mut u8> {
        let Some(row) = self.grid.get_mut(position.1 as usize) else {
            return None;
        };
        row.get_mut(position.0 as usize)
    }

    /// return true if it is a empty position
    /// return false if it is non-empty or the position doesn't exist
    pub fn is_empty(&self, position: &Position) -> bool {
        let Some(pos) = self.get(*position) else {
            return false;
        };
        *pos == b'.'
    }

    pub fn elimination(&mut self) {
        self.grid.retain(|row| row.iter().any(|point| point == &b'.'));
        let eliminated = self.height - self.grid.len();
        for _ in 0..eliminated {
            self.grid.push(vec![b'.'; self.width]);
        }
    }

    pub fn insert(&mut self, position: Position) -> Result<(), ManagerError> {
        if position.1 >= self.height as isize {
            return Ok(());
        }

        let Some(pos) = self.get_mut(position) else {
            return Err(ManagerError::PositionNotOnBoard);
        };
        if *pos == b'o' {
            return Err(ManagerError::InsertIntoOccupiedPosition);
        }
        *pos = b'o';
        Ok(())
    }

    pub fn remove(&mut self, position: Position) -> Result<(), ManagerError> {
        let Some(pos) = self.get_mut(position) else {
            return Err(ManagerError::PositionNotOnBoard);
        };
        if *pos == b'.' {
            return Err(ManagerError::RemoveFromEmptyPosition);
        }
        *pos = b'.';
        Ok(())
    }
}
