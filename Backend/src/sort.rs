/*
Findet Index mit dem maximalen Wert ohne dabei Bereits betrachtete Elemente mit einzubeziehen
 */
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
} //TODO bei gleichen Werten den besten Block wählen

/*
Findet aus den verbleibenden Werten das Maximum und die zwei zugehörigen Werte in absteigender Reihenfolge
 */
fn find_max_triple(values: &[usize], not_available: &[usize]) -> (usize, usize, usize) {
    let mut max_index = find_max_index(values, not_available);
    let mut second_index;
    let mut third_index;
    if (max_index % 3) == 0 {
        if(values[max_index + 1] >= values[max_index + 2]) {
            second_index = max_index + 1;
            third_index = max_index + 2;
        } else {
            second_index = max_index + 2;
            third_index = max_index + 1;
        }
    }
    else if(max_index % 3) == 1 {
        if(values[max_index - 1] >= values[max_index + 1]) {
            second_index = max_index - 1;
            third_index = max_index + 1;
        } else {
            second_index = max_index + 1;
            third_index = max_index - 1;
        }
    }
    else{
        if(values[max_index - 2] >= values[max_index - 1]) {
            second_index = max_index - 2;
            third_index = max_index - 1;
        } else {
            second_index = max_index - 1;
            third_index = max_index - 2;
        }
    }
    (max_index, second_index, third_index)
}

/*
Findet aus den verbleibenden Werten das Maximum und den zugehörigen Wert
 */
fn find_max_double(values: &[usize], not_available: &[usize]) -> (usize, usize) {
    let mut max_index = find_max_index(values, not_available);
    let mut second_index;
    if (max_index % 2) == 0 {
        second_index = max_index + 1;
    }
    else{
        second_index = max_index - 1;
    }
    (max_index, second_index)
}

pub fn find_permutations(grid: &[usize], grid_size: &usize) -> (Vec<usize>, Vec<usize>, bool) {
    let GRID_SIZE = *grid_size;
    if GRID_SIZE != 9 && GRID_SIZE != 4 && GRID_SIZE != 6 {
        panic!("Grid size must be 9, 4 or 6");
    }

    // Initialisierung der Summen
    let mut row_sums = vec![0; GRID_SIZE];
    let mut col_sums = vec![0; GRID_SIZE];

    // Berechnung der Summen für Reihen und Spalten
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            row_sums[row] += grid[row * GRID_SIZE + col];
            col_sums[col] += grid[row * GRID_SIZE + col];
        }
    }
    if GRID_SIZE == 9 {
        compute_permutations_nine(&row_sums, &col_sums)
    } else if GRID_SIZE == 4 {
        compute_permutations_four(&row_sums, &col_sums)
    } else {
        compute_permutations_six(&row_sums, &col_sums)
    }

}

/*
Berechnet Permutation die dann auf die Spalten und Zeilen angewendet werden sollen um die beste Sortierung zu erreichen für 9*9
 */
fn compute_permutations_nine(row_sums: &Vec<usize>, col_sums: &Vec<usize>) -> (Vec<usize>, Vec<usize>, bool) {
    let mut row_permutation: Vec<usize> = Vec::new();
    let mut col_permutation: Vec<usize> = Vec::new();
    let mut row_not_available: Vec<usize> = Vec::new();
    let mut col_not_available: Vec<usize> = Vec::new();

    let mut mirror: bool = false;

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

/*
Berechnet Permutation die dann auf die Spalten und Zeilen angewendet werden sollen um die beste Sortierung zu erreichen für 4*4
 */
fn compute_permutations_four(row_sums: &Vec<usize>, col_sums: &Vec<usize>) -> (Vec<usize>, Vec<usize>, bool) {
    let mut row_permutation: Vec<usize> = Vec::new();
    let mut col_permutation: Vec<usize> = Vec::new();
    let mut row_not_available: Vec<usize> = Vec::new();
    let mut col_not_available: Vec<usize> = Vec::new();

    let mut mirror: bool = false;

    let (max_row, second_row) = find_max_double(&row_sums, &row_not_available);
    let (max_col, second_col) = find_max_double(&col_sums, &col_not_available);

    if col_sums[max_col] > row_sums[max_row] {
        mirror = true;
    }

    row_permutation.push(max_row);
    row_permutation.push(second_row);
    col_permutation.push(max_col);
    col_permutation.push(second_col);

    row_not_available.push(max_row);
    row_not_available.push(second_row);
    col_not_available.push(max_col);
    col_not_available.push(second_col);

    let (max_row, second_row) = find_max_double(&row_sums, &row_not_available);
    let (max_col, second_col) = find_max_double(&col_sums, &col_not_available);

    row_permutation.push(max_row);
    row_permutation.push(second_row);
    col_permutation.push(max_col);
    col_permutation.push(second_col);

    (row_permutation, col_permutation, mirror)
}

/*
Berechnet Permutation die dann auf die Spalten und Zeilen angewendet werden sollen um die beste Sortierung zu erreichen für 6*6
6*6 können nicht gespiegelt werden, da sonst die Ausrichtung der Quader verloren gehen würde
 */
fn compute_permutations_six(row_sums: &Vec<usize>, col_sums: &Vec<usize>) -> (Vec<usize>, Vec<usize>, bool) {
    let mut row_permutation: Vec<usize> = Vec::new();
    let mut col_permutation: Vec<usize> = Vec::new();
    let mut row_not_available: Vec<usize> = Vec::new();
    let mut col_not_available: Vec<usize> = Vec::new();

    let (max_row, second_row) = find_max_double(&row_sums, &row_not_available);

    row_permutation.push(max_row);
    row_permutation.push(second_row);
    row_not_available.push(max_row);
    row_not_available.push(second_row);

    let (max_row, second_row) = find_max_double(&row_sums, &row_not_available);

    row_permutation.push(max_row);
    row_permutation.push(second_row);
    row_not_available.push(max_row);
    row_not_available.push(second_row);

    let (max_row, second_row) = find_max_double(&row_sums, &row_not_available);

    row_permutation.push(max_row);
    row_permutation.push(second_row);

    let (max_col, second_col, third_col) = find_max_triple(&col_sums, &col_not_available);
    col_permutation.push(max_col);
    col_permutation.push(second_col);
    col_permutation.push(third_col);
    col_not_available.push(max_col);
    col_not_available.push(second_col);
    col_not_available.push(third_col);

    let (max_col, second_col, third_col) = find_max_triple(&col_sums, &col_not_available);
    col_permutation.push(max_col);
    col_permutation.push(second_col);
    col_permutation.push(third_col);

    (row_permutation, col_permutation, false)
}