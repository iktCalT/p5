use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::{
    Arc, Condvar, Mutex, RwLock,
    mpsc::{self, Receiver},
};
use std::thread::{self, JoinHandle};

use crate::setting::SETTINGS;
use crate::tetromino::{self, Position, Rotation};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
const SPAWN_BUFFER: isize = 2;
const NUM_WORKERS: usize = 4;
const ROT_PLANS: [&[TetrisAction]; 4] = [
    &[],
    &[TetrisAction::C],
    &[TetrisAction::C, TetrisAction::C],
    &[TetrisAction::W],
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
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
            TetrominoType::T => tetromino::TetrominoType::T,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TetrisAction {
    C,
    W,
    L,
    R,
    D,
    H,
    E,
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
            TetrisAction::E => tetromino::TetrisAction::E,
        }
    }
}

impl Into<TetrisAction> for &tetromino::TetrisAction {
    fn into(self) -> TetrisAction {
        match self {
            tetromino::TetrisAction::C => TetrisAction::C,
            tetromino::TetrisAction::W => TetrisAction::W,
            tetromino::TetrisAction::L => TetrisAction::L,
            tetromino::TetrisAction::R => TetrisAction::R,
            tetromino::TetrisAction::D => TetrisAction::D,
            tetromino::TetrisAction::H => TetrisAction::H,
            tetromino::TetrisAction::E => TetrisAction::E,
            _ => panic!(),
        }
    }
}

impl Into<u8> for TetrisAction {
    fn into(self) -> u8 {
        match self {
            TetrisAction::C => b'C',
            TetrisAction::W => b'W',
            TetrisAction::L => b'L',
            TetrisAction::R => b'R',
            TetrisAction::D => b'D',
            TetrisAction::H => b'H',
            TetrisAction::E => b'E',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Filled,
    Empty,
}

#[derive(Clone, Copy)]
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
    fn get_move(&mut self, board_state: &BoardState) -> TetrisAction;
}

pub use SinglePlayerBot as Bot;
/**
 * SPMC queue for main thread queuing jobs to consumer threads to simulate and eval
 */
#[derive(Clone, Copy)]
struct Job {
    id: usize,
    rot_idx: usize,
    dx: isize,
}

struct SpmcQueue {
    q: VecDeque<Job>,
    shutdown: bool,
}

pub struct SimulatorBot {
    shared_board: Arc<RwLock<BoardState>>,
    moves_to_sim: Arc<(Mutex<SpmcQueue>, Condvar)>, //
    scores_rx: Receiver<(usize, f64)>,              // mpsc to receive scores from threads
    workers: Vec<JoinHandle<()>>,                   // thread pool
    plan: VecDeque<TetrisAction>,                   // actual plan
}

impl SimulatorBot {
    fn usize_to_rotation(rotation_type: usize) -> Rotation {
        match rotation_type % 4 {
            0 => Rotation::Origin,
            1 => Rotation::Right,
            2 => Rotation::Inverse,
            3 => Rotation::Left,
            _ => panic!(),
        }
    }

    fn is_valid_pos(
        board: &[[CellState; WIDTH]; HEIGHT],
        cells: &[Position],
        pos: Position,
    ) -> bool {
        cells.iter().all(|p| {
            // new cell position
            let x = pos.0 + p.0;
            let y = pos.1 + p.1;

            if x < 0 || x >= WIDTH as isize || y < 0 || y >= (HEIGHT as isize + SPAWN_BUFFER) {
                return false;
            }

            // true if in buffer range or cell is empty in baord
            y >= HEIGHT as isize || board[y as usize][x as usize] == CellState::Empty
        })
    }

    /**
     * Given a candidate rotation and start, find cells where it would drop if possible
     */
    fn find_landing_cells(
        state: &BoardState,
        rot_idx: usize,
        dx: isize,
    ) -> Option<[(usize, usize); 4]> {
        let ttype: tetromino::TetrominoType = (&state.tetromino).into();
        let mut cur_rot = Self::usize_to_rotation(state.tetromino_rotation);
        let mut cur_pos = Position(state.tetromino_col as isize, state.tetromino_row as isize);

        // check validity of rotations with kicks
        for &op in ROT_PLANS[rot_idx] {
            let next_rot = match op {
                TetrisAction::C => cur_rot.clockwise(),
                TetrisAction::W => cur_rot.counter_clockwise(),
                _ => unreachable!(),
            };
            let next_cells = &SETTINGS.state(&ttype, &next_rot).0;
            let from_kicks = SETTINGS.wall_kick(&ttype, &cur_rot);
            let to_kicks = SETTINGS.wall_kick(&ttype, &next_rot);

            let offset = from_kicks
                .iter()
                .zip(to_kicks.iter())
                .map(|(f, t)| Position(f.0 - t.0, f.1 - t.1))
                .find(|&off| Self::is_valid_pos(&state.board, next_cells, cur_pos + off))?;

            cur_pos += offset;
            cur_rot = next_rot;
        }

        // check non rotated validity
        let cells = &SETTINGS.state(&ttype, &cur_rot).0;
        if !Self::is_valid_pos(&state.board, cells, cur_pos) {
            return None;
        }

        // perform dx slides
        let step = dx.signum();
        for _ in 0..dx.abs() {
            let next_pos = cur_pos + Position(step, 0);
            if !Self::is_valid_pos(&state.board, cells, next_pos) {
                return None;
            }
            cur_pos = next_pos;
        }

        // Drop until frontier
        while Self::is_valid_pos(&state.board, cells, cur_pos + Position(0, -1)) {
            cur_pos.1 -= 1;
        }

        // get final coordinates
        let mut landed = [(0usize, 0usize); 4];
        for (i, p) in cells.iter().enumerate() {
            let x = cur_pos.0 + p.0;
            let y = cur_pos.1 + p.1;
            if x < 0 || x >= WIDTH as isize || y < 0 || y >= HEIGHT as isize {
                return None;
            }
            landed[i] = (x as usize, y as usize);
        }
        Some(landed)
    }

