use std::collections::HashMap;

use crate::tetromino::{Position, TetrominoState, TetrominoType};

// Read settings from json files
pub struct Settings {
    states: HashMap<TetrominoType, [TetrominoState; 4]>,
    wall_kicks: HashMap<TetrominoType, Vec<Vec<Position>>>,
}

impl Settings {
    pub fn new() -> Self {
        // TODO: read it from json
        Self { 
            states: HashMap::new(),
            wall_kicks: HashMap::new()
        }
    }
}