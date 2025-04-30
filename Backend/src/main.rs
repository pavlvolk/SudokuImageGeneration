mod sort;
mod apply_permutations;
mod possibilities_first_column;
mod set_values;
mod fill_grid;
mod sudoku;
mod sudoku_clauses;
mod set_values_from_solution;
mod testen_Wahrscheinlichkeit;
// Added to ensure the module is included

use std::time::Instant;
use sort::find_permutations;
use apply_permutations::apply_permutations;
use crate::fill_grid::fill_grid;
use crate::possibilities_first_column::{possibilities_first_column_nine, possibilities_first_column_six, possibilities_not_complete_first_column};
use crate::set_values::set_values;
use crate::set_values_from_solution::set_values_from_solution;
use crate::sudoku_clauses::sudoku_clauses;
use crate::testen_Wahrscheinlichkeit::csv_tests;

fn main() {
/*
    let start = Instant::now();
/*
    possibilities_first_column_six();
    possibilities_first_column_nine();
    possibilities_not_complete_first_column(&vec![0, 1], &6);

 */
    let mut grid = vec![0; 81];
    let mut fields = vec![0, 4, 9, 17, 18, 21, 25, 28, 29, 31, 32, 33, 40, 41, 42, 43, 45, 46, 50, 51, 54, 55, 60, 61, 67, 68, 73, 75, 76, 78 ];

    for field in fields{
        grid[field] = 1;
    }

    let (results, row_permutation, col_permutation, mirror) = fill_grid(&grid, &9);
    for result in results{
        println!("{:?}", result);
    }

    /*
    for row in 0..9{
        for col in 0..9{
            print!("{} ", grid[row*9+col]);
        }
        println!("");}
    println!("");

     */

    /*
    let (row_permutation, col_permutation, mirror) = find_permutations(&grid, &9);
    println!("{:?}", row_permutation);
    println!("{:?}", col_permutation);
    println!("{:?}", mirror);
    let mut grid_permuted = apply_permutations(&grid, &row_permutation, &col_permutation, &mirror, &9);
    for row in 0..9{
        for col in 0..9{
            print!("{} ", grid_permuted[row*9+col]);
        }
        println!("");}

    set_values(&grid_permuted, &9);

     */

    /*
    let mut grid = vec![0; 36];
    let mut fields = vec![2, 4, 11, 14, 16, 18, 19, 21, 26, 27, 28, 31, 34, 35];

    for field in fields{
        grid[field] = 1;
    }

    println!("");
    for row in 0..6{
        for col in 0..6{
            print!("{} ", grid[row*6+col]);
        }
        println!("");}
    println!("");


    let (row_permutation, col_permutation, mirror) = find_permutations(&grid, &6);
    println!("{:?}", row_permutation);
    println!("{:?}", col_permutation);
    println!("{:?}", mirror);
    let mut grid_permuted = apply_permutations(&grid, &row_permutation, &col_permutation, &mirror, &6);
    for row in 0..6{
        for col in 0..6{
            print!("{} ", grid_permuted[row*6+col]);
        }
        println!("");}

    set_values(&grid_permuted, &6);

    let mut grid = vec![0; 16];
    let mut fields = vec![2, 3, 6, 8, 10, 13];

    for field in fields{
        grid[field] = 1;
    }

    println!("");
    for row in 0..4{
        for col in 0..4{
            print!("{} ", grid[row*4+col]);
        }
        println!("");}
    println!("");

    let (row_permutation, col_permutation, mirror) = find_permutations(&grid, &4);
    println!("{:?}", row_permutation);
    println!("{:?}", col_permutation);
    println!("{:?}", mirror);
    let mut grid_permuted = apply_permutations(&grid, &row_permutation, &col_permutation, &mirror, &4);
    for row in 0..4{
        for col in 0..4{
            print!("{} ", grid_permuted[row*4+col]);
        }
        println!("");}

    set_values(&grid_permuted, &4);

     */

    let duration = start.elapsed();
    println!("Dauer: {:?}", duration);

 */


    if let Err(e) = csv_tests("C:/Users/Hanne/PycharmProjects/SATvsCSP/sudoku_test_set_9x9.txt") {
        eprintln!("Fehler beim Verarbeiten der Datei: {}", e);
    }




}

