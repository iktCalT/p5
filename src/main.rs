use std::sync::mpsc::channel;
use std::env;
use std::fs;
use std::path::PathBuf;

use p5_tetris::{bag_randomizer::BagRandomizer, board::Board, manager::GameManager, setting::Settings, tetromino::{TetrisAction, TetrominoType}};

#[derive(Default, Debug)]
struct CliArgs {
    resources: Option<PathBuf>,
    single_player_input: Option<PathBuf>,
    multi_player_inputs: Vec<PathBuf>,
    list_bots: bool,
    trace_bot_run: Option<(String, PathBuf, String, PathBuf)>,
}

fn main() {
    let mut input_path: Option<PathBuf> = None;
    let mut resources_dir = PathBuf::from("resources");

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--simulate-inputs-single-player" => {
                let file = args.next().expect("Expected file");
                input_path = Some(PathBuf::from(file));
            }
            "--resources" => {
                let dir = args.next().expect("Expected directory");
                resources_dir = PathBuf::from(dir);
            }
            other => panic!("Unknown arg"),
        }
    }

    let input_path = input_path.expect("Missing input path for single player inputs");

    let config_str = fs::read_to_string(resources_dir.join("config.json")).expect("Failed to read config.json");
    let config: serde_json::Value = serde_json::from_str(&config_str).expect("failed to parse config.json");

    let width = config["width"].as_u64().unwrap_or(10) as usize;
    let height = config["height"].as_u64().unwrap_or(20) as usize;
    let gravity = config["gravity"].as_u64().unwrap_or(20);

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

    let bag = vec![
        TetrominoType::I, 
        TetrominoType::J, 
        TetrominoType::L, 
        TetrominoType::O, 
        TetrominoType::S, 
        TetrominoType::T, 
        TetrominoType::Z
    ];

    let (tx, rx) = channel::<TetrisAction>();

    let content = fs::read_to_string(&input_path).expect("error reading file ");
    for c in content.chars().filter(|c| !c.is_ascii_whitespace()) {
        let action = match c {
            'C' => TetrisAction::C,
            'W' => TetrisAction::W,
            'L' => TetrisAction::L,
            'R' => TetrisAction::R,
            'D' => TetrisAction::D,
            'H' => TetrisAction::H,
            'E' => TetrisAction::E,
            other => panic!("invalid input move"),
        };
        let _ = tx.send(action);
    }

    let _ = tx.send(TetrisAction::Q);
    drop(tx);

    let bag_randomizer = BagRandomizer::from_parts(bag.into_iter(), seed);
    let board = Board::new(width, height);
    let settings = Settings::new();

    let mut manager = GameManager::new(gravity, settings, rx, board, bag_randomizer);
    let _ = manager.run();
    manager.show_with_tetromino();
}
