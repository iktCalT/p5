use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::path::PathBuf;
use std::sync::mpsc::channel;

use p5_tetris::bot::BadMultiBot;
use p5_tetris::bot::GarbageLine;
use p5_tetris::bot::GoodMultiBot;
use p5_tetris::bot::MultiPlayerBot;
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
    let mut goodbot_path: Option<PathBuf> = None;
    let mut badbot_path: Option<PathBuf> = None;

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
            "--list-bots" => {
                let goodbot = GoodMultiBot::new();
                let badbot = BadMultiBot::new();
                println!("{}", goodbot);
                println!("{}", badbot);
            }
            "--trace-bot-run" => {
                for _ in 0..2 {
                    match args.next().expect("missing bot name").as_str() {
                        "good-bot" => goodbot_path = Some(PathBuf::from(args.next().expect("missing good bot path"))),
                        "bad-bot" => badbot_path = Some(PathBuf::from(args.next().expect("missing bad bot path"))),
                        _ => panic!("Invalid arguments")
                    }
                }
                let (Some(goodbot_path), Some(badbot_path)) = (goodbot_path, badbot_path) else {
                    panic!("Invalid arguments");
                };
                let goodbot_file = File::create(goodbot_path).unwrap();
                let badbot_file = File::create(badbot_path).unwrap();
                simulate_multi_bots(goodbot_file, badbot_file);
                return;
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
    let (tx_outcome, rx_outcome) = channel();

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
        tx_outcome,
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

fn simulate_multi_bots(mut goodbot_file: File, mut badbot_file: File) {
    let mut goodbot = GoodMultiBot::new();
    let mut badbot = BadMultiBot::new();

    let mut manager = MultiplayerManager::new();

    while manager.topped_out() == (false, false) {
        let (board1, board2) = manager.boards();

        // TODO: finish manager.garbages()
        // let (garbage1, garbage2) = manager.garbages();
        let (garbage1, garbage2) = (Vec::<GarbageLine>::new(), Vec::<GarbageLine>::new());
        let action1 = goodbot.get_move(&board1, &garbage1, &board2, &garbage2);
        let action2 = badbot.get_move(&board2, &garbage2, &board1, &garbage1);
        manager.tick((&action1).into(), (&action2).into());

        goodbot_file.write_all(&[Into::<u8>::into(action1)]).unwrap();
        badbot_file.write_all(&[Into::<u8>::into(action2)]).unwrap();
    }

    let board1 = manager.players[0].game.show_as_string();
    let board2 = manager.players[1].game.show_as_string();
    let rows1 = board1.lines().take(SPAWN_BUFFER + SETTINGS.height);
    let rows2 = board2.lines().take(SPAWN_BUFFER + SETTINGS.height);
    for (r1, r2) in zip(rows1, rows2) {
        println!("{}    {}", r1, r2);
    }
}
