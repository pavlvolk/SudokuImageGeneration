use once_cell::sync::Lazy;


pub const SOLUTION: &str = "data/generated_cut_80.txt";
pub const SOLUTION_4: &str = "data/sudoku_solutions_4x4.txt";
pub const SOLUTION_6: &str = "data/sudoku_solutions_6x6.txt";
pub const TEST: &str = "data/unbiased_sudokus_formated.txt";
pub static NUMBER_OF_THREADS: Lazy<usize> = Lazy::new(|| num_cpus::get_physical());
pub const SOLUTION_PER_THREAD: usize = 10000;