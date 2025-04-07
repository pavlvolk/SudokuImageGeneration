const GRID_SIZE: usize = 9; // Dimension der 9x9-Matrix
const GRID_CELLS: usize = GRID_SIZE * GRID_SIZE; // Anzahl der Zellen in der Matrix

// Funktion zum Initialisieren eines Vektors der Größe 9 mit auf 0 gesetzten Werten
fn initialize_empty_sums() -> Vec<usize> {
    vec![0; GRID_SIZE]
}

// Funktion zum Finden des maximalen Werts und dessen Index
fn find_max_index(values: &[usize], not_available: &[usize]) -> usize {
    let mut max_value= 10; //TODO schönere Lösung finden
    let mut max_index= 10;
    for (i, &value) in values.iter().enumerate() {
        if(!not_available.contains(&i)){
            max_value = value;
            max_index = i; //TODO Reihenfolge vielleicht noch abändern für weniger Permutationen
        }
    }
    for (i, &value) in values.iter().enumerate() {
        if(!not_available.contains(&i)){
            if value > max_value {
                max_value = value;
                max_index = i;
            }
        }
    }
    max_index
}

fn find_max_triple(values: &[usize], not_available: &[usize]) -> (usize, usize, usize) {
    let mut max_index = find_max_index(values, not_available);
    let mut second_index;
    let mut third_index;
    if (max_index % 3) == 0 {
        if(values[max_index + 1] > values[max_index + 2]) {
            second_index = max_index + 1;
            third_index = max_index + 2;
        } else {
            second_index = max_index + 2;
            third_index = max_index + 1;
        }
    }
    else if(max_index % 3) == 1 {
        if(values[max_index - 1] > values[max_index + 1]) {
            second_index = max_index - 1;
            third_index = max_index + 1;
        } else {
            second_index = max_index + 1;
            third_index = max_index - 1;
        }
    }
    else{
        if(values[max_index - 2] > values[max_index - 1]) {
            second_index = max_index - 2;
            third_index = max_index - 1;
        } else {
            second_index = max_index - 1;
            third_index = max_index - 2;
        }
    }
    (max_index, second_index, third_index)
}

pub fn find_permutations(grid: &[usize]) -> (Vec<usize>, Vec<usize>, bool) {
    let mut row_permutation: Vec<usize> = Vec::new();
    let mut col_permutation: Vec<usize> = Vec::new();
    let mut mirror: bool = false;
    // Initialisierung der Summen
    let mut row_sums = initialize_empty_sums();
    let mut col_sums = initialize_empty_sums();
    let mut row_not_available: Vec<usize> = Vec::new();
    let mut col_not_available: Vec<usize> = Vec::new();

    // Berechnung der Summen für Reihen und Spalten
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            row_sums[row] += grid[row * GRID_SIZE + col];
            col_sums[col] += grid[row * GRID_SIZE + col];
        }
    }

    let (max_row, second_row, third_row) = find_max_triple(&row_sums, &row_not_available);
    let (max_col, second_col, third_col) = find_max_triple(&col_sums, &col_not_available);

    if col_sums[max_col] > row_sums[max_row] {
        mirror = true;
    }

    row_permutation.push(max_row);
    row_permutation.push(second_row);
    row_permutation.push(third_row);
    col_permutation.push(max_col);
    col_permutation.push(second_col);
    col_permutation.push(third_col);

    row_not_available.push(max_row);
    row_not_available.push(second_row);
    row_not_available.push(third_row);
    col_not_available.push(max_col);
    col_not_available.push(second_col);
    col_not_available.push(third_col);

    let (max_row, second_row, third_row) = find_max_triple(&row_sums, &row_not_available);
    let (max_col, second_col, third_col) = find_max_triple(&col_sums, &col_not_available);
    row_permutation.push(max_row);
    row_permutation.push(second_row);
    row_permutation.push(third_row);
    col_permutation.push(max_col);
    col_permutation.push(second_col);
    col_permutation.push(third_col);

    row_not_available.push(max_row);
    row_not_available.push(second_row);
    row_not_available.push(third_row);
    col_not_available.push(max_col);
    col_not_available.push(second_col);
    col_not_available.push(third_col);

    let (max_row, second_row, third_row) = find_max_triple(&row_sums, &row_not_available);
    let (max_col, second_col, third_col) = find_max_triple(&col_sums, &col_not_available);
    row_permutation.push(max_row);
    row_permutation.push(second_row);
    row_permutation.push(third_row);
    col_permutation.push(max_col);
    col_permutation.push(second_col);
    col_permutation.push(third_col);

    (row_permutation, col_permutation, mirror)
}
