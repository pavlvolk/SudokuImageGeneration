use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::time::Instant;
use cadical::Solver;
use crate::fill_grid::fill_grid;
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
            grid_list.push(list[i]*transformed[i]);
        }


        let (unique, possible_sol) = Sudoku::unique(&mut sudoku, &grid_list, &mut solver);
        if unique {
            return Ok(Some(Sudoku::to_list(&mut possible_sol.unwrap(), sudoku.board_size)));
        }
        count += 1;

    }
    return Ok(None);
}

pub fn csv_tests(file_path: &str) -> Result<(), Box<dyn Error>> {
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
        println!("{:?}", process_list(list));
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