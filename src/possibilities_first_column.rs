fn possibilities_first_column() -> Vec<Vec<i32>> {

    let lists = vec![
        vec![4, 5, 6, 7, 8], // list1
        vec![5, 6, 7, 8, 9], // list2
        vec![2, 3, 4, 5, 6], // list3
        vec![3, 4, 5, 6, 7, 8], // list4
        vec![4, 5, 6, 7, 8, 9], // list5
        vec![3, 4, 5, 6, 7], // list6
        vec![4, 5, 6, 7, 8], // list7
        vec![5, 6, 7, 8, 9], // list8
    ];

    let results = find_unique_combinations(&lists);
    println!("Anzahl der möglichen Belegungen: {}", results.len());

    results
}

pub fn possibilities_not_complete_first_column(fields_of_first_columns: &Vec<usize>) {
    let results = possibilities_first_column();
    let mut new_combinations = std::collections::HashSet::new();
    for combination in results.iter() {
        let mut new_combination = Vec::new();
        for field in fields_of_first_columns.iter() {
            new_combination.push(combination[*field-1]);
        }
        new_combinations.insert(new_combination);
    }
    println!("Anzahl der möglichen Belegungen: {}", new_combinations.len());
}

// Funktion, die alle möglichen Belegungen berechnet
fn find_unique_combinations(lists: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut current_combination = Vec::new();
    let mut used_numbers = std::collections::HashSet::new();
    backtrack(0, &lists, &mut current_combination, &mut used_numbers, &mut results);
    results
}

// Hilfsfunktion für die Backtracking-Logik
fn backtrack(
    index: usize,
    lists: &[Vec<i32>],
    current_combination: &mut Vec<i32>,
    used_numbers: &mut std::collections::HashSet<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    // Basisfall: Wenn wir durch alle Listen gegangen sind
    if index == lists.len() {
        results.push(current_combination.clone());
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