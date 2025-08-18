use crate::constants::TEST;
use crate::constants::{NUMBER_OF_THREADS, SOLUTION, SOLUTION_4, SOLUTION_6, SOLUTION_PER_THREAD};
use crate::sudoku;
use crate::sudoku::Sudoku;
use crate::sudoku_clauses::sudoku_clauses;
use cadical::Solver;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc};
use std::thread;
use std::time::Instant;

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
    let size= sudoku.board_size;
    let mut solver = Solver::new();
    let clauses = sudoku_clauses(sudoku.board_size);
    for clause in clauses {
        solver.add_clause(clause);
    }
    if !filled {
        if !test_if_solvable(list, &mut sudoku){
            return Ok(None);       
            //TODO Error Nachricht anpassen, damit man nicht denken kann das es vielleicht doch eine Lösung gibt.
        }
        if size == 4 || size == 6 {
            let file = if size == 4 { File::open(SOLUTION_4).unwrap()} else { File::open(SOLUTION_6).unwrap()};
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let transformed: Vec<usize> = line?.chars()
                    .map(|c| c.to_digit(10).expect("Ungültige Ziffer") as usize)
                    .collect();
                let mut grid_list = Vec::new();
                for i in  0..(size*size) as usize{
                    grid_list.push(list[i]*transformed[i]);
                }
                let (unique, possible_sol) = Sudoku::unique(sudoku, &grid_list, &mut solver);
                if unique {
                    let mut check_solver = Solver::new();
                    let check_clauses = sudoku_clauses(size);
                    for clause in check_clauses{
                        check_solver.add_clause(clause);
                    }
                    let (check_unique, _) = Sudoku::unique(&sudoku, &grid_list, &mut check_solver);
                    if !check_unique{
                        continue;
                    }
                    
                    let mut solution = Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku.board_size);
                    solution = permute_numbers(&solution, sudoku.board_size);
                    return Ok(Some(solution));
                }
            }
            
            
            
            /*
            let (results, row_permutation, col_permutation, mirror) = fill_grid(&list, &(sudoku.board_size as usize));
            for result in results {
                let (unique, possible_sol) = Sudoku::unique(sudoku, &result, &mut solver);
                if unique {
                    let mut solution = Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku.board_size);
                    solution = permute_numbers(&solution, sudoku.board_size);
                    let mut usize_solution: Vec<usize> = solution.into_iter().map(|x| x as usize).collect();
                    usize_solution = apply_reverse_permutations(&usize_solution, &row_permutation, &col_permutation, &mirror, &(sudoku.board_size as usize));
                    let solution = usize_solution.into_iter().map(|x| x as i32).collect();
                    return Ok(Some(solution));
                }
            }
            
             */
        }else if sudoku.board_size == 9 {
            
            let mapped_list: Vec<usize> = list
                .iter()
                .map(|x| if *x == 0 { 0 } else { 1 })
                .collect();

            return Ok(thread_calculation(list, &mapped_list));
        }
    } else {
        let (unique, possible_sol) = sudoku::Sudoku::unique(sudoku, &list, &mut solver);
        if unique {
            return Ok(Some(Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku.board_size)));
        }
    }
    return Ok(None);
}