    /**
     * given the candidate landing spot, compute score
     *
     * score: A1 *number of cells filled below the peak
     */
    // fn eval(board: &[[CellState; WIDTH]; HEIGHT], landed: &[(usize, usize); 4]) -> f64 {
    //     // tunable params
    //     let A1 = 10;
    //     let A2 = 20;

    //     let old_peak: isize = (0..HEIGHT)
    //         .rev()
    //         .find(|&row| board[row].iter().any(|&cell| cell == CellState::Filled))
    //         .map_or(-1, |row| row as isize);

    //     let mut score: i32 = 0;
    //     for &(_, row) in landed {
    //         if (row as isize) <= old_peak {
    //             let diff = old_peak - row as isize + 1;
    //             score += A1 * (diff * diff) as i32;
    //         }
    //     }

    //     for (col, row) in landed {
    //         let mut cur_row = *row as isize - 1;
    //         while cur_row >= 0 && !landed.contains(&(*col, cur_row as usize)) && board[cur_row as usize][*col] == CellState::Empty {
    //             cur_row = cur_row - 1;
    //             score -= A2;
    //         }
    //     }

    //     score as f64
    //     // Add additional heuristics if this doesn't do well enough ...
    // }

    // https://codemyroad.wordpress.com/2013/04/14/tetris-ai-the-near-perfect-player/?utm_source=chatgpt.com
    fn eval(board: &[[CellState; WIDTH]; HEIGHT], landed: &[(usize, usize); 4]) -> f64 {
        // tunable params
        const A: f64 = -0.510066;
        const B: f64 = 0.760666;
        const C: f64 = -0.356630;
        const D: f64 = -0.184483;

        let mut aggregate_height: usize = 0;
        let mut cleared_lines: usize = 0;
        let mut holes: usize = 0;
        let mut bumpiness: usize = 0;

        let mut heights: [usize; WIDTH] = [0; WIDTH];

        for row in (0..HEIGHT).rev() {
            let mut clear = true;
            for col in 0..WIDTH {
                let filled = board[row][col] == CellState::Filled || landed.contains(&(col, row));
                if !filled {
                    clear = false;
                    if heights[col] != 0 {
                        holes += 1;
                    }
                }
                if filled && heights[col] == 0 {
                    heights[col] = row + 1;
                }
            }
            if clear {
                cleared_lines += 1;
            }
        }

        for i in 0..WIDTH {
            aggregate_height += heights[i];
            if i >= 1 {
                bumpiness += heights[i].abs_diff(heights[i - 1]);
            }
        }

        let score = A * aggregate_height as f64
            + B * cleared_lines as f64
            + C * holes as f64
            + D * bumpiness as f64;

        score
    }

