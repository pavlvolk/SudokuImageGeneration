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
        let mut outputdata = result.unwrap();
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