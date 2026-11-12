use p5_tetris::{board::Board, tetromino::Position};
use p5_tetris::manager::ManagerError;


#[test]
fn test_board_print() {
    let board = Board::new(10, 20);
    board.print();
}

#[test]
fn test_board_insert() {
    let mut board = Board::new(3, 4);
    let pos = Position(0, 0);
    assert_eq!(true, board.is_empty(&pos));

    board.insert(pos).unwrap();
    assert_eq!(false, board.is_empty(&pos));
    assert_eq!(Err(ManagerError::InsertIntoOccupiedPosition), board.insert(pos));
    board.print();
}

#[test]
fn test_board_remove() {
    let mut board = Board::new(3, 4);
    let pos = Position(0, 0);
    assert_eq!(true, board.is_empty(&pos));

    board.insert(pos).unwrap();
    assert_eq!(false, board.is_empty(&pos));
    assert_eq!(Err(ManagerError::InsertIntoOccupiedPosition), board.insert(pos));

    board.remove(pos).unwrap();
    assert_eq!(true, board.is_empty(&pos));

    assert_eq!(Err(ManagerError::RemoveFromEmptyPosition), board.remove(pos));
    board.print();
}