use std::path::PathBuf;

use p5_tetris::{setting::{SETTINGS, Settings}, tetromino::{Position, Rotation, TetrominoState, TetrominoType}};

#[test]
fn settings_states_test() {
    println!("States: {:#?}", SETTINGS._states());
    let L_state_right = TetrominoState(
        vec![
            Position( 0,  1), 
            Position( 0,  0), 
            Position( 0, -1), 
            Position( 1, -1)
        ]
    );
    assert_eq!(&L_state_right, SETTINGS.state(&TetrominoType::L, &Rotation::Right))
}

#[test]
fn settings_wall_kicks_test() {
    println!("States: {:#?}", SETTINGS._wall_kicks());
    let Normal_wall_kick_right = vec!(
        Position( 0,  0), 
        Position( 1,  0), 
        Position( 1, -1), 
        Position( 0,  2), 
        Position( 1,  2)
    );
    assert_eq!(&Normal_wall_kick_right, SETTINGS.wall_kick(&TetrominoType::L, &Rotation::Right))
}