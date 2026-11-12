use std::{sync::mpsc::channel, thread::current};

use p5_tetris::{bag_randomizer::BagRandomizer, board::Board, manager::GameManager, setting::Settings, tetromino::{TetrisAction, TetrominoType}};

#[test]
fn test_manager() {
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

    let seed:[u8; 16] = [6, 7, 6, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4];
    let bag_randomizer = BagRandomizer::from_parts(bag.into_iter(), seed);
    let board = Board::new(10, 20);

    let mut manager = GameManager::new(20, Settings::new(), rx, board, bag_randomizer);

    let handle = std::thread::spawn(move || {
        manager.run().unwrap();
        manager.show();
    });

    tx.send(TetrisAction::H).unwrap();
    tx.send(TetrisAction::H).unwrap();
    tx.send(TetrisAction::H).unwrap();
    tx.send(TetrisAction::Q).unwrap();
    handle.join().unwrap();
}

#[test]
fn test_manager_gravity() {
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

    let seed:[u8; 16] = [6, 7, 6, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4];
    let bag_randomizer = BagRandomizer::from_parts(bag.into_iter(), seed);
    let board = Board::new(10, 20);

    let mut manager = GameManager::new(5, Settings::new(), rx, board, bag_randomizer);

    let handle = std::thread::spawn(move || {
        manager.show_with_tetromino();
        manager.run().unwrap();
        manager.show_with_tetromino();
    });

    tx.send(TetrisAction::E).unwrap();
    tx.send(TetrisAction::E).unwrap();
    tx.send(TetrisAction::E).unwrap();
    tx.send(TetrisAction::E).unwrap();
    tx.send(TetrisAction::E).unwrap();

    tx.send(TetrisAction::E).unwrap();
    tx.send(TetrisAction::E).unwrap();
    tx.send(TetrisAction::E).unwrap();
    tx.send(TetrisAction::E).unwrap();
    tx.send(TetrisAction::E).unwrap();

    tx.send(TetrisAction::Q).unwrap();
    handle.join().unwrap();
}

#[test]
fn test_manager_rotate() {
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

    let seed:[u8; 16] = [6, 7, 6, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4];
    let bag_randomizer = BagRandomizer::from_parts(bag.into_iter(), seed);
    let board = Board::new(10, 20);

    let mut manager = GameManager::new(20, Settings::new(), rx, board, bag_randomizer);

    let handle = std::thread::spawn(move || {
        manager.run().unwrap();
        manager.show_with_tetromino();
    });

    tx.send(TetrisAction::D).unwrap();
    tx.send(TetrisAction::D).unwrap();
    tx.send(TetrisAction::H).unwrap();
    tx.send(TetrisAction::Q).unwrap();
    handle.join().unwrap();
}