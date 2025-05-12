mod sort;
mod apply_permutations;
mod possibilities_first_column;
mod set_values;
mod fill_grid;
mod sudoku;
mod sudoku_clauses;
mod set_values_from_solution;
mod testen_Wahrscheinlichkeit;
mod calculation;
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
use crate::calculation::calculate_solution;
use crate::sudoku::Sudoku;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self, Write};


fn main() {
    // Welcome Screen
    println!("===============================");
    println!(" Willkommen zum Rust CLI Tool ");
    println!("===============================");
    println!("Bitte wählen Sie eine Option aus:");
    println!();

    // Optionen anzeigen
    let options = vec![
        "Option 1: Beispielsudokus berechnen",
        "Option 2: Zeiten testen",
        "Option 3: Neue Methode",
        "Option 4: Programm beenden",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Was möchten Sie tun?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => option_1(),
        1 => option_2(),
        2 => option_3(),
        3 => option_4(),
        _ => println!("Ungültige Auswahl."),
    }

    
    
    
     
/*
    let start = Instant::now();
    
    let mut s = sudoku::Sudoku::new(9);
    let hints:Vec<usize> = vec![0, 7, 0, 0, 0, 0, 0, 4, 3, 0, 4, 0, 0, 0, 9, 6, 1, 0, 8, 0, 0, 6, 3, 4, 9, 0, 0, 0, 9, 4, 0, 5, 2, 0, 0, 0, 3, 5, 8, 4, 6, 0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 5, 3, 0, 0, 8, 0, 0, 7, 0, 0, 9, 1, 9, 0, 2, 1, 0, 0, 0, 0, 5, 0, 0, 7, 0,4,0,8,0,2];
    let h1 = vec![0; 81];
    println!("{}", sudoku::Sudoku::unique(&mut s, &hints));
    println!("{}", sudoku::Sudoku::unique(&mut s, &h1));
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


    




}
// Beispiel-Funktionen für Optionen
fn option_1() {
    let h = vec![
        0, 0, 0, 0, 0, 0, 0, 1, 0,
        4, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 0,

        0, 0, 0, 0, 5, 0, 4, 0, 7,
        0, 0, 8, 0, 0, 0, 3, 0, 0,
        0, 0, 1, 0, 9, 0, 0, 0, 0,

        3, 0, 0, 4, 0, 0, 2, 0, 0,
        0, 5, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 6, 0, 0, 0,
    ];

    let hints:Vec<usize> = vec![0, 7, 0, 0, 0, 0, 0, 4, 3, 0, 4, 0, 0, 0, 9, 6, 1, 0, 8, 0, 0, 6, 3, 4, 9, 0, 0, 0, 9, 4, 0, 5, 2, 0, 0, 0, 3, 5, 8, 4, 6, 0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 5, 3, 0, 0, 8, 0, 0, 7, 0, 0, 9, 1, 9, 0, 2, 1, 0, 0, 0, 0, 5, 0, 0, 7, 0, 4, 0, 8, 0, 2];
    let mut s = Sudoku::new(9);
    let hints1:Vec<usize> = vec![0, 7, 0, 0, 0, 0, 0, 4, 3, 0, 4, 0, 0, 0, 9, 6, 1, 0, 8, 0, 0, 6, 3, 4, 9, 0, 0, 0, 9, 4, 0, 5, 2, 0, 0, 0, 3, 5, 8, 4, 6, 0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 5, 3, 0, 0, 8, 0, 0, 7, 0, 0, 9, 1, 9, 0, 2, 1, 0, 0, 0, 0, 5, 0, 0, 7, 0, 4, 0, 8, 0, 2];
    let transformed: Vec<usize> = hints1
        .into_iter()
        .map(|x| if x == 0 { 0 } else { 1 })
        .collect();
    let t1: Vec<_> = h.into_iter().map(|x| if x == 0 { 0 } else { 1 }).collect();
    //println!("{:?}", calculate_solution(&hints, &mut s, true).unwrap());
    //println!("{:?}", calculate_solution(&transformed, &mut s, false).unwrap());
    println!("{:?}", calculate_solution(&t1, &mut s, false).unwrap());
}

fn option_2() {
    println!("Zeiten testen");
    if let Err(e) = csv_tests("C:/Users/Hanne/PycharmProjects/SATvsCSP/sudoku_test_set_9x9.txt") {
        eprintln!("Fehler beim Verarbeiten der Datei: {}", e);
    }
}

fn option_3() {
    println!("Neue Methode");
}

fn option_4() {
    println!("👋 Programm wird beendet. Auf Wiedersehen!");
}

