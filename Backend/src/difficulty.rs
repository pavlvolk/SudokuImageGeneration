use std::collections::{HashMap, HashSet};

pub fn rate_difficulty(list: Vec<usize>) -> f64{
    let new_list = list.into_iter().map(|x| x as i32).collect::<Vec<i32>>();
    let mut list_2d = new_list.chunks(new_list.len().isqrt()).map(|row| row.to_vec()).collect::<Vec<Vec<i32>>>();
    serate(&mut list_2d)
}

fn serate(list: &mut Vec<Vec<i32>>) -> f64{
    let board_size = list.len();
    let hidden_single_d = 1.2;
    let naked_single_d = 2.3;
    let pointing_d = 2.6;
    let claiming_d = 2.8;
    let x_wing_d = 4.0;
    let mut difficulty: f64 = 0.0;
    let mut candidates = initial_candidates(&list, board_size as i32);
    display_candidates(&candidates);
    loop{
        println!("Candidates: {:?}", list);
        display_candidates(&candidates);
        compute_candidates(&mut candidates, board_size as i32);
        let mut solved = true;
        for (_, vals) in candidates.iter(){
            if vals.len() != 1{
                solved = false;
            }
        }
        if solved{
            display_candidates(&candidates);
            return difficulty;
        }
        compute_candidates(&mut candidates, board_size as i32);
        println!("Naked: {:?}", list);
        display_candidates(&candidates);
        if naked_single(&mut candidates, list) {
           difficulty = difficulty.max(naked_single_d);
            continue;
        }
        compute_candidates(&mut candidates, board_size as i32);
        println!("Hidden: {:?}", list);
        display_candidates(&candidates);
        if hidden_single(&mut candidates, board_size as i32, list){
            difficulty = difficulty.max(hidden_single_d);
            continue;
        }
        compute_candidates(&mut candidates, board_size as i32);
        println!("Pointing: {:?}", list);
        display_candidates(&candidates);
        if apply_pointing_pair(&mut candidates, list){
            difficulty = difficulty.max(pointing_d);
            continue;
        }
        compute_candidates(&mut candidates, board_size as i32);
        println!("Claiming: {:?}", list);
        display_candidates(&candidates);
        if apply_claiming_pair(&mut candidates, list){
            difficulty = difficulty.max(claiming_d);
            continue;
        }
        compute_candidates(&mut candidates, board_size as i32);
        println!("X: {:?}", list);
        display_candidates(&candidates);
        if apply_x_wing(&mut candidates, list){
            difficulty = difficulty.max(x_wing_d);
            continue;
        }
        println!("{:?}", list);
        return 5.0
    }
    5.0
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
                        //println!("{}", list[inner_r as usize][c as usize]);
                        if cell.contains(&val) && r != inner_r && list[inner_r as usize][c as usize] == 0{
                            let index = cell.iter().position(|n| *n == val).unwrap();
                            cell.remove(index);
                            //println!("{} {} {}", inner_r, c, val)
                        }
                    }
                }
                for inner_c in 0..board_size {
                    if let Some(cell) = candidates.get_mut(&(r,inner_c)){
                        if cell.contains(&(val)) && c != inner_c && list[r as usize][inner_c as usize] == 0{
                            let index = cell.iter().position(|n| *n == val).unwrap();
                            cell.remove(index);
                            //println!("{} {} {}", r, inner_c, val)
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

pub fn compute_candidates(candidates: &mut HashMap<(i32, i32), Vec<i32>>, board_size: i32) -> &mut HashMap<(i32, i32), Vec<i32>> {
    let vertical_box_size = board_size.isqrt();
    let mut horizontal_box_size = vertical_box_size;
    if board_size == 6{
        horizontal_box_size += 1;
    }

    for r in 0..board_size {
        for c in 0..board_size {
            if candidates.get(&(r, c)).unwrap().len() == 1 {
                if let Some(vals) = candidates.get(&(r, c)) {
                    let val = vals[0];
                    for inner_r in 0..board_size {
                        if let Some(cell) = candidates.get_mut(&(inner_r, c)) {
                            //println!("{}", list[inner_r as usize][c as usize]);
                            if cell.contains(&val) && r != inner_r /*&& candidates.get(&(inner_r, c)).unwrap().len() != 1*/{
                                let index = cell.iter().position(|n| *n == val).unwrap();
                                cell.remove(index);
                                //println!("{} {} {}", inner_r, c, val)
                            }
                        }
                    }
                    for inner_c in 0..board_size {
                        if let Some(cell) = candidates.get_mut(&(r, inner_c)) {
                            if cell.contains(&(val)) && c != inner_c /*&& candidates.get(&(r, inner_c)).unwrap().len() != 1*/{
                                let index = cell.iter().position(|n| *n == val).unwrap();
                                cell.remove(index);
                                //println!("{} {} {}", r, inner_c, val)
                            }
                        }
                    }
                    let block_corner_r = (r / vertical_box_size) * vertical_box_size;
                    let block_corner_c = (c / horizontal_box_size) * horizontal_box_size;
                    for br in block_corner_r..block_corner_r + vertical_box_size {
                        for bc in block_corner_c..block_corner_c + horizontal_box_size {
                            if let Some(cell) = candidates.get_mut(&(br, bc)) {
                                if cell.contains(&val) && r != br && c != bc /*&& candidates.get(&(br, bc)).unwrap().len() != 1*/{
                                    let index = cell.iter().position(|n| *n == val).unwrap();
                                    cell.remove(index);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    candidates
}

fn new_compute_candidates(candidates: &mut HashMap<(i32, i32), Vec<i32>>, board_size: i32) {
    let vertical_box_size = board_size.isqrt();
    let mut horizontal_box_size = vertical_box_size;
    if board_size == 6 {
        horizontal_box_size += 1;
    }

    // Step 1: Collect all known values
    let solved_cells: Vec<((i32, i32), i32)> = candidates.iter()
        .filter_map(|(&(r, c), vals)| if vals.len() == 1 {
            Some(((r, c), vals[0]))
        } else {
            None
        })
        .collect();

    // Step 2: Remove each known value from peers
    for ((r, c), val) in solved_cells {
        // Remove from same column
        for inner_r in 0..board_size {
            if inner_r != r {
                if let Some(cell) = candidates.get_mut(&(inner_r, c)) {
                    cell.retain(|&x| x != val);
                }
            }
        }
        // Remove from same row
        for inner_c in 0..board_size {
            if inner_c != c {
                if let Some(cell) = candidates.get_mut(&(r, inner_c)) {
                    cell.retain(|&x| x != val);
                }
            }
        }
        // Remove from same box
        let box_corner_r = (r / vertical_box_size) * vertical_box_size;
        let box_corner_c = (c / horizontal_box_size) * horizontal_box_size;

        for br in box_corner_r..box_corner_r + vertical_box_size {
            for bc in box_corner_c..box_corner_c + horizontal_box_size {
                if br != r || bc != c {
                    if let Some(cell) = candidates.get_mut(&(br, bc)) {
                        cell.retain(|&x| x != val);
                    }
                }
            }
        }
    }
}

fn get_all_units(board_size: i32) -> Vec<Vec<(i32, i32)>> {
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
                    continue;
                }
                if let Some(cand) = candidates.get_mut(&(r,c)){
                    *cand = vec![val];
                    list[r as usize][c as usize] = val;
                }
                //list[r as usize][c as usize] = val;
                //println!("{:?} {:?}", (r, c), val);
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
            if let Some(cand) = candidates.get_mut(&(r,c)){
                *cand = vec![val[0]];
                list[r as usize][c as usize] = val[0];
            }
            //list[r as usize][c as usize] = val[0];
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
                                    //println!("{:?}", c);
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

pub fn apply_claiming_pair(candidates: &mut HashMap<(i32, i32), Vec<i32>>, board: &mut Vec<Vec<i32>>) -> bool {
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
                        if !boxes.contains(&((c / horizontal_box_size) as i32)){
                            boxes.push((c / horizontal_box_size) as i32)
                        }
                    }
                }
            }
            if boxes.len() == 1{
                let c = *boxes.iter().next().unwrap() as usize;
                let box_corner_r =  (r / vertical_box_size) * vertical_box_size;
                let box_corner_c = c*horizontal_box_size;
                for br in box_corner_r..box_corner_r + vertical_box_size {
                    for bc in box_corner_c..box_corner_c + horizontal_box_size {
                        if br != r {
                            if let Some(cell) = candidates.get_mut(&(br as i32, bc as i32)){
                                if cell.contains(&(val as i32)) && cell.len() > 1{
                                    cell.retain(|&x| x != val as i32);
                                    //println!("{:?} {:?}", (box_corner_r + br, box_corner_c + bc), val);
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
        for val in 1..=board.len(){
            let mut boxes:Vec<i32> = vec![];
            for r in 0..board.len(){
                if let Some(cell_cands) = candidates.get(&(r as i32, c as i32)){
                    if cell_cands.contains(&(val as i32)){
                        if !boxes.contains(&((r / (vertical_box_size)) as i32)){
                            boxes.push((r / (vertical_box_size)) as i32)
                        }
                    }
                }
            }

            if boxes.len() == 1{
                let r = *boxes.iter().next().unwrap() as usize;
                let box_corner_r = r*(vertical_box_size);
                let box_corner_c = (c / horizontal_box_size) * horizontal_box_size;
                for br in box_corner_r..box_corner_r + vertical_box_size {
                    for bc in box_corner_c..box_corner_c + horizontal_box_size {
                        if bc != c {
                            if let Some(cell) = candidates.get_mut(&(br as i32, bc as i32)){
                                if cell.contains(&(val as i32)) && cell.len() > 1{
                                    cell.retain(|&x| x != val as i32);
                                    //println!("{:?} {:?}", (box_corner_r + br, box_corner_c + bc), val);
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
                                    if cell_cands.contains(&(val as i32)) && cell_cands.len() != 1 && board[r][c] == 0 {
                                        let index = cell_cands.iter().position(|n| *n == val as i32).unwrap();
                                        cell_cands.remove(index);
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

fn display_candidates(candidates: &HashMap<(i32, i32), Vec<i32>>) {
    let size = 9;
    let box_size = 3;

    for r in 0..size {
        if r % box_size == 0 && r != 0 {
            println!("{}", "-".repeat((box_size * 8) + 2));
        }

        for c in 0..size {
            if c % box_size == 0 && c != 0 {
                print!("| ");
            }
            let empty_vec: Vec<i32> = Vec::new();
            let val = candidates.get(&(r as i32, c as i32)).unwrap_or(&empty_vec);
            if val.len() == 1 {
                // Filled cell
                print!("{:3}     ", val[0]);
            } else if val.is_empty() {
                // Invalid cell
                print!(" .      ");
            } else {
                // Display candidates like 123
                let mut val_str = val.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("");
                if val_str.len() > 6 {
                    val_str.truncate(6); // trim for neatness
                    val_str.push('+');
                }
                print!("{:<7}", val_str);
            }
        }
        println!();
    }
}
