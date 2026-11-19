use std::{sync::mpsc::channel, thread::spawn};

use p5_tetris::{
    bag_randomizer::BagRandomizer,
    board::Board,
    bot::{BoardState, CellState, GoodMultiBot, MultiPlayerBot, SimulatorBot, TetrominoType},
    manager::GameManager,
    setting::SETTINGS,
};
use rand::random;

pub type MySinglePlayerBot = SimulatorBot;
pub type MyMultiPlayerBot = GoodMultiBot;


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

