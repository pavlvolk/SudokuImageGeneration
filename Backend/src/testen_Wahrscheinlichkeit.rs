use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Instant;
use cadical::Solver;
use crate::apply_permutations::{apply_permutations, apply_reverse_permutations};
use crate::calculation::permute_numbers;
use crate::fill_grid::fill_grid;
use crate::sort::find_permutations;
use crate::sudoku;
use crate::sudoku::Sudoku;
use crate::sudoku_clauses::sudoku_clauses;

fn process_list(list: Vec<usize>) -> Result<Option<Vec<i32>>, Box<dyn Error> > {
    let mut solver = Solver::new();
    let mut sudoku = sudoku::Sudoku::new(9);
    let clauses = sudoku_clauses(9);
    for clause in clauses {
        solver.add_clause(clause);
    }
    let transformed: Vec<usize> = list
        .into_iter()
        .map(|x| if x == 0 { 0 } else { 1 })
        .collect();

    let count_ones = transformed.iter().filter(|&&x| x == 1).count();
    println!("Anzahl der 1s: {}", count_ones);

    let file = File::open("data/permuted_solutions.txt")?;
    let reader = BufReader::new(file);


    let mut count = 0;
    let max_count = 10000;   //TODO in main übergeben
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
            grid_list.push(list[i]*transformed[i]);
        }


        let (unique, possible_sol) = Sudoku::unique(&sudoku, &grid_list, &mut solver);
        if unique {

            let mut solution = Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku.board_size);
            solution = permute_numbers(&solution, 9);
            return Ok(Some(solution));
        }
        count += 1;

    }
    return Ok(None);
}

fn process_list_threads(list: Vec<usize>) -> Option<Vec<i32>> {
    let transformed: Vec<usize> = list
        .into_iter()
        .map(|x| if x == 0 { 0 } else { 1 })
        .collect();
    let count_ones = transformed.iter().filter(|&&x| x == 1).count();
    println!("Anzahl der 1s: {}", count_ones);
    let mut sudoku = sudoku::Sudoku::new(9);
    let stop_flag = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    let file = File::open("data/permuted_solutions.txt").unwrap();
    let reader = BufReader::new(file);
    let mut results = Vec::new();
    for _ in 0..16{
        results.push(Vec::new());
    }
    let mut i = 0;
    for line in reader.lines() {
        if i > 160000{
            break;
        }
        results[i%8].push(line.unwrap());
        i += 1;
    }
    for j in 0..16{
        let transformed_clone: Vec<usize> = transformed.clone();
        let tx = tx.clone();
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
                    grid_list.push(list[i]*transformed_clone[i]);
                }
                let (unique, possible_sol) = Sudoku::unique(&sudoku_clone, &grid_list, &mut solver);
                if unique{
                    let mut solution = Sudoku::to_list(&mut possible_sol.unwrap(), &sudoku.board_size);
                    solution = permute_numbers(&solution, 9);
                    let _ = tx.send(solution);
                    stop_flag.store(true, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx);
    rx.recv().ok()
}

pub fn csv_tests(file_path: &str, threads: bool) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let cleaned_line = line.trim().trim_start_matches('[').trim_end_matches(']');
        let list: Vec<usize> = cleaned_line
            .split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let start = Instant::now();
        if threads {
            println!("{:?}", process_list_threads(list));
        }else{
            println!("{:?}", process_list(list));
        }
        let end = start.elapsed();
        println!("Dauer: {:?}", end);
    }

    Ok(())
}



/*
fn main() {
    if let Err(e) = read_and_process_csv("pfad/zur/deiner/datei.csv") {
        eprintln!("Fehler beim Verarbeiten der Datei: {}", e);
    }
}

 */