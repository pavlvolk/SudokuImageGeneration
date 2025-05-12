use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn set_values_from_solution(grid: &[usize]) -> HashSet<Vec<usize>> {
    let outcome = read_and_process_csv("permuted_solutions.txt", grid);
    if let Err(e) = &outcome {
        eprintln!("Fehler beim Verarbeiten der Datei: {}", e);
    }

    outcome.unwrap()
}

/*
Liest solution_9.txt ein, wo vollständige Sodokus drinnen stehen
Anschließend werden an den Stellen wo eine 1 im grid steht die jeweilige Zahl aus der Liste in solution_9 übertragen
Das entstandene partielle Sudoku wird in results hinzugefügt
Dieser Vorgang wird für alle Lösungslisten ausgeführt
 */
fn read_and_process_csv(file_path: &str, grid: &[usize]) -> Result<HashSet<Vec<usize>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut result = HashSet::new();


    let mut count = 0;
    let max_count = 10000;   //TODO in main übergeben
    for line_result in reader.lines() {
        if count > max_count{
            break
        }
        count += 1;
        let line = line_result?;
        let cleaned_line = line.trim().trim_start_matches('[').trim_end_matches(']');
        let list: Vec<usize> = cleaned_line
            .split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut grid_list = Vec::new();
        for i in  0..81{
            grid_list.push(list[i]*grid[i]);
        }
        result.insert(grid_list);
    }

    Ok(result)
}