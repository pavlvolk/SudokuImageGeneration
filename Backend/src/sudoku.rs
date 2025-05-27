use cadical::Solver;
use sudoku_clauses::add_hint;
use crate::sudoku_clauses;

#[derive(Clone)]
pub struct Sudoku{
    pub(crate) board_size: i32,
    standard_clauses:Vec<Vec<i32>>
}

impl Sudoku {
    
    /**
    *   This method creates another sudoku with the standard restrictions.
    *   You can reuse this sudoku template with different hints since they will not be added.
    *
    *   @returns Sudoku The created sudoku.
    */
    pub fn new(board_size: i32) -> Sudoku {
        assert!(board_size == 4 || board_size == 6 || board_size == 9);
        let clauses = sudoku_clauses::sudoku_clauses(board_size);
        Sudoku {
            board_size,
            standard_clauses: clauses
        }
    }

    /**
    *   This method calculates whether the sudoku is unique or not given the hints.
    *
    *   @returns (bool, Option<Vec >) Boolean if it is unique and possible solution. 
    */

    pub fn unique(&self, hints: &Vec<usize>, solver: &mut Solver) -> (bool, Option<Vec<i32>>){
        let (solvable, possible_sol) = Self::solvable(&self, &hints, solver);
        if solvable {
            let solution = possible_sol.unwrap();
            let mut forbidden:Vec<i32> = Vec::new();
            for var in &solution {
                forbidden.push(-var);
            }
            solver.add_clause(forbidden);
            let hint_vec = Self::add_hints(&self, &hints);
            let not_unique = solver.solve_with(hint_vec.into_iter());
            return (!not_unique.unwrap(), Some(solution));
        }
        (false, None)
    }

    /**
    *   This method calculates whether the sudoku is solvable given the hints and optionally the solution if it's available.
    *
    *   @returns (bool, Option<Vec<i32>>) Boolean if it is solvable and possible solution.
    */

    pub fn solvable(&self, hints: &Vec<usize>, solver: &mut Solver) -> (bool, Option<Vec<i32>>){
        let hint_iterator = Self::add_hints(&self, &hints).into_iter();
        let solvable = solver.solve_with(hint_iterator).unwrap();
        if solvable{
            let mut solution = Vec::new();
            for var in 1..=solver.max_variable(){
                if solver.value(var) == Some(true){
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

    fn add_hints(&self, hints: &Vec<usize>) -> Vec<i32> {
        let hints_i32 = Self::switch_to_i32(hints);
        let mut hint_clauses:Vec<i32> = Vec::new();
        for i in 0..hints.len() {
            if hints_i32[i] > 0 {
                let (row, col) = Self::find_column_row(&self, i as i32);
                add_hint(&mut hint_clauses, hints_i32[i], row, col, self.board_size);
            }
        }
        hint_clauses
    }
    
    fn switch_to_i32(vec: &Vec<usize>) -> Vec<i32>{
        let mut res:Vec<i32> = Vec::new();
        for value in vec {
            res.push(*value as i32);
        }
        res
    }

    pub fn to_list(vec: &mut Vec<i32>, board_size: &i32) ->Vec<i32> {
        let mut res: Vec<i32> = vec![0; (board_size*board_size) as usize];
        for i in 0..vec.len() {
            let var0 = vec[i]-1;
            let row = var0 / (board_size*board_size);
            let col = (var0 % (board_size * board_size)) / board_size;
            let val = (var0 % board_size) + 1;
            res[(row*board_size + col) as usize] = val;
        }
        res
    }

}






