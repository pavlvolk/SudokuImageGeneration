/*
wendet die Permutationen auf das Grid an
 */
pub fn apply_permutations(grid: &[usize], row_permutations: &[usize], col_permutations: &[usize], mirror: &bool, grid_size: &usize) -> Vec<usize> {
    let mut permuted_grid: Vec<usize> = Vec::new();
    let mut mirrored_permuted_grid: Vec<usize> = Vec::new();
    let GRID_SIZE: usize = *grid_size;

    for row in 0..GRID_SIZE{
        for col in 0..GRID_SIZE{
            permuted_grid.push(grid[(row_permutations[row] * GRID_SIZE + col_permutations[col])]);
        }
    }
    if *mirror {
        for row in 0..GRID_SIZE{
            for col in 0..GRID_SIZE{
                mirrored_permuted_grid.push(permuted_grid[col * GRID_SIZE + row]);
            }
        }
        return mirrored_permuted_grid;
    }
    permuted_grid

}

pub fn apply_reverse_permutations(grid: &[usize], row_permutations: &[usize], col_permutations: &[usize], mirror: &bool, grid_size: &usize) -> Vec<usize> {
    let GRID_SIZE: usize = *grid_size;
    let row_permutations: Vec<usize> = row_permutations.to_vec();
    let col_permutations: Vec<usize> = col_permutations.to_vec();

    let mut reverse_row_permutations: Vec<usize> = inverse_permutation(&row_permutations);

    let mut reverse_col_permutations: Vec<usize> = inverse_permutation(&col_permutations);

    let mut mirrored_grid: Vec<usize> = Vec::new();
    let mut permuted_grid: Vec<usize> = Vec::new();
    permuted_grid = grid.into();

    if *mirror {
        for row in 0..GRID_SIZE{
            for col in 0..GRID_SIZE{
                mirrored_grid.push(grid[col * GRID_SIZE + row]);
            }
        }
        permuted_grid = mirrored_grid;
    }

    let mut final_grid: Vec<usize> = Vec::new();

    for row in 0..GRID_SIZE{
        for col in 0..GRID_SIZE{
            final_grid.push(permuted_grid[(reverse_row_permutations[row] * GRID_SIZE + reverse_col_permutations[col])]);
        }
    }

    final_grid
}

fn inverse_permutation(permutation: &[usize]) -> Vec<usize> {
    let mut inverse = vec![0; permutation.len()];
    for (i, &p) in permutation.iter().enumerate() {
        inverse[p] = i;
    }
    inverse
}
