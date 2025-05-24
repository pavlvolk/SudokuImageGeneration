use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Instant;
use cadical::Solver;
use crate::fill_grid::fill_grid;
use crate::sudoku;
use crate::sudoku::Sudoku;
use crate::sudoku_clauses::sudoku_clauses;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/**
*   This function takes hints and a sudoku of a specific size and outputs a solution if there is a definite one.
*   list and filled must agree if the list of hints is filled.
*
*   @param list List of hints that are either filled with {0, 1} or with numbers 0,...,9.
*   @param sudoku The sudoku structure of a given size
*   @param filled boolean if the hints is filled with values or just representation if a cell is filled or not.
*   @return Option<Vec<i32>> the solution of the sudoku if it is unique.
*/
pub fn calculate_solution(list: &Vec<usize>, mut sudoku: &mut Sudoku, filled: bool) -> Result<Option<Vec<i32>>, Box<dyn Error>> {
    let mut solver = Solver::new();
    let clauses = sudoku_clauses(sudoku.board_size);
    for clause in clauses {
        solver.add_clause(clause);
    }
    if !filled {
        if sudoku.board_size == 4 || sudoku.board_size == 6 {
            let mut i = 1;
            let (results, row_permutation, col_permutation, mirror) = fill_grid(&list, &(sudoku.board_size as usize));
            for result in results {
                println!("{}", i);
                let (unique, possible_sol) = Sudoku::unique(sudoku, &result, &mut solver);
                if unique {
                    return Ok(Some(Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku.board_size)));
                }
                i += 1;
            }
        }else if sudoku.board_size == 9 {
            let file = File::open("data/permuted_solutions.txt")?;
            let reader = BufReader::new(file);


            let mut count = 0;
            let max_count = 100000;   //TODO in main übergeben
            for line_result in reader.lines() {
                if count > max_count{
                    break
                }
                let line = line_result?;
                let cleaned_line = line.trim().trim_start_matches('[').trim_end_matches(']');
                let list: Vec<usize> = cleaned_line
                    .split(',')
                    .map(|s| s.trim().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?;

                let mut grid_list = Vec::new();
                for i in  0..81{
                    grid_list.push(list[i]);
                }


                let (unique, possible_sol) = Sudoku::unique(&mut sudoku, &grid_list, &mut solver);
                if unique {
                    return Ok(Some(Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku.board_size)));
                }
                count += 1;

            }
        }
    } else {
        let (unique, possible_sol) = sudoku::Sudoku::unique(sudoku, &list, &mut solver);
        if unique {
            return Ok(Some(Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku.board_size)));
        }
    }
    return Ok(None);
}

pub fn thread_calculation(path: &str, sudoku: &Sudoku) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut results = Vec::new();
    for _ in 0..8{
        results.push(Vec::new());
    }
    let mut i = 0;
    for line in reader.lines() {
        results[i%8].push(line.unwrap());
        i += 1;
    }
    for j in 0..8{
        let stop_flag = Arc::clone(&stop_flag);
        let lines = results[j].clone();
        let mut sudoku_clone = sudoku.clone();
        let mut solver = Solver::new();
        let clauses = sudoku_clauses(sudoku_clone.board_size);
        for clause in clauses {
            solver.add_clause(clause);
        }
        let handle = thread::spawn(move || {
            for line in lines{
                if stop_flag.load(Ordering::Relaxed) {
                    return
                }
                let cleaned_line = line.trim().trim_start_matches('[').trim_end_matches(']');
                let list: Vec<usize> = cleaned_line
                    .split(',')
                    .map(|s| s.trim().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>().unwrap();

                let mut grid_list = Vec::new();
                for i in  0..81{
                    grid_list.push(list[i]);
                }

                let (unique, possible_sol) = Sudoku::unique(&mut sudoku_clone, &grid_list, &mut solver);
                if unique{
                    println!("{:?}", Some(Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku_clone.board_size)));
                    stop_flag.store(true, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

