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
use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

//Input structure
#[derive(Deserialize)]
struct Input {
    data: Vec<usize>,
    length: usize,
    markingmode: bool,  
}

// Output structure
#[derive(Serialize)]
struct Output {
    data: Vec<i32>,
    hassolution: bool ,
}


#[post("/api/process-tuple")]
async fn process_tuple(input: web::Json<Input>) -> HttpResponse {
    println!("Attempted Connection");
    let result;
    let mut output = Output {
        data: vec![0; 81],
        hassolution: false,
    };
    if input.length == 81 {
        result = calculate_solution(&input.data, &mut Sudoku::new(9), !input.markingmode);
        println!("{:?}", result);
        println!("{:?}", input.data);
    }
    else if input.length == 36 {
        result = calculate_solution(&input.data, &mut Sudoku::new(6), !input.markingmode);
    }
    else if input.length == 16 {
        result = calculate_solution(&input.data, &mut Sudoku::new(4), !input.markingmode);
    }
    else{
        return HttpResponse::BadRequest().json("Wrong Dimension");
    }
    if result.is_none() {
        return HttpResponse::Ok().json(output);
    }
    else{
        output.data = result.unwrap();
        output.hassolution = true;
        return HttpResponse::Ok().json(output);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("81D Tuple processor running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .service(process_tuple)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
/*
fn main() {
    
    let hints:Vec<usize> = vec![0, 7, 0, 0, 0, 0, 0, 4, 3, 0, 4, 0, 0, 0, 9, 6, 1, 0, 8, 0, 0, 6, 3, 4, 9, 0, 0, 0, 9, 4, 0, 5, 2, 0, 0, 0, 3, 5, 8, 4, 6, 0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 5, 3, 0, 0, 8, 0, 0, 7, 0, 0, 9, 1, 9, 0, 2, 1, 0, 0, 0, 0, 5, 0, 0, 7, 0, 4, 0, 8, 0, 2];
    let mut s = Sudoku::new(9);
    let hints1:Vec<usize> = vec![0, 7, 0, 0, 0, 0, 0, 4, 3, 0, 4, 0, 0, 0, 9, 6, 1, 0, 8, 0, 0, 6, 3, 4, 9, 0, 0, 0, 9, 4, 0, 5, 2, 0, 0, 0, 3, 5, 8, 4, 6, 0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 5, 3, 0, 0, 8, 0, 0, 7, 0, 0, 9, 1, 9, 0, 2, 1, 0, 0, 0, 0, 5, 0, 0, 7, 0, 4, 0, 8, 0, 2];
    let transformed: Vec<usize> = hints1
        .into_iter()
        .map(|x| if x == 0 { 0 } else { 1 })
        .collect();
    println!("{:?}", calculate_solution(&hints, &mut s, true).unwrap());
    println!("{:?}", calculate_solution(&transformed, &mut s, false).unwrap());
    
     
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


    if let Err(e) = csv_tests("C:/Users/Hanne/PycharmProjects/SATvsCSP/sudoku_test_set_9x9.txt") {
        eprintln!("Fehler beim Verarbeiten der Datei: {}", e);
    }




}

*/