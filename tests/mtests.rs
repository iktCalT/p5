use p5_tetris::board::Board;


#[test]
fn test_board_print() {
    let board = Board::new(10, 20);
    board.print();
}