pub fn thread_calculation(constraint_list: &Vec<usize>, list: &Vec<usize>) -> Option<Vec<i32>> {
    let sudoku = sudoku::Sudoku::new(9);
    let (tx, rx) = mpsc::channel();
    let stop_flag = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];
    let file = File::open(SOLUTION).unwrap();
    let reader = BufReader::new(file);
    let mut results = Vec::new();
    for _ in 0..*NUMBER_OF_THREADS{
        results.push(Vec::new());
    }
    let mut i = 0;
    for line in reader.lines() {
        if i > *NUMBER_OF_THREADS * SOLUTION_PER_THREAD {
            break;
        }
        results[i%*NUMBER_OF_THREADS].push(line.unwrap());
        i += 1;
    }
    for j in 0..*NUMBER_OF_THREADS{
        let thread_list = list.clone();
        let tx = tx.clone();
        let stop_flag = Arc::clone(&stop_flag);
        let lines = results[j].clone();
        let mut sudoku_clone = sudoku.clone();
        let mut solver = Solver::new();
        let clauses = sudoku_clauses(sudoku_clone.board_size);
        for clause in clauses {
            solver.add_clause(clause);
        }
        let constraint_list_clone = constraint_list.clone();
        let handle = thread::spawn(move || {
            for line in lines{
                if stop_flag.load(Ordering::Relaxed) {
                    return
                }
                let transformed: Vec<usize> = line
                    .chars()
                    .map(|c| c.to_digit(10).expect("Ungültige Ziffer") as usize)
                    .collect();

                let mut grid_list = Vec::new();
                for i in  0..81{
                    grid_list.push(thread_list[i]*transformed[i]);
                }

                let (unique, possible_sol) = Sudoku::unique(&mut sudoku_clone, &grid_list, &mut solver);
                if unique{
                    let mut check_solver = Solver::new();
                    let check_clauses = sudoku_clauses(sudoku_clone.board_size);
                    for clause in check_clauses{
                        check_solver.add_clause(clause);
                    }
                    let (check_unique, _) = Sudoku::unique(&sudoku_clone, &grid_list, &mut check_solver);
                    if !check_unique{
                        continue;
                    }
                    let mut solution = Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku_clone.board_size);
                    let start = Instant::now();
                    let (bool, solution)= permute_numbers_with_constraint(&solution, &constraint_list_clone, 9);
                    println!("{:?}", start.elapsed());
                    //solution = permute_numbers(&solution, 9);
                    if !bool{
                        continue;
                    }
                    let _ = tx.send(solution);
                    stop_flag.store(true, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle)
    }
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread panic: {:?}", e);
        }
    }

    drop(tx);
    rx.recv().ok()
}


/*
Testet zunächst ob das Sudoku mit den ausgewählten Zahlen überhaupt lösbar ist.
Wenn nicht kann dann schnell abgelehnt werden.
 */
fn test_if_solvable(list: &Vec<usize>, mut sudoku: &mut Sudoku) -> bool {
    let new_list: Vec<usize> = list
        .iter()
        .map(|x| if *x == 0 { 0 } else { *x - 1 })
        .collect();
    
    let mut solver = Solver::new();
    let clauses = sudoku_clauses(sudoku.board_size);
    for clause in clauses {
        solver.add_clause(clause);
    }
    
    let (solvable, possible_sol) = sudoku::Sudoku::solvable(sudoku, &new_list, &mut solver);
    solvable
}



/*
Permutiert die Zahlen random
 */
pub(crate) fn permute_numbers(list: &Vec<i32>, size: i32) -> Vec<i32> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();

    // Erzeuge Zahlen von 0 bis size-1
    let mut indices: Vec<usize> = (1..(size+1) as usize).collect();

    // Mische die Indizes zufällig
    indices.shuffle(&mut rng);

    let size_square = size*size;
    let mut result = vec![0; size_square as usize];
    for i in 0..size_square as usize {
        result[i] = indices[list[i] as usize - 1] as i32;
    }
    result
}

pub fn permute_numbers_with_constraint(result: &Vec<i32>, constraints: &Vec<usize>, size: i32) -> (bool, Vec<i32>) {
    let mut numbers: HashSet<usize> = HashSet::new();
    let mut indices= vec![0; size as usize];
    for i in 1..(size+1) as usize {
        numbers.insert(i);   
    }
    
    let size_square = size*size;
    for i in 0..size_square as usize {
        if constraints[i] > 1{
            let current_value = result[i] as usize;
            let new_value = constraints[i] - 1;
            if (indices[current_value - 1] == 0){
                indices[current_value - 1] = new_value;
                if numbers.contains(&(new_value)){
                    numbers.remove(&new_value);
                } else { 
                    return (false, Vec::new());
                }
            } else if indices[current_value - 1] != new_value {
                return (false, Vec::new());
            }
        }
    }
    let mut rng = thread_rng();
    for i in 0..indices.len(){
        if indices[i] == 0{
            if let Some(&element) = numbers.iter().choose(&mut rng) {
                numbers.remove(&element);
                indices[i] = element;
            }
        }
    }
    
    println!("{:?}", indices);

    let mut new_result = vec![0; size_square as usize];
    for i in 0..size_square as usize {
        new_result[i] = indices[result[i] as usize - 1] as i32;
    }
    
    (true, new_result)
}
