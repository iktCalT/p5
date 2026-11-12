use std::sync::mpsc::{self, channel};

use p5_tetris::{bag_randomizer::BagRandomizer, board::Board, manager::GameManager, tetromino::{TetrisAction, TetrominoType}};

fn main() {
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

    let manager = GameManager::new(20, rx, board, bag_randomizer);
}
