use crate::fill_grid::fill_grid;
use crate::sudoku;
use crate::sudoku::Sudoku;

pub fn calculate_solution(list: &Vec<usize>, sudoku: &mut Sudoku, filled:bool) -> Option<Vec<i32>> {
    if !filled {
        let (results, row_permutation, col_permutation, mirror) = fill_grid(&list, &9);
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