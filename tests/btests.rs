use p5_tetris::manager::ManagerError;
use p5_tetris::{board::Board, tetromino::Position};

#[test]
fn test_board_print() {
    let board = Board::new(10, 20);
    board.print();
}

#[test]
fn test_board_insert() {
    let mut board = Board::new(3, 4);
    let pos = Position(1, 0);
    assert_eq!(true, board.is_empty(&pos));

    board.insert(pos).unwrap();
    assert_eq!(false, board.is_empty(&pos));
    assert_eq!(
        Err(ManagerError::InsertIntoOccupiedPosition),
        board.insert(pos)
    );
    board.print();
}

#[test]
fn test_board_remove() {
    let mut board = Board::new(3, 4);
    let pos = Position(1, 0);
    assert_eq!(true, board.is_empty(&pos));

    board.insert(pos).unwrap();
    assert_eq!(false, board.is_empty(&pos));
    assert_eq!(
        Err(ManagerError::InsertIntoOccupiedPosition),
        board.insert(pos)
    );

    board.remove(pos).unwrap();
    assert_eq!(true, board.is_empty(&pos));

    assert_eq!(
        Err(ManagerError::RemoveFromEmptyPosition),
        board.remove(pos)
    );
    board.print();
}

#[test]
fn test_board_add_garbage_row() {
    let mut board = Board::new(3, 4);
    // Bottom row is filled except for one index column 1.
    board.add_garbage_row(1);

    assert!(!board.is_empty(&Position(0, 0)));
    assert!(board.is_empty(&Position(1, 0)));
    assert!(!board.is_empty(&Position(2, 0)));
    assert!(board.is_empty(&Position(0, 1)));
    board.print();
}

#[test]
fn test_board_add_garbage_row_shifts_up() {
    let mut board = Board::new(3, 4);
    // Place a cell at the bottom left.
    board.insert(Position(0, 0)).unwrap();
    board.add_garbage_row(2);

    // The old bottom-left cell moved up one row.
    assert!(!board.is_empty(&Position(0, 1)));
    // New garbage row at the bottom.
    assert!(!board.is_empty(&Position(0, 0)));
    assert!(!board.is_empty(&Position(1, 0)));
    assert!(board.is_empty(&Position(2, 0)));
    board.print();
}

#[test]
fn test_board_is_topped_out() {
    let mut board = Board::new(3, 4);
    // A cell on top row (row 3) is not top-out.
    board.insert(Position(0, 3)).unwrap();
    assert!(!board.is_topped_out());
    // One garbage row shifts it up, top out.
    board.add_garbage_row(0);
    assert!(board.is_topped_out());
}
