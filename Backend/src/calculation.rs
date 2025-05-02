use crate::fill_grid::fill_grid;
use crate::sudoku;
use crate::sudoku::Sudoku;


/**
*   This function takes hints and a sudoku of a specific size and outputs a solution if there is a definite one.
*   list and filled must agree if the list of hints is filled.
*   @param list List of hints that are either filled with {0, 1} or with numbers 0,...,9.
*   @param sudoku The sudoku structure of a given size
*   @param filled boolean if the hints is filled with values or just representation if a cell is filled or not.
*   @return Option<Vec<i32>> the solution of the sudoku if it is unique.
*/
pub fn calculate_solution(list: &Vec<usize>, sudoku: &mut Sudoku, filled:bool) -> Option<Vec<i32>> {
    if !filled {
        let (results, row_permutation, col_permutation, mirror) = fill_grid(&list, &(sudoku.board_size as usize));
        for result in results {
            let unique = Sudoku::unique(sudoku, &result);
            if unique {
                let (_, solution) = sudoku::Sudoku::solvable(sudoku, &result);
                return Some(Sudoku::to_list(&mut solution.unwrap(), sudoku.board_size))
            }
        }
    }else { 
        let unique = sudoku::Sudoku::unique(sudoku, &list);
        if unique {
            let (_, solution) = sudoku::Sudoku::solvable(sudoku, &list);
            return Some(Sudoku::to_list(&mut solution.unwrap(), sudoku.board_size))
        }
    }
    return None;
}