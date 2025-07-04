use std::collections::{HashMap, HashSet};

fn rate_difficulty(list: &Vec<i32>, size: i32) -> f32{
    return 0.0
}

pub fn serate(list: &mut Vec<Vec<i32>>) -> f64{
    let board_size = list.len();
    let hidden_single_d = 1.2;
    let naked_single_d = 2.3;
    let pointing_d = 2.6;
    let claiming_d = 2.8;
    let x_wing_d = 4.0;
    let swordfish_d = 5.0;
    let mut candidates = initial_candidates(&list, board_size as i32);
    println!("{:?}", naked_single(&mut candidates, list));
    println!("{:?}", hidden_single(&mut candidates, board_size as i32, list));
    println!("{:?}", apply_pointing_pair(&mut candidates, list));
    print!("{:?}", candidates);
    return 0.0;
}

pub fn initial_candidates(list: &Vec<Vec<i32>>, board_size: i32) -> HashMap<(i32, i32), Vec<i32>>{
    let mut candidates: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    for r in 0..board_size {
        for c in 0..board_size {
            if list[r as usize][c as usize] != 0 {
                candidates.entry((r,c)).or_insert(vec![]).push(list[r as usize][c as usize]);
            }else{
                for val in 1..=board_size {
                    candidates.entry((r,c)).or_insert(vec![]).push(val);
                }
            }
        }
    }
    candidates
}

pub fn get_all_units(board_size: i32) -> Vec<Vec<Vec<(i32, i32)>>> {
    let mut row_units: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut col_units: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut subgrid_units: Vec<Vec<(i32, i32)>> = Vec::new();
    let subgrid_size_vertical = board_size.isqrt();
    let mut subgrid_size_horizontal = subgrid_size_vertical;
    if board_size == 6{
        subgrid_size_horizontal += 1;
    }
    for r in 0..board_size {
        let mut row: Vec<(i32, i32)> = Vec::new();
        for c in 0..board_size {
            row.push((r, c));
        }
        row_units.push(row);
    }

    for c in 0..board_size {
        let mut col: Vec<(i32, i32)> = Vec::new();
        for r in 0..board_size {
            col.push((r, c));
        }
        col_units.push(col);
    }


    for i in 0..subgrid_size_horizontal{
        for j in 0..subgrid_size_vertical{
            let mut subgrid: Vec<(i32, i32)> = Vec::new();
            for k in 0..subgrid_size_vertical{
                for l in 0..subgrid_size_horizontal{
                    subgrid.push((i*subgrid_size_vertical+k, j*subgrid_size_horizontal+l));
                }
            }
            subgrid_units.push(subgrid);
        }
    }


    let mut hm =Vec::new();
    hm.push(row_units);
    hm.push(col_units);
    hm.push(subgrid_units);
    hm
}

fn hidden_single(candidates: &mut HashMap<(i32, i32), Vec<i32>>, board_size: i32, list: &mut Vec<Vec<i32>>) -> bool{
    let unit_types = get_all_units(board_size);
    for units in unit_types {
        for unit in units{
            let mut count:HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
            for (r, c) in unit{
                for val in candidates.get(&(r,c)).unwrap(){
                    count.entry(*val).or_insert(vec![]).push((r, c));
                }
            }
            for (val, cells) in count{
                if cells.len() == 1{
                    let (r,c) = cells[0];
                    //fix hopefully
                    if list[r as usize][c as usize] != 0{
                        break;
                    }
                    let mut _cand = candidates.get_mut(&(r,c)).unwrap();
                    _cand = &mut vec![val];
                    list[r as usize][c as usize] = val;
                    println!("{:?} {:?}", (r, c), val);
                    return true;
                }
            }
        }
    }
    false
}

fn naked_single(candidates: &mut HashMap<(i32, i32), Vec<i32>>, list: &mut Vec<Vec<i32>>) -> bool {
    for ((r, c), val) in candidates.clone(){
        if val.len() == 1 && list[r as usize][c as usize] == 0{
            let mut _cand = candidates.get_mut(&(r, c)).unwrap();
            _cand = &mut vec![val[0]];
            list[r as usize][c as usize] = val[0];
            println!("{:?} {:?}", (r, c), val[0]);
            return true;
        }
    }
    false
}

fn apply_pointing_pair(candidates: &mut HashMap<(i32, i32), Vec<i32>>, board: &mut Vec<Vec<i32>>) -> bool {
    let mut changed = false;
    
    let vertical_stepsize = board.len().isqrt();
    let mut horizontal_stepsize = vertical_stepsize;
    if board.len() == 6{
        horizontal_stepsize += 1;
    }
    
    for box_r in (0..board.len()).step_by(vertical_stepsize) {
        for box_c in (0..board.len()).step_by(horizontal_stepsize) {
            let mut pos_by_val: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();

            // Collect candidates in the box
            for r in box_r..box_r + vertical_stepsize {
                for c in box_c..box_c + horizontal_stepsize {
                    if let Some(cell_cands) = candidates.get(&(r as i32, c as i32)) {
                        for &val in cell_cands {
                            pos_by_val.entry(val).or_default().push((r, c));
                        }
                    }
                }
            }

            // Check for values that appear only in one row or column in the box
            for (val, positions) in pos_by_val {
                let rows: HashSet<_> = positions.iter().map(|(r, _)| r).collect(); //rows where that value appears
                let cols: HashSet<_> = positions.iter().map(|(_, c)| c).collect();

                if rows.len() == 1 {
                    let row = *rows.iter().next().unwrap();
                    for col in 0..board.len() {
                        if !(box_c..box_c + horizontal_stepsize).contains(&col) && board[*row][col] == 0 {
                            if let Some(c) = candidates.get_mut(&(*row as i32, col as i32)) {
                                if c.contains(&val) {
                                    let index = c.iter().position(|n| *n == val).unwrap();
                                    c.remove(index);
                                    changed = true;
                                }
                            }
                        }
                    }
                }

                if cols.len() == 1 {
                    let col = *cols.iter().next().unwrap();
                    for row in 0..board.len() {
                        if !(box_r..box_r + vertical_stepsize).contains(&row) && board[row][*col] == 0 {
                            if let Some(c) = candidates.get_mut(&(row as i32, *col as i32)) {
                                if c.contains(&val) {
                                    let index = c.iter().position(|n| *n == val).unwrap();
                                    c.remove(index);
                                    println!("{:?}", c);
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    changed
}
