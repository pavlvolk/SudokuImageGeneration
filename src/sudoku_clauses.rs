pub fn sudoku_clauses(board_size: i16) -> Vec<Vec<i16>> {
    let mut clauses = vec![vec![]];

    clauses
}

pub fn one_number_each(clauses: &mut [i16], board_size: i16) {
    //at least one number
    for r in 1..=board_size {
        for c in 1..=board_size {
            let mut cell_clauses:Vec<i16> = vec![];
            for val in 1..=board_size {
                cell_clauses.push(var_num(r, c, val, board_size))
            }
            clauses.push(cell_clauses);
        }
    }

    //At most one number
    for r in 1..=board_size {
        for c in 1..=board_size {
            for val1 in 1..board_size {
                for val2 in val1+1..=board_size {
                    clauses.push(vec![-var_num(r, c, val1, board_size), -var_num(r, c, val2, board_size)])
                }
            }
        }
    }
}

fn var_num(row: i16, column: i16, val: i16, board_size: i16) -> i16{
    (row-1) * board_size * board_size + (column-1) * board_size + val
}