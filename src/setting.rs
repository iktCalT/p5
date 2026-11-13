use std::{collections::HashMap, fs, path::PathBuf, sync::LazyLock};

use crate::tetromino::{Position, Rotation, TetrominoState, TetrominoType, WallKick};

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    Settings::new(PathBuf::from("resources"))
});

// Read settings from json files
pub struct Settings {
    states: HashMap<TetrominoType, (Vec<TetrominoState>, WallKick)>,
    wall_kicks: HashMap<WallKick, Vec<Vec<Position>>>,
}

impl Settings {
    pub fn new(resources_dir: PathBuf) -> Self {
        // Parse tetrominoes.json
        let tetrominoes_path = fs::read_to_string(resources_dir.join("tetrominoes.json")).expect("tetrominoes.json doesn't exist");
        let tetrominoes: serde_json::Value = serde_json::from_str(&tetrominoes_path).expect("Failed to parse tetrominoes.json");

        let mut states: HashMap<TetrominoType, (Vec<TetrominoState>, WallKick)> = HashMap::new();

        for ttype_str in ["i", "j", "l", "o", "s", "z", "t"] {
            states.insert(ttype_str.into(), (
                tetrominoes[ttype_str]["states"]
                    .as_array()
                    .expect("no states in tetrominoes.json")
                    .iter()
                    .map(|tstate| TetrominoState(tstate
                        .as_array()
                        .expect("states cannot be vec")
                        .iter()
                        .map(|pos| Position(
                            pos.as_array().unwrap()[0].as_i64().unwrap() as isize, 
                            pos.as_array().unwrap()[1].as_i64().unwrap() as isize))
                        .collect()))
                    .collect(),
                tetrominoes[ttype_str]["wall_kick"].as_str().unwrap().into()
            ));
        }

        // Parse wall_kicks.json
        let wall_kicks_path = fs::read_to_string(resources_dir.join("wall_kicks.json")).expect("wall_kicks.json doesn't exist");
        let wall_kicks_json: serde_json::Value = serde_json::from_str(&wall_kicks_path).expect("Failed to parse wall_kicks.json");

        let mut wall_kicks: HashMap<WallKick, Vec<Vec<Position>>> = HashMap::new();

        for wall_kick_str in ["Normal", "I", "O"] {
            let kicks = vec![
                wall_kicks_json[wall_kick_str]["o"].as_array().unwrap(),
                wall_kicks_json[wall_kick_str]["r"].as_array().unwrap(),
                wall_kicks_json[wall_kick_str]["t"].as_array().unwrap(),
                wall_kicks_json[wall_kick_str]["l"].as_array().unwrap(),
            ];
    
            wall_kicks.insert(wall_kick_str.into(),
                kicks
                    .iter()
                    .map(|row| row
                        .iter()
                        .map(|pos| Position(
                            pos.as_array().unwrap()[0].as_i64().unwrap() as isize, 
                            pos.as_array().unwrap()[1].as_i64().unwrap() as isize))
                        .collect::<Vec<Position>>())
                    .collect()
            );
        }

        Self {
            states,
            wall_kicks
        }
    }

    pub fn state<'a>(&'a self, ttype: &TetrominoType, rotation: &Rotation) -> &'a TetrominoState {
        &self.states[ttype].0[*rotation as usize]
    }

    pub fn wall_kick<'a>(&'a self, ttype: &TetrominoType, rotation: &Rotation) -> &'a Vec<Position> {
        &self.wall_kicks[&self.states[ttype].1][*rotation as usize]
    }

    /// For debugging
    pub fn _states<'a>(&'a self) -> &'a HashMap<TetrominoType, (Vec<TetrominoState>, WallKick)> {
        &self.states
    }

    /// For debugging
    pub fn _wall_kicks<'a>(&'a self) -> &'a HashMap<WallKick, Vec<Vec<Position>>> {
        &self.wall_kicks
    }
}