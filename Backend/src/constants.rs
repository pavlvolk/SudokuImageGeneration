use once_cell::sync::Lazy;


pub const SOLUTION: &str = "data/permuted_10_million_cut_trimmed.txt";

pub const TEST: &str = "data/unbiased_sudokus_formated.txt";

pub static NUMBER_OF_THREADS: Lazy<usize> = Lazy::new(|| 6);

pub const SOLUTION_PER_THREAD: usize = 10000;