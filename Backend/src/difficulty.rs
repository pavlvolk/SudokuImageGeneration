use std::collections::HashMap;

fn rate_difficulty(list: &Vec<i32>, size: i32) -> f32{
    return 0.0
}

fn serate(list: &Vec<Vec<i32>>) -> f64{
    let board_size = list.len().isqrt();
    let hidden_single = 1.2;
    let naked_single = 2.3;
    let pointing = 2.6;
    let claiming = 2.8;
    let x_wing = 4.0;
    let swordfish = 5.0;



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

fn hidden_single(candidates: &HashMap<(i32, i32), Vec<i32>>, board_size: i32, list: &mut Vec<Vec<i32>>) -> bool{
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
                    list[r as usize][c as usize] = val;
                    return true;
                }
            }
        }
    }
    false
}

fn naked_single(candidates: &HashMap<(i32, i32), Vec<i32>>, list: &mut Vec<Vec<i32>>) -> bool {
    for ((r, c), val) in candidates{
        if val.len() == 1{
            list[*r as usize][*c as usize] = val[0];
            return true;
        }
    }
    false
}