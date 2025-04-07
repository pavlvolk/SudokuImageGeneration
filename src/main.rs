mod sort;
mod apply_permutations;
mod possibilities_first_column; // Added to ensure the module is included
use possibilities_first_column::possibilities_not_complete_first_column;

use sort::find_permutations;
use apply_permutations::apply_permutations;

fn main() {
// Ensure the function definition exists and matches your intent:
    possibilities_not_complete_first_column(&vec![1, 4, 6]);
    let mut grid = vec![0; 81];
    let mut fields = vec![0, 4, 9, 17, 18, 21, 25, 28, 29, 31, 32, 33, 40, 41, 42, 43, 45, 46, 50, 51, 54, 55, 60, 61, 67, 68, 73, 75, 76, 78 ];

    for field in fields{
        grid[field] = 1;
    }

    for row in 0..9{
        for col in 0..9{
            print!("{} ", grid[row*9+col]);
        }
        println!("");}
    println!("");

    let (row_permutation, col_permutation, mirror) = find_permutations(&grid);
    println!("{:?}", row_permutation);
    println!("{:?}", col_permutation);
    println!("{:?}", mirror);
    let mut grid_permuted = apply_permutations(&grid, &row_permutation, &col_permutation, &mirror);
    for row in 0..9{
        for col in 0..9{
            print!("{} ", grid_permuted[row*9+col]);
        }
        println!("");}

}

