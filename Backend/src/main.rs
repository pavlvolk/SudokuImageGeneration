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
mod testen_der_neuen_loesung;
mod generate_picture;

mod constants;
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
use crate::constants::SOLUTION;
use crate::constants::TEST;
use crate::testen_der_neuen_loesung::csv_tests_compare;
use crate::generate_picture::generate_picture;

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
    solution: Vec<i32>,
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
        "Option 4: Neue Tests",
        "Option 5: Start Server",
        "Option 6: Use Picture",
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
    if let Err(e) = csv_tests(TEST, true) {
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
    println!("{:?}", calculation::thread_calculation(&transformed, SOLUTION, &mut s));

}

fn option_4() {
    println!("verschiedene Tests");
    csv_tests_compare(TEST);
}

#[post("/api/process-tuple")]
async fn process_tuple(input: web::Json<Input>) -> HttpResponse {
    println!("Attempted Connection");
    let result;
    let mut output = Output {
        data: vec![0; 81],
        solution: vec![0; 81],       
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
        let solution = outputdata.clone();
        for i in 0..input.length {
            if input.data[i] == 0 {
                outputdata[i] = 0;
            }
        }
        output.data = outputdata;
        output.hassolution = true;
        output.solution = solution;
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
    generate_picture();
}