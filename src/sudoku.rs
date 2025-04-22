use cadical::Solver;

struct Sudoku{
    boardsize: usize,
    board: Vec<Vec<u8>>,
    standard_clauses:Vec<i16>,
}

impl Sudoku {
    fn new(boardsize: usize) -> Sudoku {
        assert!(boardsize == 4 || boardsize == 6 || boardsize == 9);
        let mut board = vec![vec![0; boardsize]; boardsize];
        Sudoku{
            boardsize: boardsize,
            board: board,
            standard_clauses: Vec::new()}

    }
}
