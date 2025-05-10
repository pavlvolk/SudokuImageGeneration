use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::time::Instant;
use cadical::Solver;
use crate::fill_grid::fill_grid;
use crate::sudoku;
use crate::sudoku_clauses::sudoku_clauses;

fn process_list(list: Vec<usize>) -> (bool, usize) {
    let mut solver = Solver::new();
    let clauses = sudoku_clauses(9);
    for clause in clauses {
        solver.add_clause(clause);
    }
    let transformed: Vec<usize> = list
        .into_iter()
        .map(|x| if x == 0 { 0 } else { 1 })
        .collect();

    let count_ones = transformed.iter().filter(|&&x| x == 1).count();

    let mut sudoku = sudoku::Sudoku::new(9);
    let mut i = 1;
    let (results, row_permutation, col_permutation, mirror) = fill_grid(&transformed, &9);
    for result in results {
        let start = Instant::now();
        let (unique, _) = sudoku::Sudoku::unique(&mut sudoku, &result, &mut solver);
        println!("unique result in {:?}", start.elapsed());
        if unique {
            println!("{:?}", i);
            return (unique, count_ones);
        }
        i += 1;
    }
    (false,count_ones)
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

        println!("{:?}", process_list(list));
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