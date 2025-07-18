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
    //println!("{:?}", naked_single(&mut candidates, list));
    //println!("{:?}", hidden_single(&mut candidates, board_size as i32, list));
    println!("{:?}", apply_pointing_pair(&mut candidates, list));
    //println!("{:?}", apply_claiming_pair(&mut candidates, list));
    println!("{:?}", candidates);
    return 0.0;
}

pub fn initial_candidates(list: &Vec<Vec<i32>>, board_size: i32) -> HashMap<(i32, i32), Vec<i32>>{
    let vertical_box_size = board_size.isqrt();
    let mut horizontal_box_size = vertical_box_size;
    if board_size == 6{
        horizontal_box_size += 1;
    }
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
    for r in 0..board_size {
        for c in 0..board_size {
            if list[r as usize][c as usize] != 0 {
                let val = list[r as usize][c as usize];
                for inner_r in 0..board_size {
                    if let Some(cell) = candidates.get_mut(&(inner_r,c)){
                        println!("{}", list[inner_r as usize][c as usize]);
                        if cell.contains(&val) && r != inner_r && list[inner_r as usize][c as usize] == 0{
                            let index = cell.iter().position(|n| *n == val).unwrap();
                            cell.remove(index);
                            println!("{} {} {}", inner_r, c, val)
                        }
                    }
                }
                for inner_c in 0..board_size {
                    if let Some(cell) = candidates.get_mut(&(r,inner_c)){
                        if cell.contains(&(val)) && c != inner_c && list[r as usize][inner_c as usize] == 0{
                            let index = cell.iter().position(|n| *n == val).unwrap();
                            cell.remove(index);
                            println!("{} {} {}", r, inner_c, val)
                        }
                    }
                }
                let block_corner_r = (r / vertical_box_size)*vertical_box_size;
                let block_corner_c = (c / horizontal_box_size)*horizontal_box_size;
                for br in block_corner_r..block_corner_r + vertical_box_size {
                    for bc in block_corner_c..block_corner_c + horizontal_box_size {
                        if let Some(cell) = candidates.get_mut(&(br,bc)){
                            if cell.contains(&val) && r != br && c != bc && list[br as usize][bc as usize] == 0{
                                let index = cell.iter().position(|n| *n == val).unwrap();
                                cell.remove(index);
                            }
                        }
                    }
                }
            
            }
        }
    }
    candidates
}

pub fn get_all_units(board_size: i32) -> Vec<Vec<(i32, i32)>> {
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

    row_units.append(&mut col_units);
    row_units.append(&mut subgrid_units);
    row_units
}

fn hidden_single(candidates: &mut HashMap<(i32, i32), Vec<i32>>, board_size: i32, list: &mut Vec<Vec<i32>>) -> bool{
    let units = get_all_units(board_size);
    let mut changed = false;
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
                candidates.insert((r, c), vec![val]);
                list[r as usize][c as usize] = val; //TODO unnecessary
                println!("{:?} {:?}", (r, c), val);
                changed = true;
            }
        }
    }
    changed
}

