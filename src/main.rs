use std::sync::mpsc::channel;

use p5_tetris::{bag_randomizer::BagRandomizer, board::Board, manager::GameManager, setting::Settings, tetromino::{TetrisAction, TetrominoType}};

fn main() {
    // a
    let bag = vec![
        TetrominoType::I, 
        TetrominoType::J, 
        TetrominoType::L, 
        TetrominoType::O, 
        TetrominoType::S, 
        TetrominoType::Z, 
        TetrominoType::T
    ];

    let (tx, rx) = channel::<TetrisAction>();

    let seed:[u8; 16] = todo!();
    let bag_randomizer = BagRandomizer::from_parts(bag.into_iter(), seed);
    let board = Board::new(20, 10);
    let settings = Settings::new();

    let manager = GameManager::new(20, settings, rx, board, bag_randomizer);
}
