const GRID_SIZE: usize = 9; // Dimension der 9x9-Matrix
const GRID_CELLS: usize = GRID_SIZE * GRID_SIZE; // Anzahl der Zellen in der Matrix

pub fn apply_permutations(grid: &[usize], row_permutations: &[usize], col_permutations: &[usize], mirror: &bool) -> Vec<usize> {
    let mut permuted_grid: Vec<usize> = Vec::new();
    let mut mirrored_permuted_grid: Vec<usize> = Vec::new();

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