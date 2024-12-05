use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn build_map_from_file(file_path: &str) -> io::Result<HashMap<i32, Vec<i32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<i32> = line.split('|').map(|s| s.trim().parse().unwrap()).collect();
        if let [key, value] = parts.as_slice() {
            map.entry(*key).or_insert_with(Vec::new).push(*value);
        }
    }
    for values in map.values_mut() {
        values.sort();
    }

    Ok(map)
}

fn build_updates_from_file(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let row: Vec<i32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
        matrix.push(row);
    }
    Ok(matrix)
}

fn main() -> io::Result<()> {
    let file_path = "input1_all";
    let updates_file_path = "input2_all";
    let map = build_map_from_file(file_path)?;
    let matrix = build_updates_from_file(updates_file_path)?;
    let mut results_part_one: Vec<Vec<i32>> = Vec::new();
    let mut results_part_two: Vec<Vec<i32>> = Vec::new();
    for row in &matrix {
        let mut updated_row = row.clone();
        let mut updated = false;
        loop {
            let mut swaped = false;
            let i_row = updated_row.clone();
            for (i1, v1) in i_row.iter().enumerate() {
                if map.get(&v1).is_none() {
                    continue;
                }
                let afters = map.get(&v1).unwrap();
                if afters.is_empty() {
                    continue;
                }
                for (i2, v2) in i_row.iter().enumerate() {
                    if afters.contains(&v2) {
                        if i1 > i2 {
                            updated = true;
                            swaped = true;
                            updated_row.swap(i1, i2);
                        }
                    }
                }
            }
            if !swaped {
                break;
            }
        }
        // println!("{:?} -> {:?}", row, updated_row);
        if updated {
            results_part_two.push(updated_row);
        } else {
            results_part_one.push(updated_row);
        }
    }
    let mut count = 0;
    for row in &results_part_one {
        let middle = row[row.len() / 2];
        count += middle;
    }
    println!("Result part one: {}", count);
    count = 0;
    for row in &results_part_two {
        let middle = row[row.len() / 2];
        count += middle;
    }
    println!("Result part two: {}", count);

    Ok(())
}
