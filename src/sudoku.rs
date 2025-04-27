use cadical::Solver;
use sudoku_clauses::add_hint;
use crate::sudoku_clauses;

struct Sudoku{
    boardsize: i16,
    board: Vec<Vec<u8>>,
    standard_clauses:Vec<Vec<i16>>,
}

impl Sudoku {
    fn new(board_size: i16, clauses: Vec<Vec<i16>>) -> Sudoku {
        assert!(board_size == 4 || board_size == 6 || board_size == 9);
        let mut board = vec![vec![0; board_size as usize]; board_size as usize];
        Sudoku {
            boardsize: board_size,
            board,
            standard_clauses: clauses
        }
    }
}

pub fn solvable(board_size: i16, hints: Vec<i16>){

}

pub fn add_hints(sudoku: &Sudoku, hints: Vec<i16>) -> Sudoku {
    let mut standard_clauses = sudoku.standard_clauses.clone();
    for i in 0..hints.len() {
        let (row, col) = find_column_row(&sudoku, i as i16);
        standard_clauses = add_hint(&standard_clauses,hints[i], row, col, sudoku.boardsize);
    }
    Sudoku{
        boardsize: sudoku.boardsize.clone(),
        board: sudoku.board.clone(),
        standard_clauses,
    }
}

pub fn find_column_row(sudoku: &Sudoku, pos:i16) -> (i16, i16) {
    let row = pos/sudoku.boardsize + 1;
    let col = pos%sudoku.boardsize + 1;
    (row,col)
}

