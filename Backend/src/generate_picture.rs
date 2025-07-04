use image::{GenericImageView, GrayImage};
use crate::calculation;
use crate::sudoku_clauses::sudoku_clauses;
use crate::calculation::calculate_solution;
use crate::constants::SOLUTION;
use crate::sudoku::Sudoku;

pub fn generate_picture() -> Result<(), Box<dyn std::error::Error>> {
    // Laden des Originalbilds
    let img = image::open("data/herz.png")?;

    // In Grauwerte umwandeln
    let grau = img.to_luma8();

    // Größenänderung auf 9x9
    let resized = image::imageops::resize(
        &grau,
        9,
        9,
        image::imageops::FilterType::Triangle,
    );

    resized.save("data/output_picture.png")?;

    // Alle grauwerte in einem Vektor speichern
    let mut grauwerte = Vec::with_capacity(81);
    for y in 0..9 {
        for x in 0..9 {
            let pixel = resized.get_pixel(x, y);
            grauwerte.push(pixel[0] as usize);
        }
    }

    println!("Die Grauwerte als Liste:\n{:?}", grauwerte);
    generate_grid(&mut grauwerte);

    Ok(())
}

fn missing_spots(grid: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut rows = vec![0; 9];
    let mut cols = vec![0; 9];

    for spot in 0..81{
        let col = spot % 9;
        let row = (spot-col)/9;
        rows[row] += grid[spot];
        cols[col] += grid[spot];
    }
    for i in 0..9{
        if rows[i] > 0{
            rows[i] = 1;
        }
        if cols[i] > 0{
            cols[i] = 1;
        }
    }

    let mut missing_rows = Vec::new();
    let mut missing_cols = Vec::new();

    for i in 0..3{
        let mut col_sum = 0;
        let mut row_sum = 0;
        for j in (3*i)..(3*i +3){
            col_sum += cols[j];
            row_sum += rows[j];
        }
        if col_sum < 2 {
            for j in (3*i)..(3*i +3){
                if cols[j] == 0{
                    missing_cols.push(j);
                }
            }
        }
        if row_sum < 2 {
            for j in (3*i)..(3*i +3){
                if rows[j] == 0{
                    missing_rows.push(j);
                }
            }
        }
    }
    for i in missing_rows.iter(){
        for j in missing_cols.iter(){
            result.push(9*i + j);
        }
    }
    result
}

fn generate_grid(gray_values: &Vec<usize>) -> Result<(), Box<dyn std::error::Error>>{
    let mut grid = vec![0; 81];
    let squares = vec![
        // Box 1 (links oben)
        vec![0, 1, 2, 9, 10, 11, 18, 19, 20],
        // Box 2 (mitte oben)
        vec![3, 4, 5, 12, 13, 14, 21, 22, 23],
        // Box 3 (rechts oben)
        vec![6, 7, 8, 15, 16, 17, 24, 25, 26],
        // Box 4 (links mitte)
        vec![27, 28, 29, 36, 37, 38, 45, 46, 47],
        // Box 5 (mitte)
        vec![30, 31, 32, 39, 40, 41, 48, 49, 50],
        // Box 6 (rechts mitte)
        vec![33, 34, 35, 42, 43, 44, 51, 52, 53],
        // Box 7 (links unten)
        vec![54, 55, 56, 63, 64, 65, 72, 73, 74],
        // Box 8 (mitte unten)
        vec![57, 58, 59, 66, 67, 68, 75, 76, 77],
        // Box 9 (rechts unten)
        vec![60, 61, 62, 69, 70, 71, 78, 79, 80],
    ];
    let mut count = 0;

    for square in squares{
        let minimum = find_minimum_index(&grid, gray_values, &square);
        grid[minimum] = 1;
        count += 1;

    }
    loop{
        let mut missing_spots = missing_spots(&grid);
        if missing_spots.len() == 0{
            break;
        }
        let minimum = find_minimum_index(&grid, gray_values, &missing_spots);
        grid[minimum] = 1;
        count += 1;


    }
    let vector: Vec<usize> = (0..81).collect();

    while count < 25{
        let minimum = find_minimum_index(&grid, gray_values, &vector);
        grid[minimum] = 1;
        count += 1;
    }
    let mut s = Sudoku::new(9);
    let mut result = calculate_solution(&grid, &mut s, false).unwrap();
    while result == None{
        let minimum = find_minimum_index(&grid, gray_values, &vector);
        grid[minimum] = 1;
        count += 1;
        result = calculate_solution(&grid, &mut s, false).unwrap();
    }

    println!("{}", count);
    println!("{:?}", result);

    let data = vec![0u8; 81]; // Beispiel: alles 0
    // Du kannst ihn beliebig füllen

    // Erstelle neues Grauwert-Bild in 9x9
    let mut img = GrayImage::new(9, 9);

    for (i, &value) in grid.iter().enumerate() {
        let x = (i % 9) as u32;
        let y = (i / 9) as u32;

        // 0 -> schwarz, 1 -> weiß
        let color = if value == 0 { 255u8 } else { 0u8 };
        img.put_pixel(x, y, image::Luma([color]));
    }

    // Speichere das Bild
    img.save("data/black_white.png")?;
    Ok(())
}

fn find_minimum_index(grid: &Vec<usize>, gray_values: &Vec<usize>, candidates: &Vec<usize>) -> usize{
    let mut min = 256;
    let mut min_index = candidates[0];

    for i in candidates.iter() {
        if gray_values[*i] < min && grid[*i] == 0{
            min = gray_values[*i];
            min_index = *i;
        }
    }
    min_index
}