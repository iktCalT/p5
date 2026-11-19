use p5_tetris::bot::{BoardState, Bot, CellState, GoodMultiBot, MultiPlayerBot, SimulatorBot, TetrominoType};

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
