use p5_tetris::bot::MultiPlayerBot;

pub type MyMultiPlayerBot = MultiPlayerBot;

#[test]
fn create_mut_bot_and_get_any_action_test() {
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
