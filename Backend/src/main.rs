mod sudoku;
mod sudoku_clauses;
mod calculation;
mod difficulty;
mod constants;
// Added to ensure the module is included

use crate::sudoku_clauses::sudoku_clauses;
use crate::calculation::calculate_solution;
use crate::sudoku::Sudoku;
use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::io::{Write};
use crate::difficulty::{rate_difficulty};
use itertools::Itertools;
use rayon::prelude::*;

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
    hassolution: bool,
    difficulty: f64,
}

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
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
#[post("/api/process-tuple")]
async fn process_tuple(input: web::Json<Input>) -> HttpResponse {
    println!("Attempted Connection");
    let result;
    let mut output = Output {
        data: vec![0; 81],
        solution: vec![0; 81],
        hassolution: false,
        difficulty: 0.0,
    };
    if input.length == 81 {
        result = calculate_solution(&input.data, &mut Sudoku::new(9), !input.markingmode);
        println!("{:?}", result);
        println!("{:?}", input.data);
    }
    else if input.length == 36 {
        println!("{:?}", input.data);
        result = calculate_solution(&input.data, &mut Sudoku::new(6), !input.markingmode);
    }
    else if input.length == 16 {
        println!("{:?}", input.data);
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
        output.difficulty = rate_difficulty(output.data.clone());
        return HttpResponse::Ok().json(output);
    }
}
