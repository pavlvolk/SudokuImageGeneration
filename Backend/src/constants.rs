use once_cell::sync::Lazy;

pub const SOLUTION: &str = "data/permuted_10_million.txt";
pub const TEST: &str = "data/unbiased_sudokus_formated.txt";

pub static NUMBER_OF_THREADS: Lazy<usize> = Lazy::new(|| num_cpus::get());

pub const SOLUTION_PER_THREAD: usize = 20000;