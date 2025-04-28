use cadical::Solver;
use sudoku_clauses::add_hint;
use crate::sudoku_clauses;

pub struct Sudoku{
    board_size: i32,
    board: Vec<Vec<i32>>,
    standard_clauses:Vec<Vec<i32>>,
}

impl Sudoku {
    pub fn new(board_size: i32, clauses: Vec<Vec<i32>>) -> Sudoku {
        assert!(board_size == 4 || board_size == 6 || board_size == 9);
        let mut board = vec![vec![0; board_size as usize]; board_size as usize];
        Sudoku {
            board_size,
            board,
            standard_clauses: clauses
        }
    }

    pub fn unique(mut self, hints:Vec<i32>) -> bool{
        let (solvable, possible_sol) = Self::solvable(&self, &hints);
        if solvable {
            let solution = possible_sol.unwrap();
            let new_sudoku = self.add_hints(&hints);
            let mut clauses = new_sudoku.standard_clauses;
            let mut forbidden:Vec<i32> = Vec::new();
            for var in solution {
                forbidden.push(-var);
            }
            clauses.push(forbidden);
            let mut sat: Solver = Solver::new();
            for clause in &clauses {
                sat.add_clause(clause.clone());
            }
            return !sat.solve().unwrap()
        }
        false
    }

    fn solvable(&self, hints: &Vec<i32>) -> (bool, Option<Vec<i32>>){
        let mut sat: Solver = Solver::new();
        let new_sudoku = Self::add_hints(self, &hints);
        for clause in &new_sudoku.standard_clauses {
            sat.add_clause(clause.clone());
        }
        let solvable = sat.solve().unwrap();
        if solvable{
            let mut solution = Vec::new();
            for var in 1..=sat.max_variable(){
                if sat.value(var) == Some(true){
                    solution.push(var)
                }
            }
            return (solvable, Some(solution))
        }
        (solvable, None)
    }

    fn find_column_row(sudoku: &Sudoku, pos:i32) -> (i32, i32) {
        let row = (pos/sudoku.board_size) + 1;
        let col = (pos%sudoku.board_size) + 1;
        (row,col)
    }

    fn add_hints(&self, hints: &Vec<i32>) -> Sudoku {
        let mut standard_clauses = self.standard_clauses.clone();
        for i in 0..hints.len() {
            if hints[i] > 0 {
                let (row, col) = Self::find_column_row(&self, i as i32);
                standard_clauses = add_hint(&standard_clauses, hints[i], row, col, self.board_size);
            }
        }
        Sudoku{
            board_size: self.board_size,
            board: self.board.clone(),
            standard_clauses,
        }
    }

    pub fn compute_solution(&mut self, solution:&Vec<i32>) {
        let mut res = vec![vec![0; self.board_size as usize]; self.board_size as usize];
        let size = self.board_size;
        for v in solution {
            let r = (v-1) / (size*size) + 1;
            let c = ((v-1) % (size*size)) / size + 1;
            let d = (v-1) % size + 1;
            if 1 <= r && r <= size && 1 <= c && c <= size && 1 <= d && d <= size {
                res[r as usize-1][c as usize-1] = d;
            }
        }
        self.board = res;
    }

    pub fn print_solution(&self){
        for r in 0..self.board_size {
            println!("{:?}", self.board[r as usize]);
        }
    }
}






