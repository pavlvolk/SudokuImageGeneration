use std::collections::HashSet;
use rand::Rng;
use crate::possibilities_first_column::{possibilities_first_column_four, possibilities_first_column_nine, possibilities_not_complete_first_column};
use crate::set_values_from_solution::set_values_from_solution;
/*
Zuerst werden je nach Größe festgelegt welche Werte noch befüllt werden müssen und was für Constraints in den Blöcken gelten.
Die Constraints der Zeile und Spalte werden später im Backtracking seperat geprüft
Die erste Zeile wird mit den Werten 1 bis 9 belegt
Für die erste Spalte werden alle möglichen Belegungen berechnet und jeweils Backtracking aufgerufen
Für 4x4 und 6x6 wurd das normale Backtracking aufgerufen und es werden alle möglichen Belegungen getestet
Für 9x9 wird das Grid auf vollständige Sudokus gelegt

 */
pub fn set_values(grid: &[usize], grid_size: &usize) -> HashSet<Vec<usize>> {
    let mut value_grid = Vec::new();
    let GRID_SIZE = *grid_size;
    let list;
    let constraint_list;
    if GRID_SIZE == 4 || GRID_SIZE == 6{
        if GRID_SIZE == 4{
            list = vec![5, 6, 7, 9, 10, 11, 13, 14, 15];
            constraint_list = vec![
                vec![0], vec![3], vec![2], vec![12], vec![15], vec![14], vec![8], vec![11], vec![10]
            ];
        } else{
            list = vec![7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 25, 26, 27, 28, 29, 31, 32, 33, 34, 35];
            constraint_list = vec![
                vec![0,2], vec![0,1], vec![4,5], vec![3,5], vec![3,4],
                vec![18,20], vec![18,19], vec![22,23], vec![21,23], vec![21,22],
                vec![12,14], vec![12,13], vec![16,17], vec![15,17], vec![15,16],
                vec![30,32], vec![30,31], vec![34,35], vec![33,35], vec![33,34],
                vec![24,26], vec![24,25], vec![28,29], vec![27,29], vec![27,28],
            ];
        }
        for i in 0..GRID_SIZE {
            value_grid.push(grid[i] * (i + 1))
        }
        for _ in 0..(GRID_SIZE * (GRID_SIZE - 1)){
            value_grid.push(0);
        }
        let mut fields_of_first_columns = Vec::new();
        for i in 0..GRID_SIZE - 1 {
            if grid[(i+1)*GRID_SIZE] == 1 {
                fields_of_first_columns.push(i);
            }
        }
        let possible_first_columns = possibilities_not_complete_first_column(&fields_of_first_columns, &GRID_SIZE);
        let mut results = HashSet::new();

        for first_column in possible_first_columns.iter() {
            for i in 1..GRID_SIZE{
                value_grid[i * GRID_SIZE] = first_column[i-1];
            }
            find_unique_combinations(&list, &constraint_list, &value_grid, grid, &GRID_SIZE, &mut results);
        }
        return results;
    }
    set_values_from_solution(grid)
}

/*
ruft das normale Backtracking auf
 */
fn find_unique_combinations(lists: &[usize], constraints_list: &[Vec<usize>], value_grid: &Vec<usize>, grid: &[usize], grid_size: &usize, results: &mut HashSet<Vec<usize>>){
    let mut current_combination = value_grid.clone();
    backtrack(0, lists, constraints_list, &mut current_combination, grid, results, grid_size);
}

/*
ruft das random Backtracking auf
 */
fn find_random_combinations(lists: &[usize], constraints_list: &[Vec<usize>], value_grid: &Vec<usize>, grid: &[usize], grid_size: &usize, results: &mut HashSet<Vec<usize>>){
    let mut current_combination = value_grid.clone();
    random_backtrack(0, lists, constraints_list, &mut current_combination, grid, results, grid_size);
}

/*
Backtracking Funktion welche alle Constraints prüft
 */
fn backtrack(
    index: usize,
    lists: &[usize],
    constraints_list: &[Vec<usize>],
    current_combination: &mut Vec<usize>,
    grid: &[usize],
    results: &mut HashSet<Vec<usize>>,
    grid_size: &usize
) {
    let GRID_SIZE = *grid_size;

    if index == lists.len() {
        results.insert(current_combination.clone());
        return;
    }

    let position = lists[index];
    if grid[position] == 0 {
        backtrack(index + 1, lists, constraints_list, current_combination, grid, results, grid_size);
    }
    else {
        for number in 1..GRID_SIZE + 1 {
            let mut collision = false;

            for &constraint in constraints_list[index].iter() {
                if(constraint < GRID_SIZE){
                    if constraint + 1 == number {
                        collision = true;
                    }
                } else {
                    if current_combination[constraint] == number {
                        collision = true;
                    }
                }
            }
            let row = position / GRID_SIZE;
            let col = position % GRID_SIZE;
            for i in 0..GRID_SIZE {
                let constraint = row * GRID_SIZE + i;
                if(constraint < GRID_SIZE){
                    if constraint + 1 == number {
                        collision = true;
                    }
                } else {
                    if current_combination[constraint] == number {
                        collision = true;
                    }
                }
                let constraint = i * GRID_SIZE + col;
                if(constraint < GRID_SIZE){
                    if constraint + 1 == number {
                        collision = true;
                    }
                } else {
                    if current_combination[constraint] == number {
                        collision = true;
                    }
                }
            }

            if !collision {
                current_combination[position] = number;

                backtrack(index + 1, lists, constraints_list, current_combination, grid, results, grid_size);

                current_combination[position] = 0;
            }
        }
    }

}