    fn build_plan(rot_idx: usize, dx: isize) -> Vec<TetrisAction> {
        let mut seq = Vec::new();
        seq.extend_from_slice(ROT_PLANS[rot_idx]);
        let shift = if dx < 0 {
            TetrisAction::L
        } else {
            TetrisAction::R
        };
        for _ in 0..dx.abs() {
            seq.push(shift);
        }

        seq.push(TetrisAction::H);
        seq
    }
}

impl SinglePlayerBot for SimulatorBot {
    fn new() -> Self {
        let init_board = BoardState {
            tetromino_row: HEIGHT,
            tetromino_col: (WIDTH - 1) / 2,
            tetromino_rotation: 0,
            tetromino: TetrominoType::I,
            board: [[CellState::Empty; WIDTH]; HEIGHT],
        };

        let shared_board = Arc::new(RwLock::new(init_board));
        let moves_to_sim = Arc::new((
            Mutex::new(SpmcQueue {
                q: VecDeque::new(),
                shutdown: false,
            }),
            Condvar::new(),
        ));

        let (scores_tx, scores_rx) = mpsc::channel::<(usize, f64)>();

        let mut workers = Vec::with_capacity(NUM_WORKERS);
        for _ in 0..NUM_WORKERS {
            // each worker gets a board ref,
            let board_ref = Arc::clone(&shared_board);
            let queue_ref = Arc::clone(&moves_to_sim);
            let tx = scores_tx.clone();

            // push worker that waits on jobs in moves to sim, sending back score for the job id
            workers.push(thread::spawn(move || {
                loop {
                    let job = {
                        let (lock, cvar) = &*queue_ref;
                        let mut guard = lock.lock().unwrap();
                        //
                        while guard.q.is_empty() && !guard.shutdown {
                            // wait for job
                            guard = cvar.wait(guard).unwrap();
                        }
                        // break if shutdown(no jobs left for this move)
                        if guard.shutdown && guard.q.is_empty() {
                            break;
                        }

                        guard.q.pop_front().unwrap()
                    };
                    let score = {
                        let state_ref = board_ref.read().unwrap();
                        match SimulatorBot::find_landing_cells(&state_ref, job.rot_idx, job.dx) {
                            Some(landed) => SimulatorBot::eval(&state_ref.board, &landed),
                            None => f64::MIN,
                        }
                    };

                    if tx.send((job.id, score)).is_err() {
                        break;
                    }
                }
            }));
        }
        Self {
            shared_board,
            moves_to_sim,
            scores_rx,
            workers,
            plan: VecDeque::new(),
        }
    }

    fn get_move(&mut self, board_state: &BoardState) -> TetrisAction {
        if board_state.tetromino_row == HEIGHT
            && board_state.tetromino_col == (WIDTH - 1) / 2
            && board_state.tetromino_rotation == 0
        {
            self.plan.clear();
        }

        if self.plan.is_empty() {
            *self.shared_board.write().unwrap() = *board_state;

            // generate moves
            let max_rot = if board_state.tetromino == TetrominoType::O {
                1
            } else {
                4
            };
            let mut candidates = Vec::with_capacity(max_rot * 13);
            for rot_idx in 0..max_rot {
                for dx in -6..=6 {
                    candidates.push((rot_idx, dx));
                }
            }

            // push the jobs
            {
                let (lock, cvar) = &*self.moves_to_sim;
                let mut guard = lock.lock().unwrap();
                for (id, &(rot_idx, dx)) in candidates.iter().enumerate() {
                    guard.q.push_back(Job { id, rot_idx, dx });
                }
                cvar.notify_all();
            }

            // process all the scores for all the candidates
            let mut best_id = 0usize;
            let mut best_score = f64::MIN;
            for _ in 0..candidates.len() {
                if let Ok((id, score)) = self.scores_rx.recv() {
                    if score > best_score || (score == best_score && id < best_id) {
                        best_score = score;
                        best_id = id;
                    }
                }
            }

            // no valid moves
            if best_score == f64::MIN {
                self.plan.push_back(TetrisAction::H);
            } else {
                // extend with best move
                let (win_rot, win_dx) = candidates[best_id];
                self.plan.extend(Self::build_plan(win_rot, win_dx));
            }
        }

        self.plan.pop_front().unwrap_or(TetrisAction::H)
    }
}
impl Drop for SimulatorBot {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.moves_to_sim;
        lock.lock().unwrap().shutdown = true;
        cvar.notify_all();
        for w in self.workers.drain(..) {
            let _ = w.join();
        }
    }
}

/// Another bot
// struct SecondSimulatorBot {
//     workers: Vec<JoinHandle<()>>,
// }

// impl SecondSimulatorBot {
//     fn worker_start() {

//     }

//     /// https://codemyroad.wordpress.com/2013/04/14/tetris-ai-the-near-perfect-player/?utm_source=chatgpt.com
//     fn eval() {

//     }
// }

// impl SinglePlayerBot for SecondSimulatorBot {
//     fn new() -> Self {
//         Self {
//             workers: Vec::new()
//         }
//     }

//     fn get_move(&mut self, board_state: &BoardState) -> TetrisAction {

//     }
// }

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

pub struct GoodMultiBot {
    bot: SimulatorBot,
}

impl Display for GoodMultiBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "good-bot")
    }
}

impl MultiPlayerBot for GoodMultiBot {
    fn new() -> Self {
        Self {
            bot: SimulatorBot::new(),
        }
    }

    fn get_move(
        &mut self,
        p1_state: &BoardState,
        _p1_garbage: &[GarbageLine],
        _p2_state: &BoardState,
        _p2_garbage: &[GarbageLine],
    ) -> TetrisAction {
        self.bot.get_move(p1_state)
    }
}

pub struct BadMultiBot {}

impl Display for BadMultiBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad-bot")
    }
}

impl MultiPlayerBot for BadMultiBot {
    fn new() -> Self {
        Self {}
    }

    fn get_move(
        &mut self,
        _p1_state: &BoardState,
        _p1_garbage: &[GarbageLine],
        _p2_state: &BoardState,
        _p2_garbage: &[GarbageLine],
    ) -> TetrisAction {
        TetrisAction::H
    }
}
