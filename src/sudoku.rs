use cadical::Solver;

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

