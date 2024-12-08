use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn load_data(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    let mut data = Vec::new();
    for line in lines {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        data.push(row);
    }
    Ok(data)
}

fn is_out_of_bounds<T>(current_pos: (i32, i32), matrix: &[Vec<T>]) -> bool {
    let (x, y) = current_pos;
    let (rows, cols) = (matrix.len(), matrix[0].len());
    x < 0 || y < 0 || x >= rows as i32 || y >= cols as i32
}

fn find_antinodes(
    matrix: &Vec<Vec<char>>,
    current_pos: (i32, i32),
    current_symbol: char,
) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();
    let (cx, cy) = current_pos;
    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == current_symbol {
                // println!("Symbol {} at: ({}, {})", c, x, y);
                // let (dx, dy) = ((x as i32 - cx).abs(), (y as i32 - cy).abs());
                let (dx, dy) = ((x as i32 - cx), (y as i32 - cy));
                if dx == 0 && dy == 0 {
                    continue;
                }

                // println!("dx: {}, dy: {}", dx, dy);
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);

                // println!("nx: {}, ny: {}", nx, ny);
                if !is_out_of_bounds((nx, ny), matrix)
                    && matrix[ny as usize][nx as usize] != *c
                    && are_points_collinear((cx, cy), (x as i32, y as i32), (nx, ny))
                {
                    antinodes.push((nx, ny));
                }
            }
        }
    }
    antinodes
}

fn find_antinodes_2(
    matrix: &Vec<Vec<char>>,
    current_pos: (i32, i32),
    current_symbol: char,
) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();
    let (cx, cy) = current_pos;
    antinodes.push((cx, cy));
    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == current_symbol {
                let mut tx = x as i32;
                let mut ty = y as i32;
                loop {
                    let (dx, dy) = ((tx as i32 - cx), (ty as i32 - cy));
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if !is_out_of_bounds((nx, ny), matrix)
                        && matrix[ny as usize][nx as usize] != *c
                        && are_points_collinear((cx, cy), (x as i32, y as i32), (nx, ny))
                    {
                        antinodes.push((nx, ny));
                    }
                    tx = nx;
                    ty = ny;
                    if is_out_of_bounds((tx, ty), matrix) {
                        break;
                    }
                } // loop
            }
        }
    }
    antinodes
}

fn are_points_collinear(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    // Determinant method to check if points are collinear
    (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) == 0
}

fn main() {
    // let file_path = "input_small";
    let file_path = "input";
    let matrix = load_data(file_path).unwrap();
    let mut result_matrix = matrix.clone();
    let mut result_matrix2 = matrix.clone();
    let mut map = HashMap::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                let antinodes_list = map.entry(*c).or_insert(HashSet::new());
                let current_symbol = *c;
                let current_pos = (x as i32, y as i32);
                // println!("Symbol {} at: ({}, {})", c, current_pos.0, current_pos.1);
                let antinodes = find_antinodes(&matrix, current_pos, current_symbol);
                antinodes_list.extend(antinodes);
            }
        }
    }
    let mut count = 0;
    for (_, value) in &map {
        for (x, y) in value {
            if result_matrix[*y as usize][*x as usize] != '#' {
                count += 1;
                result_matrix[*y as usize][*x as usize] = '#';
            }
        }
    }

    println!("Result 1: Antinodes count: {}", count);

    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                let antinodes_list = map.entry(*c).or_insert(HashSet::new());
                let current_symbol = *c;
                let current_pos = (x as i32, y as i32);
                // println!("Symbol {} at: ({}, {})", c, current_pos.0, current_pos.1);
                let antinodes = find_antinodes_2(&matrix, current_pos, current_symbol);
                antinodes_list.extend(antinodes);
            }
        }
    }
    let mut count = 0;
    for (_, value) in &map {
        for (x, y) in value {
            if result_matrix2[*y as usize][*x as usize] != '#' {
                count += 1;
                result_matrix2[*y as usize][*x as usize] = '#';
            }
        }
    }

    println!("Result2: Antinodes count: {}", count);
}
