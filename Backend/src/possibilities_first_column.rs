/*
Berechnet verschiedene Belegungen für die erste Spalte unter Berücksichtigung der Symmetrie
 */
use std::collections::HashSet;

pub fn possibilities_first_column_nine() -> Vec<Vec<usize>> {

    let lists:Vec<Vec<usize>> = vec![
        vec![4, 5, 6, 7, 8],
        vec![5, 6, 7, 8, 9],
        vec![2],
        vec![3, 4, 5, 6, 7, 8],
        vec![4, 5, 6, 7, 8, 9],
        vec![3, 4, 5, 6, 7],
        vec![4, 5, 6, 7, 8],
        vec![5, 6, 7, 8, 9],
    ];
    let results = find_unique_combinations(&lists);

    /*
    for result in results.iter() {
        println!("{:?}", result);
    }
    println!("");
    println!("Belegungen für Neun: {}", results.len());

     */
    results
}
/*
Gibt die beiden möglichen Belegungen für die erste Spalte eines 4x4 Sudokus aus
 */
pub fn possibilities_first_column_four() -> Vec<Vec<usize>> {
    let first_result = vec![3, 2, 4];
    let second_result = vec![4, 2, 3];
    vec![first_result, second_result]
}

/*
Gibt die neun möglichen Belegungen für die erste Spalte eines 6x6 Sudokus aus
 */
pub fn possibilities_first_column_six() -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    results.push(vec![4, 2, 3, 5, 6]);
    results.push(vec![4, 2, 5, 3, 6]);
    results.push(vec![4, 2, 6, 3, 5]);
    results.push(vec![5, 2, 3, 4, 6]);
    results.push(vec![5, 2, 4, 3, 6]);
    results.push(vec![5, 2, 6, 3, 4]);
    results.push(vec![6, 2, 3, 4, 5]);
    results.push(vec![6, 2, 4, 3, 5]);
    results.push(vec![6, 2, 5, 3, 4]);

    for result in results.iter() {
        println!("{:?}", result);
    }
    println!("");
    println!("Belegungen für sechs: {}", results.len());
    results
}
/*
Berechnet verschiedene Belegungen für Teile der ersten Spalte unter Berücksichtigung der Symmetrie
 */
pub fn possibilities_not_complete_first_column(fields_of_first_columns: &Vec<usize>, grid_size: &usize) -> HashSet<Vec<usize>> {
    let GRID_SIZE = *grid_size;
    if fields_of_first_columns.len() > GRID_SIZE - 1 {
        panic!("So viele können gar nicht fest sein")
    }
    let results;
    if GRID_SIZE == 9{
        results = possibilities_first_column_nine();
    } else if GRID_SIZE == 4 {
        results = possibilities_first_column_four();
    } else if GRID_SIZE == 6 {
        results = possibilities_first_column_six();
    } else {
        results = Vec::new();
        panic!("falsche Gridsize");
    }
    let mut new_combinations = std::collections::HashSet::new();
    for combination in results.iter() {
        let mut new_combination = Vec::new();
        for number in 0..combination.len() {
            if fields_of_first_columns.contains(&number) {
                new_combination.push(combination[number]);
            } else {
                new_combination.push(0);
            }
        }
        new_combinations.insert(new_combination);
    }
    println!("Anzahl der möglichen Belegungen: {}", new_combinations.len());
    new_combinations
}

// Funktion, die alle möglichen Belegungen berechnet
fn find_unique_combinations(lists: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    let mut current_combination = Vec::new();
    let mut used_numbers = std::collections::HashSet::new();
    backtrack(0, &lists, &mut current_combination, &mut used_numbers, &mut results);
    results
}

// Hilfsfunktion für die Backtracking-Logik
fn backtrack(
    index: usize,
    lists: &[Vec<usize>],
    current_combination: &mut Vec<usize>,
    used_numbers: &mut std::collections::HashSet<usize>,
    results: &mut Vec<Vec<usize>>,
) {
    // Basisfall: Wenn wir durch alle Listen gegangen sind
    if index == lists.len() {
        if (current_combination[0]< current_combination[1]) && (current_combination[2]< current_combination[3]) && (current_combination[3]< current_combination[4]) &&
            (current_combination[5]< current_combination[6]) && (current_combination[6]< current_combination[7]) && (current_combination[2]< current_combination[5])
        {
            results.push(current_combination.clone());
        }
        return;
    }

    // Überprüfe jede Zahl in der aktuellen Liste
    for &number in &lists[index] {
        if !used_numbers.contains(&number) {
            // Füge die Zahl zur aktuellen Kombination hinzu und merke sie als "verwendet"
            current_combination.push(number);
            used_numbers.insert(number);

            // Rekursion für die nächsten Listen
            backtrack(index + 1, lists, current_combination, used_numbers, results);

            // Entferne die Zahl wieder, um den nächsten Zweig zu durchsuchen
            current_combination.pop();
            used_numbers.remove(&number);
        }
    }
}
