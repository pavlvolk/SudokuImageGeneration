pub fn sudoku_clauses(board_size: usize) -> Vec<Vec<i16>> {
    let mut clauses = vec![vec![]];

    clauses
}

pub fn one_number_each(clauses: &mut [i16], board_size: usize) {
    //at least one number
    for r in 1..board_size {
        for c in 1..board_size {
            let mut cell_clauses:Vec<usize> = vec![];
            for val in 1..board_size {
                cell_clauses.push(var_num(r, c, val, board_size))
            }
            clauses.push(cell_clauses);
        }
    }

    //At most one number

}

fn var_num(row: usize, column: usize, val: usize, board_size: usize) -> usize{
    (row-1) * board_size * board_size + (column-1) * board_size + val
}