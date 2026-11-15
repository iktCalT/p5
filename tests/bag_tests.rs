use p5_tetris::{bag_randomizer::BagRandomizer, setting::SETTINGS, tetromino::TetrominoType};

#[test]
fn bag_randomizer_test() {
    let bag = SETTINGS.bag();

    let seed:[u8; 16] = [6, 7, 6, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4];
    let mut bag_randomizer = BagRandomizer::from_parts(bag.into_iter(), seed);
    println!("{:?}", bag_randomizer.cur());
    println!("{:?}", bag_randomizer.cur());
    bag_randomizer.next();
    println!("{:?}", bag_randomizer.cur());
    println!("{:?}", bag_randomizer.cur());
}