pub fn sudoku_clauses(board_size: i16) -> Vec<Vec<i16>> {
    let mut clauses = vec![vec![]];
    one_number_each(&clauses, board_size);
    add_row_clauses(&clauses, board_size);
    add_column_clauses(&clauses, board_size);
    add_grid_clauses(&clauses, board_size);
    clauses
}

pub fn one_number_each(mut clauses: &Vec<Vec<i16>>, board_size: i16) {
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

pub fn add_row_clauses(mut clauses: &Vec<Vec<i16>>, board_size: i16) {
    //at least every number in each row
    for r in 1..=board_size {
        for val in 1..=board_size {
            let mut row_clauses:Vec<i16> = vec![];
            for c in 1..=board_size {
                row_clauses.push(var_num(r, c, val, board_size))
            }
            clauses.push(row_clauses);
        }
    }

    //at most every number per row
    for r in 1..=board_size {
        for val in 1..=board_size {
            for c1 in 1..board_size {
                for c2 in c1+1..=board_size {
                    clauses.push(vec![-var_num(r, c1, val, board_size), -var_num(r, c2, val, board_size)])
                }
            }
        }
    }
}

pub fn add_column_clauses(mut clauses: &Vec<Vec<i16>>, board_size: i16) {
    //at least every number in each row
    for c in 1..=board_size {
        for val in 1..=board_size {
            let mut row_clauses:Vec<i16> = vec![];
            for r in 1..=board_size {
                row_clauses.push(var_num(r, c, val, board_size))
            }
            clauses.push(row_clauses);
        }
    }

    //at most every number per row
    for c in 1..=board_size {
        for val in 1..=board_size {
            for r1 in 1..board_size {
                for r2 in r1+1..=board_size {
                    clauses.push(vec![-var_num(r1, c, val, board_size), -var_num(r2, c, val, board_size)])
                }
            }
        }
    }
}

pub fn add_grid_clauses(mut clauses: &Vec<Vec<i16>>, board_size: i16) {
    let mut r_board_size:i16 = 0;
    let mut c_board_size:i16 = 0;
    match board_size {
        4 => {r_board_size = 2; c_board_size = 2},
        6 => {r_board_size = 2; c_board_size = 3},
        9 => {r_board_size = 3; c_board_size = 3},
        _ => println!("Invalid board size!")
    }
    for br in 0..c_board_size {
        for bc in 0..r_board_size {
            for val in 1..=board_size {
                let mut block_clauses:Vec<i16> = vec![];
                for r in 1 + br * r_board_size..1 + (br + 1) * r_board_size {
                    for c in 1 + bc * c_board_size..1 + (bc + 1) * c_board_size {
                        block_clauses.push(var_num(r, c, val, board_size))
                    }
                }
                for i in 0..block_clauses.len() {
                    for j in 0..block_clauses.len() {
                        clauses.push(vec![-block_clauses[i], -block_clauses[j]]);
                    }
                }
                clauses.push(block_clauses);
            }
        }
    }
}

fn var_num(row: i16, column: i16, val: i16, board_size: i16) -> i16{
    (row-1) * board_size * board_size + (column-1) * board_size + val
}