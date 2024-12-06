use std::fs::File;
use std::io::{self, BufRead};

fn load_data(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let row: Vec<i32> = line.split(' ').map(|s| s.trim().parse().unwrap()).collect();
        matrix.push(row);
    }

    Ok(matrix)
}

fn test_row(row: &Vec<i32>) -> bool {
    let row_s = row.len();
    let row2 = row.clone();
    let mut increase: bool = true;
    for (i, v) in row.iter().enumerate() {
        if i == row_s - 1 {
            break;
        }
        let v2 = row2[i + 1];
        // increase check
        if v >= &v2 {
            if increase && (i > 0) {
                return false;
            }
            increase = false;
        } else {
            if !increase {
                return false;
            }
            increase = true;
        }
        // diff check
        if (v - v2).abs() > 3 || (v - v2).abs() == 0 {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let file_path = "input";
    let matrix = load_data(file_path)?;
    let matrix_s = matrix.len();
    let mut safe_rows: Vec<Vec<i32>> = Vec::new();
    let mut unsafe_rows: Vec<Vec<i32>> = Vec::new();

    // Part 1
    for row in matrix {
        if test_row(&row) {
            safe_rows.push(row.clone());
        } else {
            unsafe_rows.push(row.clone());
        }
    }
    println!("----Part 1----");
    let safe_row_count = safe_rows.len();
    let unsafe_row_count = unsafe_rows.len();
    println!("Safe Rows: {}", safe_row_count);
    println!("Unsafe Rows: {}", unsafe_row_count);
    println!("Total Rows: {}", matrix_s);
    // Part 2
    for row in unsafe_rows {
        for i in 0..row.len() {
            let mut row2 = row.clone();
            row2.remove(i);
            if test_row(&row2) {
                safe_rows.push(row2.clone());
                break;
            }
        }
    }
    println!("----Part 2----");
    let safe_row_count = safe_rows.len();
    println!("Safe Rows: {}", safe_row_count);
    println!("Unsafe Rows: {}", matrix_s - safe_row_count);
    println!("Total Rows: {}", matrix_s);
    Ok(())
}