//TODO Funktion eigentlich unnötig zum lösen, nur für schwierigkeit zu gebrauchen.
fn naked_single(candidates: &mut HashMap<(i32, i32), Vec<i32>>, list: &mut Vec<Vec<i32>>) -> bool {
    let mut changed = false;
    for ((r, c), val) in candidates.clone(){
        if val.len() == 1 && list[r as usize][c as usize] == 0{
            let mut _cand = candidates.get_mut(&(r, c)).unwrap();
            _cand = &mut vec![val[0]];
            list[r as usize][c as usize] = val[0];
            println!("{:?}", candidates.get(&(r, c)).unwrap());
            changed = true;
        }
    }
    changed
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
                let rows: HashSet<_> = positions.iter().map(|(r, _)| r).collect();
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

fn apply_claiming_pair(candidates: &mut HashMap<(i32, i32), Vec<i32>>, board: &mut Vec<Vec<i32>>) -> bool {
    let vertical_box_size = board.len().isqrt();
    let mut horizontal_box_size = vertical_box_size;
    if board.len() == 6{
        horizontal_box_size += 1;
    }
    let mut changed = false;
    for r in 0..board.len(){
        for val in 1..=board.len(){
            let mut boxes:Vec<i32> = vec![];
            for c in 0..board.len(){
                if let Some(cell_cands) = candidates.get(&(r as i32, c as i32)){
                    if cell_cands.contains(&(val as i32)){
                        if !boxes.contains(&((c % horizontal_box_size) as i32)){
                            boxes.push((c % horizontal_box_size) as i32)
                        }
                    }
                }
            }
            if boxes.len() == 1{
                let c = *boxes.iter().next().unwrap() as usize;
                let box_corner_r = r%(vertical_box_size);
                let box_corner_c = c*horizontal_box_size;
                for br in box_corner_r..box_corner_r + vertical_box_size {
                    for bc in box_corner_c..box_corner_c + horizontal_box_size {
                        if (box_corner_r + br) != r {
                            if let Some(cell) = candidates.get_mut(&((box_corner_r + br) as i32, (box_corner_c + bc) as i32)){ //TODO hier nur br, bc??
                                if cell.contains(&(val as i32)) {
                                    let index = cell.iter().position(|n| *n == val as i32).unwrap();
                                    cell.remove(index);
                                    println!("{:?} {:?}", (box_corner_r + br, box_corner_c + bc), val);
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    for c in 0..board.len(){
        for val in 0..=board.len(){
            let mut boxes:Vec<i32> = vec![];
            for r in 0..board.len(){
                if let Some(cell_cands) = candidates.get(&(r as i32, c as i32)){
                    if cell_cands.contains(&(val as i32)){
                        if !boxes.contains(&((r % (vertical_box_size)) as i32)){
                            boxes.push((r % (vertical_box_size)) as i32)
                        }
                    }
                }
            }

            if boxes.len() == 1{
                let r = *boxes.iter().next().unwrap() as usize;
                let box_corner_r = r*(vertical_box_size);
                let box_corner_c = c%(horizontal_box_size);
                for br in box_corner_r..box_corner_r + vertical_box_size {
                    for bc in box_corner_c..box_corner_c + horizontal_box_size {
                        if (box_corner_c + bc) != c {
                            if let Some(cell) = candidates.get_mut(&((box_corner_r + br) as i32, (box_corner_c + bc) as i32)){
                                if cell.contains(&(val as i32)) {
                                    let index = cell.iter().position(|n| *n == val as i32).unwrap();
                                    cell.remove(index);
                                    println!("{:?} {:?}", (box_corner_r + br, box_corner_c + bc), val);
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

fn apply_x_wing(candidates: &mut HashMap<(i32, i32), Vec<i32>>, board: &mut Vec<Vec<i32>>) -> bool {
    let mut changed = false;
    for val in 1..=board.len(){
        let mut row_to_cols: HashMap<usize, Vec<usize>> = HashMap::new();
        for r in 0..board.len(){
            let col = (0..board.len()).filter(|&c| candidates.get(&(r as i32, c as i32)).map_or(false, |s| s.contains(&(val as i32)))).collect::<Vec<_>>();
            if col.len() == 2{
                row_to_cols.insert(r, col);
            }
        }

        let rows = row_to_cols.keys().cloned().collect::<Vec<_>>();
        for i in 0..rows.len(){
            for j in i+1..rows.len(){
                let r1 = rows[i];
                let r2 = rows[j];
                if row_to_cols[&r1] == row_to_cols[&r2]{
                    let cols = &row_to_cols[&r1];
                    for r in 0..board.len(){
                        if r != r1 && r != r2 {
                            for &c in cols{
                                if let Some(cell_cands) = candidates.get_mut(&(r as i32, c as i32)){
                                    if cell_cands.contains(&(val as i32)){
                                        cell_cands.remove(r);
                                        changed = true;
                                    }
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
