use std::time::Instant;
use cadical::Solver;
use crate::fill_grid::fill_grid;
use crate::sudoku;
use crate::sudoku::Sudoku;
use crate::sudoku_clauses::sudoku_clauses;

/**
*   This function takes hints and a sudoku of a specific size and outputs a solution if there is a definite one.
*   list and filled must agree if the list of hints is filled.
*
*   @param list List of hints that are either filled with {0, 1} or with numbers 0,...,9.
*   @param sudoku The sudoku structure of a given size
*   @param filled boolean if the hints is filled with values or just representation if a cell is filled or not.
*   @return Option<Vec<i32>> the solution of the sudoku if it is unique.
*/
pub fn calculate_solution(list: &Vec<usize>, sudoku: &mut Sudoku, filled:bool) -> Option<Vec<i32>> {
    let mut solver = Solver::new();
    let clauses = sudoku_clauses(sudoku.board_size);
    for clause in clauses {
        solver.add_clause(clause);
    }
    if !filled {
        let (results, row_permutation, col_permutation, mirror) = fill_grid(&list, &(sudoku.board_size as usize));
        for result in results {
            let instant = Instant::now();
            let (unique, possible_sol) = Sudoku::unique(sudoku, &result, &mut solver);
            println!("unique solution time: {:?}", instant.elapsed());
            if unique {
                return Some(Sudoku::to_list(&mut possible_sol.unwrap(), sudoku.board_size));
            }
        }
    }else { 
        let (unique, possible_sol) = sudoku::Sudoku::unique(sudoku, &list, &mut solver);
        if unique {
            return Some(Sudoku::to_list(&mut possible_sol.unwrap(), sudoku.board_size))
        }
    }
    return None;
}