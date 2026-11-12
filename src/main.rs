use p5_tetris::{bag_randomizer::BagRandomizer, tetromino::TetrominoType};

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

    let seed:[u8; 16] = todo!();
    let bag = BagRandomizer::from_parts(bag.into_iter(), seed);
}
