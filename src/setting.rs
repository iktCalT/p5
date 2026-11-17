use std::{collections::HashMap, fs, path::PathBuf, sync::LazyLock, sync::OnceLock};

use crate::tetromino::{Position, Rotation, TetrominoState, TetrominoType, WallKick};

static RESOURCES_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn set_resources_dir(dir: PathBuf) {
    let _ = RESOURCES_DIR.set(dir);
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    Settings::new(
        RESOURCES_DIR
            .get()
            .cloned()
            .unwrap_or_else(|| PathBuf::from("resources")),
    )
});

// Read settings from json files
pub struct Settings {
    pub width: usize,
    pub height: usize,
    pub gravity: u64,
    pub gravity_decrease: u64,
    pub lock_delay: u64,
    pub seed: [u8; 16],
    pub garbage_queue_time: usize,
    bag: Vec<TetrominoType>,
    states: HashMap<TetrominoType, (Vec<TetrominoState>, WallKick)>,
    wall_kicks: HashMap<WallKick, Vec<Vec<Position>>>,
}

impl Settings {
    pub fn new(resources_dir: PathBuf) -> Self {
        // bag
        let mut bag = vec![
            TetrominoType::I,
            TetrominoType::J,
            TetrominoType::L,
            TetrominoType::O,
            TetrominoType::S,
            TetrominoType::T,
            TetrominoType::Z,
        ];
        bag.sort(); // make sure in ascending order

        // Parse config.json
        let config_path = fs::read_to_string(resources_dir.join("config.json"))
            .expect("config.json doesn't exist");
        let config: serde_json::Value =
            serde_json::from_str(&config_path).expect("Failed to parse config.json");

        let width = config["width"].as_u64().unwrap_or(10) as usize;
        let height = config["height"].as_u64().unwrap_or(20) as usize;
        let gravity = config["gravity"].as_u64().unwrap_or(20);
        let gravity_decrease = config["gravity_decrease"].as_u64().unwrap_or(10);
        let lock_delay = config["lock_delay"].as_u64().unwrap_or(15);
        let garbage_queue_time = config["garbage_queue_time"].as_u64().unwrap_or(10) as usize;

        let mut seed = [0u8; 16];
        for (i, v) in config["random_seed"]
            .as_array()
            .expect("Missing random_seed in config.json")
            .iter()
            .take(16)
            .enumerate()
        {
            seed[i] = v.as_u64().expect("Invalid byte in random_seed") as u8;
        }

        // Parse tetrominoes.json
        let tetrominoes_path = fs::read_to_string(resources_dir.join("tetrominoes.json"))
            .expect("tetrominoes.json doesn't exist");
        let tetrominoes: serde_json::Value =
            serde_json::from_str(&tetrominoes_path).expect("Failed to parse tetrominoes.json");

        let mut states: HashMap<TetrominoType, (Vec<TetrominoState>, WallKick)> = HashMap::new();

        for ttype_str in ["i", "j", "l", "o", "s", "z", "t"] {
            states.insert(
                ttype_str.into(),
                (
                    tetrominoes[ttype_str]["states"]
                        .as_array()
                        .expect("no states in tetrominoes.json")
                        .iter()
                        .map(|tstate| {
                            TetrominoState(
                                tstate
                                    .as_array()
                                    .expect("states cannot be vec")
                                    .iter()
                                    .map(|pos| {
                                        Position(
                                            pos.as_array().unwrap()[0].as_i64().unwrap() as isize,
                                            pos.as_array().unwrap()[1].as_i64().unwrap() as isize,
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    tetrominoes[ttype_str]["wall_kick"].as_str().unwrap().into(),
                ),
            );
        }

        // Parse wall_kicks.json
        let wall_kicks_path = fs::read_to_string(resources_dir.join("wall_kicks.json"))
            .expect("wall_kicks.json doesn't exist");
        let wall_kicks_json: serde_json::Value =
            serde_json::from_str(&wall_kicks_path).expect("Failed to parse wall_kicks.json");

        let mut wall_kicks: HashMap<WallKick, Vec<Vec<Position>>> = HashMap::new();

        for wall_kick_str in ["Normal", "I", "O"] {
            let kicks = vec![
                wall_kicks_json[wall_kick_str]["o"].as_array().unwrap(),
                wall_kicks_json[wall_kick_str]["r"].as_array().unwrap(),
                wall_kicks_json[wall_kick_str]["t"].as_array().unwrap(),
                wall_kicks_json[wall_kick_str]["l"].as_array().unwrap(),
            ];

            wall_kicks.insert(
                wall_kick_str.into(),
                kicks
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|pos| {
                                Position(
                                    pos.as_array().unwrap()[0].as_i64().unwrap() as isize,
                                    pos.as_array().unwrap()[1].as_i64().unwrap() as isize,
                                )
                            })
                            .collect::<Vec<Position>>()
                    })
                    .collect(),
            );
        }

        Self {
            width,
            height,
            gravity,
            gravity_decrease,
            lock_delay,
            garbage_queue_time,
            seed,
            bag,
            states,
            wall_kicks,
        }
    }

    pub fn bag(&self) -> Vec<TetrominoType> {
        // Return a copy of bag, because
        // BagRandomizer needs to consume a bag
        self.bag.clone()
    }

    pub fn state<'a>(&'a self, ttype: &TetrominoType, rotation: &Rotation) -> &'a TetrominoState {
        &self.states[ttype].0[*rotation as usize]
    }

    pub fn wall_kick<'a>(
        &'a self,
        ttype: &TetrominoType,
        rotation: &Rotation,
    ) -> &'a Vec<Position> {
        &self.wall_kicks[&self.states[ttype].1][*rotation as usize]
    }

    /// For debugging
    pub fn __states<'a>(&'a self) -> &'a HashMap<TetrominoType, (Vec<TetrominoState>, WallKick)> {
        &self.states
    }

    /// For debugging
    pub fn __wall_kicks<'a>(&'a self) -> &'a HashMap<WallKick, Vec<Vec<Position>>> {
        &self.wall_kicks
    }
}
