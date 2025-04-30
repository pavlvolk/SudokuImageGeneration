use cadical::Solver;
use sudoku_clauses::add_hint;
use crate::sudoku_clauses;
use crate::sudoku_clauses::sudoku_clauses;

pub struct Sudoku{
    board_size: i32,
    standard_clauses:Vec<Vec<i32>>,
}

impl Sudoku {
    
    /**
    *   This method creates another sudoku with the standard restrictions.
    *   You can reuse this sudoku template with different hints since they will not be added.
    */
    pub fn new(board_size: i32) -> Sudoku {
        assert!(board_size == 4 || board_size == 6 || board_size == 9);
        Sudoku {
            board_size,
            standard_clauses: sudoku_clauses::sudoku_clauses(board_size)
        }
    }

    /**
    *   This method calculates whether the sudoku is unique or not given the hints.
    *   @returns bool The boolean if the sudoku is unique.
    */

    pub fn unique(&mut self, hints: &Vec<usize>) -> bool{
        let (solvable, possible_sol) = Self::solvable(&self, &hints);
        if solvable {
            let solution = possible_sol.unwrap();
            let new_sudoku = Self::add_hints(&self, &hints);
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

    /**
    *   This method calculates whether the sudoku is solvable given the hints and optionally the solution if it's available.
    *   @returns (bool, Option<Vec<i32>>) Boolean if it is solvable and possible solution.
    */

    pub fn solvable(&self, hints: &Vec<usize>) -> (bool, Option<Vec<i32>>){
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

    fn add_hints(&self, hints: &Vec<usize>) -> Sudoku {
        let hints_i32 = Self::switch_to_i32(hints);
        let mut clauses = self.standard_clauses.clone();
        for i in 0..hints.len() {
            if hints_i32[i] > 0 {
                let (row, col) = Self::find_column_row(&self, i as i32);
                add_hint(&mut clauses, hints_i32[i], row, col, self.board_size);
            }
        }
        Sudoku{
            board_size: self.board_size,
            standard_clauses: clauses,
        }
    }
    
    fn switch_to_i32(vec: &Vec<usize>) -> Vec<i32>{
        let mut res:Vec<i32> = Vec::new();
        for value in vec {
            res.push(*value as i32);
        }
        res
    }
}






