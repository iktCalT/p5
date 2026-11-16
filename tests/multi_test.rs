use p5_tetris::multiplayer::MultiplayerManager;
use p5_tetris::setting::SETTINGS;
use p5_tetris::tetromino::{Position, TetrisAction};

// fn test_cfg() -> GameConfig {
//     GameConfig {
//         width: 10,
//         height: 20,
//         gravity: 20,
//         gravity_decrease: 10,
//         lock_delay: 15,
//         garbage_queue_time: 10,
//         seed: [6, 7, 6, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
//     }
// }

/// Total garbage rows pending in a player's queue (summed across blocks).
fn rows(mp: &MultiplayerManager, idx: usize) -> usize {
    mp.players[idx].garbage_queue
        .iter()
        .map(|line| line.num_rows_to_create)
        .sum()
}

/// Run `ticks` garbage resolutions with no line clears, ageing the queues.
fn age(mp: &mut MultiplayerManager, ticks: usize) {
    for _ in 0..ticks {
        mp.exchange_garbage(0, 0);
    }
}

fn any_filled(mp: &MultiplayerManager, idx: usize) -> bool {
    let board = &mp.players[idx].game.board;
    (0..20).any(|r| (0..10).any(|c| !board.is_empty(&Position(c, r))))
}

#[test]
fn multiline_clear_sends_garbage() {
    let mut mp = MultiplayerManager::new();
    // p1 clears 4 lines -> s1 = 3 garbage rows to p2.
    mp.exchange_garbage(4, 0);
    assert_eq!(rows(&mp, 0), 0);
    assert_eq!(rows(&mp, 1), 3);
}

#[test]
fn single_line_clear_sends_nothing() {
    let mut mp = MultiplayerManager::new();
    mp.exchange_garbage(1, 1);
    assert_eq!(rows(&mp, 0), 0);
    assert_eq!(rows(&mp, 1), 0);
}

#[test]
fn sending_cancels_own_pending_garbage() {
    let mut mp = MultiplayerManager::new();
    // p2 clears 3 lines -> s2 = 2 pending rows sent to p1.
    mp.exchange_garbage(0, 3);
    assert_eq!(rows(&mp, 0), 2);
    // p1 clears 4 lines -> s1 = 3: cancels its own 2 pending, sends 1 to p2.
    mp.exchange_garbage(4, 0);
    assert_eq!(rows(&mp, 0), 0);
    assert_eq!(rows(&mp, 1), 1);
}

#[test]
fn newly_sent_garbage_is_not_immediately_applied() {
    let mut mp = MultiplayerManager::new();
    mp.exchange_garbage(4, 0);
    // Timer started at garbage_queue_time and was aged once this tick.
    assert_eq!(
        mp.players[1].garbage_queue.front().unwrap().ticks_till_applied,
        SETTINGS.garbage_queue_time - 1
    );
    // Nothing has landed on p2's board yet.
    assert!(!any_filled(&mp, 1));
}

#[test]
fn ready_garbage_applies_with_a_shared_hole_column() {
    let mut mp = MultiplayerManager::new();
    // p2 clears 3 lines -> sends a 2-row aligned block to p1.
    mp.exchange_garbage(0, 3);
    assert_eq!(rows(&mp, 0), 2);
    // Age it until its timer hits 0 and it is applied to p1's board.
    age(&mut mp, SETTINGS.garbage_queue_time);
    assert_eq!(rows(&mp, 0), 0);

    // Both rows are filled except for one shared hole column.
    let board = &mp.players[0].game.board;
    let holes_row0: Vec<usize> = (0..SETTINGS.width)
        .filter(|&c| board.is_empty(&Position(c as isize, 0)))
        .collect();
    let holes_row1: Vec<usize> = (0..SETTINGS.width)
        .filter(|&c| board.is_empty(&Position(c as isize, 1)))
        .collect();
    assert_eq!(holes_row0.len(), 1);
    assert_eq!(holes_row1.len(), 1);
    assert_eq!(holes_row0[0], holes_row1[0]);
}

#[test]
fn garbage_can_top_out_a_player() {
    let mut mp = MultiplayerManager::new();
    // Send more garbage rows than the playfield is tall: once a row is pushed
    // to or above row `height`, the receiving player tops out.
    let send = SETTINGS.height + 1;
    mp.exchange_garbage(0, send + 1); // s2 = send
    age(&mut mp, SETTINGS.garbage_queue_time);
    assert!(mp.players[0].topped_out);
    assert!(mp.players.iter().any(|p| p.topped_out));
}

#[test]
fn tick_hard_drops_lock_pieces_on_both_boards() {
    let mut mp = MultiplayerManager::new();
    mp.tick(TetrisAction::H, TetrisAction::H);
    // A hard drop locks the piece, so both boards now have filled cells.
    assert!(any_filled(&mp, 0));
    assert!(any_filled(&mp, 1));
    assert!(!mp.players.iter().any(|p| p.topped_out));
}
