use std::{fs, path::PathBuf, sync::mpsc::channel};

use p5_tetris::{bag_randomizer::BagRandomizer, board::Board, manager::GameManager, setting::SETTINGS, tetromino::{TetrisAction, TetrominoType}};

#[test]
fn single_player_batch_test() {
    let single_player_path = PathBuf::from("cli-tests/single-player");
    for entry in fs::read_dir(single_player_path).expect("path is not a dir") {
        let path = entry.unwrap().path();
        // Skip expectations
        if !path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("txt")) {
            continue;
        }

        // Need to create a new manager, because 
        // manager has no "reset game" function for now
        let bag = SETTINGS.bag();

        let (tx, rx) = channel::<TetrisAction>();

        let seed:[u8; 16] = [6, 7, 6, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4];
        let bag_randomizer = BagRandomizer::from_parts(bag.into_iter(), seed);
        let board = Board::new(10, 20);

        let mut manager = GameManager::new(20, 10, 15, rx, board, bag_randomizer);

        println!("Current file: {:?}", path.file_name());
        let handle = std::thread::spawn(move || {
            manager.run().unwrap();
            manager.show_as_string()
        });
        
        let instructions = fs::read(path.clone()).unwrap();

        for instruction in instructions.iter().filter(|c| **c != b'\n') {
            tx.send(instruction.clone().into()).unwrap();
        }
        tx.send(TetrisAction::Q).unwrap(); // Quit
        
        let result = handle.join().unwrap();
        println!("Result: \n{}", result);
        let expected_path = path.with_extension("expect");
        let expected = fs::read_to_string(expected_path).expect("expected file {expected_path:?} doesn't exist");

        assert_eq!(result.trim(), expected.trim());
    }
}