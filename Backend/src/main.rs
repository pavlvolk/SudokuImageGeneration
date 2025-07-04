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
mod difficulty;
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
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self, Write};
use crate::apply_permutations::apply_reverse_permutations;
use crate::difficulty::serate;

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

#[actix_web::main]
async fn main() {
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
        "Option 5: Start Server",   
        "Option 6: Difficulty",
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
        4 => option_5().await.unwrap(),
        5 => option_6(),
        _ => println!("Ungültige Auswahl."),
    }
}
fn option_1() {
    let h = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0,
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
    let transformed: Vec<usize> = h
        .into_iter()
        .map(|x| if x == 0 { 0 } else { 1 })
        .collect();
    let count_ones = transformed.iter().filter(|&&x| x != 0).count();
    println!("Anzahl der 1s: {}", count_ones);
    //let t1: Vec<_> = h.into_iter().map(|x| if x == 0 { 0 } else { 1 }).collect();
    //println!("{:?}", calculate_solution(&hints, &mut s, true).unwrap());
    println!("{:?}", calculate_solution(&transformed, &mut s, false).unwrap());
    //println!("{:?}", calculate_solution(&t1, &mut s, false).unwrap());
}

fn option_2() {
    println!("Zeiten testen");
    if let Err(e) = csv_tests("data/sudoku_test_set_9x9.txt", false) {
        eprintln!("Fehler beim Verarbeiten der Datei: {}", e);
    }
}

fn option_3() {

    println!("Threads");
    let h = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0,
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
    let transformed: Vec<usize> = h
        .into_iter()
        .map(|x| if x == 0 { 0 } else { 1 })
        .collect();
    let count_ones = transformed.iter().filter(|&&x| x != 0).count();
    println!("Anzahl der 1s: {}", count_ones);
    let mut s = Sudoku::new(9);
    println!("{:?}", calculation::thread_calculation(&transformed, "data/permuted_solutions.txt", &mut s));

}

fn option_4() {
    println!("👋 Programm wird beendet. Auf Wiedersehen!");
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
    if result.as_ref().unwrap().is_none() {
        return HttpResponse::Ok().json(output);
    }
    else{
        let mut outputdata = result.unwrap().unwrap();
        for i in 0..input.length {
            if input.data[i] == 0 {
                outputdata[i] = 0;
            }
        }
        output.data = outputdata;
        output.hassolution = true;
        return HttpResponse::Ok().json(output);
    }
}

async fn option_5() -> std::io::Result<()> {
    println!("81D Tuple processor running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .service(process_tuple)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}

fn option_6(){
    println!("Difficulty");
    let vec_4x4: Vec<Vec<i32>> = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];

    let array_6x6: [[i32; 6]; 6] = [
        [1, 2, 3, 4, 5, 6],
        [7, 8, 9, 10, 11, 12],
        [13, 14, 15, 16, 17, 18],
        [19, 20, 21, 22, 23, 24],
        [25, 26, 27, 28, 29, 30],
        [31, 32, 33, 34, 35, 36],
    ];
    let array_9x9: [[i32; 9]; 9] = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [10, 11, 12, 13, 14, 15, 16, 17, 18],
        [19, 20, 21, 22, 23, 24, 25, 26, 27],
        [28, 29, 30, 31, 32, 33, 34, 35, 36],
        [37, 38, 39, 40, 41, 42, 43, 44, 45],
        [46, 47, 48, 49, 50, 51, 52, 53, 54],
        [55, 56, 57, 58, 59, 60, 61, 62, 63],
        [64, 65, 66, 67, 68, 69, 70, 71, 72],
        [73, 74, 75, 76, 77, 78, 79, 80, 81],
    ];
    let mut board = vec![
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![3, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![5, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![6, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![7, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let mut board1 = vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 0], // Only '9' is missing
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
    ];
    serate(&mut board);
    serate(&mut board1);
    println!("{:?}", difficulty::get_all_units(9));

    let mut board2 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
        vec![0; 9],
    ];

    // Manually place values so that only cells (0,1), (1,1), (2,1) could have 5
    board2[0][0] = 1;
    board2[0][2] = 2;
    board2[1][0] = 3;
    board2[1][2] = 4;
    board2[2][0] = 6;
    board2[2][2] = 7;
    
    println!("{:?}", serate(&mut board2));
}