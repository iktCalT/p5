use crate::tetromino;

pub static WIDTH: usize = 10;
pub static HEIGHT: usize = 20;

// pasted from readme
pub enum TetrominoType {
    I, J, L, O, S, Z, T
}

impl Into<tetromino::TetrominoType> for &TetrominoType {
    fn into(self) -> tetromino::TetrominoType {
        match self {
            TetrominoType::I => tetromino::TetrominoType::I,
            TetrominoType::J => tetromino::TetrominoType::J,
            TetrominoType::L => tetromino::TetrominoType::L,
            TetrominoType::O => tetromino::TetrominoType::O,
            TetrominoType::S => tetromino::TetrominoType::S,
            TetrominoType::Z => tetromino::TetrominoType::Z,
            TetrominoType::T => tetromino::TetrominoType::T
        }
    }
}

pub enum TetrisAction {
    C, W, L, R, D, H, E
}

impl Into<tetromino::TetrisAction> for &TetrisAction {
    fn into(self) -> tetromino::TetrisAction {
        match self {
            TetrisAction::C => tetromino::TetrisAction::C,
            TetrisAction::W => tetromino::TetrisAction::W,
            TetrisAction::L => tetromino::TetrisAction::L,
            TetrisAction::R => tetromino::TetrisAction::R,
            TetrisAction::D => tetromino::TetrisAction::D,
            TetrisAction::H => tetromino::TetrisAction::H,
            TetrisAction::E => tetromino::TetrisAction::E
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Filled, Empty
}

pub struct BoardState {
    pub tetromino_row: usize,
    pub tetromino_col: usize,
    pub tetromino_rotation: usize,
    pub tetromino: TetrominoType,
    pub board: [[CellState; WIDTH]; HEIGHT],
}

pub trait SinglePlayerBot {
    /// Create a new bot.
    fn new() -> Self;

    /// Gets an action to perform based on the board state. This should be called
    /// every frame.
    fn get_move(&mut self, board_state: &BoardState) -> TetrisAction {
        // 1. push possible movements into work queue
        // 2. eval the states in each work queue, choose the optimal one
        todo!()
    }

    /// Evaluate the score based on current board
    fn eval(board: &[[CellState; 10]; 20]) -> i32 {
        let mut score: i32 = 0;
        let mut is_prev_empty = true;
        for row in board.iter().rev() {
            let empty = row.iter()
                .filter(|cell| cell == &&CellState::Empty)
                .count();

            // Add 10 if we have an empty row
            if empty == WIDTH {
                score += 10;
                continue;
            }

            is_prev_empty = false;

            if !is_prev_empty { // non-empty row (not first one)
                score -= 10;
                score -= 2 * empty as i32; // punish bubbles
            }
        }
        score
    }
}

pub struct GarbageLine {
    pub num_rows_to_create: usize,
    pub ticks_till_applied: usize,
}

pub trait MultiPlayerBot {
    /// Create a new bot.
    fn new() -> Self;

    /// Gets an action to perform based on the board state. This should be called
    /// every frame.
    fn get_move(
        &mut self,
        p1_state: &BoardState,
        p1_garbage: &[GarbageLine],
        p2_state: &BoardState,
        p2_garbage: &[GarbageLine],
    ) -> TetrisAction;
}