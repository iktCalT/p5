use std::collections::HashMap;

use crate::tetromino::{Position, TetrominoState, TetrominoType};

// Read settings from json files
pub struct Settings {
    pub states: HashMap<TetrominoType, [TetrominoState; 4]>,
    pub offsets: HashMap<TetrominoType, Vec<Vec<Position>>>,
}