/*
Backtracking Funktion die random auswählt welcher Teilbaum zuerst betrachtet wird
Sobald eine Lösung gefunden wurde wird abgebrochen
 */
fn random_backtrack(
    index: usize,
    lists: &[usize],
    constraints_list: &[Vec<usize>],
    current_combination: &mut Vec<usize>,
    grid: &[usize],
    results: &mut HashSet<Vec<usize>>,
    grid_size: &usize
) {
    let GRID_SIZE = *grid_size;

    if index == lists.len() {
        results.insert(current_combination.clone());
        return;
    }

    let position = lists[index];
    if grid[position] == 0 {
        random_backtrack(index + 1, lists, constraints_list, current_combination, grid, results, grid_size);
    }
    else {
        let mut rng = rand::thread_rng();
        let random_number: usize = rng.random_range(1..GRID_SIZE + 1);
        let amount_of_results = results.len();
        let mut increment = 0;
        while(results.len() < amount_of_results + 1) && increment < GRID_SIZE{
            let number = random_number + increment;

            let mut collision = false;

            for &constraint in constraints_list[index].iter() {
                if(constraint < GRID_SIZE){
                    if constraint + 1 == number {
                        collision = true;
                    }
                } else {
                    if current_combination[constraint] == number {
                        collision = true;
                    }
                }
            }
            let row = position / GRID_SIZE;
            let col = position % GRID_SIZE;
            for i in 0..GRID_SIZE {
                let constraint = row * GRID_SIZE + i;
                if(constraint < GRID_SIZE){
                    if constraint + 1 == number {
                        collision = true;
                    }
                } else {
                    if current_combination[constraint] == number {
                        collision = true;
                    }
                }
                let constraint = i * GRID_SIZE + col;
                if(constraint < GRID_SIZE){
                    if constraint + 1 == number {
                        collision = true;
                    }
                } else {
                    if current_combination[constraint] == number {
                        collision = true;
                    }
                }
            }

            if !collision {
                current_combination[position] = number;
                random_backtrack(index + 1, lists, constraints_list, current_combination, grid, results, grid_size);
            }
            increment += 1;
        }

    }
}

/*
else {
        list = vec![10, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26,
                    28, 29, 30, 31, 32, 33, 34, 35, 37, 38, 39, 40, 41, 42, 43, 44,
                    46, 47, 48, 49, 50, 51, 52, 53, 55, 56, 57, 58, 59, 60, 61, 62,
                    64, 65, 66, 67, 68, 69, 70, 71, 73, 74, 75, 76, 77, 78, 79, 80];
        constraint_list = vec![
            vec![0,2, 18,20], vec![0,1,18,19], vec![4,5,22,23], vec![3,5,21,23], vec![3,4,21,22], vec![7,8,25,26], vec![6,8,24,26], vec![6,7,24,25],
            vec![0,2, 9,11], vec![0,1,9, 10], vec![4,5,13,14], vec![3,5,12,14], vec![3,4,12,13], vec![7,8,16,17], vec![6,8,15,17], vec![6,7,15,16],
            vec![36,38, 45, 47], vec![36,37,45,46], vec![40,41,49,50], vec![39,41,48,50], vec![39,40,48,49], vec![43,44,52,53], vec![42,44,51,53], vec![42,43,51,52],
            vec![27, 29, 45, 47], vec![27, 28, 45, 46], vec![31, 32, 49, 50], vec![30, 32, 48, 50], vec![30, 31, 48, 49], vec![34, 35, 52, 53], vec![33, 35, 51, 53], vec![33, 34, 51, 52],
            vec![27, 29, 36, 38], vec![27, 28, 36, 37], vec![31, 32, 40, 41], vec![30, 32, 39, 41], vec![30, 31, 39, 40], vec![34, 35, 43, 44], vec![33, 35, 42, 44], vec![33, 34, 42, 43],
            vec![63, 65, 72, 74], vec![63, 64, 72, 73], vec![67, 68, 76, 77], vec![66, 68, 75, 77], vec![66, 67, 75, 76], vec![70, 71, 79, 80], vec![69, 71, 78, 80], vec![69, 70, 78, 79],
            vec![54, 56, 72, 74], vec![54, 55, 72, 73], vec![58, 59, 76, 77], vec![57, 59, 75, 77], vec![57, 58, 75, 76], vec![61, 62, 79, 80], vec![60, 62, 78, 80], vec![60, 61, 78, 79],
            vec![54, 56, 63, 65], vec![54, 55, 63, 64], vec![58, 59, 67, 68], vec![57, 59, 66, 68], vec![57, 58, 66, 67], vec![61, 62, 70, 71], vec![60, 62, 69, 71], vec![60, 61, 69, 70],
        ];
    }

    if GRID_SIZE == 9{
        let amount_of_first_columns = possible_first_columns.len();
        let amount_of_field_per_column = 300 / amount_of_first_columns; //Todo Wert wieder hochsetzen
        let mut column_number = 1;
        for first_column in possible_first_columns.iter() {
            for i in 1..GRID_SIZE{
                value_grid[i * GRID_SIZE] = first_column[i-1];
            }
            while(results.len() < amount_of_field_per_column * column_number) {
                find_random_combinations(&list, &constraint_list, &value_grid, grid, &GRID_SIZE, &mut results);
            }
            column_number += 1;
        }
    }
 */