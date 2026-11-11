use crate::{manager::ManagerError, tetromino::Position};

pub struct Board {
    width: usize,
    height: usize,
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
        for row in &self.grid {
            println!("{}", String::from_utf8(row.clone()).expect("Error row"));
        }
    }

    fn get_mut(&mut self, position: Position) -> Option<&mut u8> {
        let Some(row) = self.grid.get_mut(position.0 as usize) else {
            return None;
        };
        row.get_mut(position.1 as usize)
    }

    pub fn insert(&mut self, position: Position) -> Result<(), ManagerError> {
        let Some(mut pos) = self.get_mut(position) else {
            return Err(ManagerError::PositionNotOnBoard);
        };
        if *pos == b'o' {
            return Err(ManagerError::InsertIntoOccupiedPosition);
        }
        *pos = b'o';
        Ok(())
    }

    pub fn remove(&mut self, position: Position) -> Result<(), ManagerError> {
        let Some(mut pos) = self.get_mut(position) else {
            return Err(ManagerError::PositionNotOnBoard);
        };
        if *pos == b'.' {
            return Err(ManagerError::RemoveFromEmptyPosition);
        }
        *pos = b'.';
        Ok(())
    }
}
