use std::collections::VecDeque;
use std::sync::mpsc::channel;

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro128PlusPlus;

use crate::bag_randomizer::BagRandomizer;
use crate::board::Board;
use crate::bot::GarbageLine;
use crate::manager::GameManager;
use crate::setting::SETTINGS;
use crate::tetromino::{TetrisAction, TetrominoType};

/// Config shared by both players
// #[derive(Debug, Clone, Copy)]
// pub struct GameConfig {
//     pub width: usize,
//     pub height: usize,
//     pub gravity: u64,
//     pub gravity_decrease: u64,
//     pub lock_delay: u64,
//     /// Number of ticks a sent garbage block waits
//     pub garbage_queue_time: usize,
//     pub seed: [u8; 16],
// }

pub struct Player {
    pub game: GameManager,
    pub garbage_queue: VecDeque<GarbageLine>,
    /// Per-board RNG for random hole of a garbage.
    garbage_rng: Xoshiro128PlusPlus,
    pub topped_out: bool,
}

impl Player {
    /// Add one garbage row to the bottom. 
    // Return true if this pushed a cell to or above row `height` i.e. top out.
    fn apply_garbage_row(&mut self, hole_col: usize) -> bool {
        self.game.board.add_garbage_row(hole_col);
        self.game.board.is_topped_out()
    }
}

/// Drives two boards in lockstep and resolves garbage
pub struct MultiplayerManager {
    pub players: [Player; 2],
    garbage_queue_time: usize,
}

impl MultiplayerManager {
    pub fn new() -> Self {
        Self {
            players: [Self::make_player(), Self::make_player()],
            garbage_queue_time: SETTINGS.garbage_queue_time,
        }
    }

    fn make_player() -> Player {
        // To satisfy `GameManager::new`.
        let (_tx, rx) = channel::<TetrisAction>();
        let board = Board::new(SETTINGS.width, SETTINGS.height);
        let bag = SETTINGS.bag();
        let bag_randomizer = BagRandomizer::from_parts(bag.into_iter(), SETTINGS.seed);
        let game = GameManager::new(
            SETTINGS.gravity,
            SETTINGS.gravity_decrease,
            SETTINGS.lock_delay,
            rx,
            board,
            bag_randomizer,
        );
        Player {
            game,
            garbage_queue: VecDeque::new(),
            garbage_rng: Xoshiro128PlusPlus::from_seed(SETTINGS.seed),
            topped_out: false,
        }
    }

    /// Advances both boards by one frame and then resolves garbage.
    pub fn tick(&mut self, p1_action: TetrisAction, p2_action: TetrisAction) {
        let l1 = self.step_player(0, p1_action);
        let l2 = self.step_player(1, p2_action);
        self.exchange_garbage(l1, l2);
    }

    /// Steps one player's game, return # lines cleared
    fn step_player(&mut self, idx: usize, action: TetrisAction) -> usize {
        let player = &mut self.players[idx];
        if player.topped_out {
            return 0;
        }
        match player.game.step(action) {
            Ok(outcome) => {
                if outcome.topped_out {
                    player.topped_out = true;
                }
                outcome.lines_cleared
            }
            // Step error stops game
            Err(_) => {
                player.topped_out = true;
                0
            }
        }
    }

    /// Resolves garbage for one tick
    pub fn exchange_garbage(&mut self, l1: usize, l2: usize) {
        // Say player 1 has g1 lines of garbage in their queue, and player 2 has 
        // g2 lines of garbage in their queue.
        let g1 = pending_rows(&self.players[0].garbage_queue);
        let g2 = pending_rows(&self.players[1].garbage_queue);

        // Additionally, say player 1 clears l1 lines and player 2 clears l2 lines.
        // Let s1 = max(l1 - 1, 0), s2 = max(l2 - 1, 0)
        let s1 = l1.saturating_sub(1);
        let s2 = l2.saturating_sub(1);

        // Player 2's queue gets max(s1 - g1, 0) new pieces of garbage tagged with 
        // timer "garbage_queue_time" from config.json and the oldest min(s1, g1) 
        // pieces of pending garbage, not including newly added garbage this tick, 
        // is removed.
        let cancel1 = s1.min(g1);
        let send1 = s1 - cancel1;
        cancel_oldest(&mut self.players[0].garbage_queue, cancel1);
        if send1 > 0 {
            self.players[1].garbage_queue.push_back(GarbageLine {
                num_rows_to_create: send1,
                ticks_till_applied: self.garbage_queue_time,
            });
        }

        // Player 1's queue gets max(s2 - g1, 0) new pieces of garbage tagged with 
        // timer "garbage_queue_time" from config.json and the oldest min(s2, g2) 
        // pieces of pending garbage, not including the newly added garbage this tick, 
        // is removed.
        let cancel2 = s2.min(g2);
        let send2 = s2 - cancel2;
        cancel_oldest(&mut self.players[1].garbage_queue, cancel2);
        if send2 > 0 {
            self.players[0].garbage_queue.push_back(GarbageLine {
                num_rows_to_create: send2,
                ticks_till_applied: self.garbage_queue_time,
            });
        }

        // All garbage in each player's queue tagged with timer equal to 0 has that 
        // garbage removed from the queue and applied to the bottom that player's board,
        //  moving all other rows upwards.
        // For a player, if any cell is above at or above the 20th row (row index >= 20),
        //  that player tops out. No further garbage should be applied to topped out 
        // players.
        self.apply_ready_garbage(0);
        self.apply_ready_garbage(1);

        // All garbage timers are reduced by 1.
        for player in self.players.iter_mut() {
            for line in player.garbage_queue.iter_mut() {
                line.ticks_till_applied = line.ticks_till_applied.saturating_sub(1);
            }
        }
    }

    /// Applies all time out garbage at the front of the queue.
    /// Stops early if player tops out.
    fn apply_ready_garbage(&mut self, idx: usize) {
        let player = &mut self.players[idx];
        if player.topped_out {
            return;
        }
        let width = player.game.board.width;
        loop {
            let ready = player
                .garbage_queue
                .front()
                .map(|line| line.ticks_till_applied == 0)
                .unwrap_or(false);
            if !ready {
                break;
            }
            let block = player.garbage_queue.pop_front().unwrap();
            let hole_col = (player.garbage_rng.next_u32() as usize) % width;
            for _ in 0..block.num_rows_to_create {
                if player.apply_garbage_row(hole_col) {
                    player.topped_out = true;
                    return;
                }
            }
        }
    }
}

/// Total number of pending garbage in queue summed by blocks.
fn pending_rows(queue: &VecDeque<GarbageLine>) -> usize {
    queue.iter().map(|line| line.num_rows_to_create).sum()
}

/// Removes the `n` oldest garbage from queue front.
fn cancel_oldest(queue: &mut VecDeque<GarbageLine>, mut n: usize) {
    while n > 0 {
        let Some(front_rows) = queue.front().map(|line| line.num_rows_to_create) else {
            break;
        };
        if front_rows <= n {
            n -= front_rows;
            queue.pop_front();
        } else {
            if let Some(block) = queue.front_mut() {
                block.num_rows_to_create -= n;
            }
            n = 0;
        }
    }
}
