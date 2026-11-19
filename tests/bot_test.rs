use std::{sync::mpsc::channel, thread::spawn};

use p5_tetris::{
    bag_randomizer::BagRandomizer,
    board::Board,
    bot::{BoardState, Bot, CellState, GoodMultiBot, MultiPlayerBot, SimulatorBot, TetrominoType},
    manager::GameManager,
    setting::SETTINGS,
};
use rand::random;

pub type MySinglePlayerBot = SimulatorBot;
pub type MyMultiPlayerBot = GoodMultiBot;

#[test]
fn create_bot_and_get_any_action_test() {
    let mut bot = MySinglePlayerBot::new();
    let _ = bot.get_move(&BoardState {
        tetromino_row: 20,
        tetromino_col: 4,
        tetromino_rotation: 0,
        tetromino: TetrominoType::J,
        board: [[CellState::Empty; 10]; 20],
    });
}

#[test]
fn single_bot_test() {
    for _ in 0..20 {
        let seed = random(); // random seed
        let (tx, rx) = channel();
        let (tx_outcome, rx_outcome) = channel();
        let board = Board::new(10, 20);
        let bag_randomizer = BagRandomizer::from_parts(SETTINGS.bag().into_iter(), seed);
        let mut manager = GameManager::new(20, 10, 15, rx, tx_outcome, board, bag_randomizer);

        let handle = spawn(move || manager.run());

        let mut bot = MySinglePlayerBot::new();

        loop {
            tx.send(p5_tetris::tetromino::TetrisAction::S).unwrap();
            let Ok(outcome) = rx_outcome.recv() else {
                break;
            };
            // println!("{:?}", outcome.1);

            if outcome.1.topped_out {
                break;
            }
            let action = bot.get_move(&outcome.0);
            tx.send((&action).into()).unwrap();
        }

        let lines_cleared = handle.join().unwrap().unwrap();
        println!("Lines cleared: {}", lines_cleared);
    }
}

#[test]
fn create_mut_bot_and_get_any_action_tescreate_mut_bot_and_get_any_action_testt() {
    let mut bot = MyMultiPlayerBot::new();
    let state = &BoardState {
        tetromino_row: 20,
        tetromino_col: 4,
        tetromino_rotation: 0,
        tetromino: TetrominoType::J,
        board: [[CellState::Empty; 10]; 20],
    };
    let _ = bot.get_move(&state, &[], &state, &[]);
}
