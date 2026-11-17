use std::env;
use std::fs;
use std::iter::zip;
use std::path::PathBuf;
use std::sync::mpsc::channel;

use p5_tetris::multiplayer::MultiplayerManager;
use p5_tetris::setting::SETTINGS;
use p5_tetris::setting::set_resources_dir;
use p5_tetris::{bag_randomizer::BagRandomizer, board::{Board, SPAWN_BUFFER}, manager::GameManager, tetromino::TetrisAction};

// #[derive(Default, Debug)]
// struct CliArgs {
//     resources: Option<PathBuf>,
//     single_player_input: Option<PathBuf>,
//     multi_player_inputs: Vec<PathBuf>,
//     list_bots: bool,
//     trace_bot_run: Option<(String, PathBuf, String, PathBuf)>,
// }

fn main() {
    let mut sp_path: Option<PathBuf> = None;
    let mut mp_paths: Vec<PathBuf> = Vec::new();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--simulate-inputs-single-player" => {
                let file = args.next().expect("Expected file");
                sp_path = Some(PathBuf::from(file));
            }
            "--simulate-inputs-multi-player" => {
                let file = args.next().expect("Expected file");
                mp_paths.push(PathBuf::from(file));
            }
            "--resources" => {
                let dir = args.next().expect("Expected directory");
                set_resources_dir(PathBuf::from(dir));
            }
            _ => panic!("Unknown arg"),
        }
    }

    if mp_paths.len() > 0 && sp_path.is_some() {
        panic!("Cannot launch both as single and multi player mode.")
    }

    if let Some(sp_path) = sp_path {
        single_player(&sp_path);
    } else {
        multi_player(&mp_paths);
    }
}

fn single_player(input_path: &PathBuf) {
    let (tx, rx) = channel::<TetrisAction>();

    let content = fs::read_to_string(&input_path).expect("error reading file ");
    for c in content.chars().filter(|c| !c.is_ascii_whitespace()) {
        let action: TetrisAction = c.into();
        let _ = tx.send(action);
    }

    let _ = tx.send(TetrisAction::Q);
    drop(tx);

    let bag_randomizer = BagRandomizer::from_parts(SETTINGS.bag().into_iter(), SETTINGS.seed);
    let board = Board::new(SETTINGS.width, SETTINGS.height);

    let mut manager = GameManager::new(
        SETTINGS.gravity,
        SETTINGS.gravity_decrease,
        SETTINGS.lock_delay,
        rx,
        board,
        bag_randomizer,
    );
    let _ = manager.run();
    manager.show_with_tetromino();
}

fn multi_player(input_paths: &Vec<PathBuf>) {
    if input_paths.len() != 2 {
        panic!("Multiplayer mode only supports 2 players");
    }
    let p1_content = fs::read_to_string(&input_paths[0]).expect("error reading file ");
    let p2_content = fs::read_to_string(&input_paths[1]).expect("error reading file ");

    let mut mp = MultiplayerManager::new();

    for (c1, c2) in zip(
        p1_content.chars().filter(|c| !c.is_ascii_whitespace()),
        p2_content.chars().filter(|c| !c.is_ascii_whitespace()),
    ) {
        mp.tick(c1.into(), c2.into());
    }

    // Full 10x22 grid is shown
    let board1 = mp.players[0].game.show_as_string();
    let board2 = mp.players[1].game.show_as_string();
    let rows1 = board1.lines().take(SPAWN_BUFFER + SETTINGS.height);
    let rows2 = board2.lines().take(SPAWN_BUFFER + SETTINGS.height);
    for (r1, r2) in zip(rows1, rows2) {
        println!("{}    {}", r1, r2);
    }
}
