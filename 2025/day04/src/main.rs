use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn to_coordinates(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

struct Directions;
impl IntoIterator for Directions {
    type Item = Direction;
    type IntoIter = std::vec::IntoIter<Direction>;
    fn into_iter(self) -> Self::IntoIter {
        vec![
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
        .into_iter()
    }
}

fn read_file_as_matrix(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    Ok(matrix)
}

fn pick_rolls(matrix: &Vec<Vec<char>>) -> (Vec<Vec<char>>, u32) {
    let mut new_matrix = matrix.clone();
    let rows_len = matrix.len();
    let cols_len = matrix[0].len();
    let mut result1 = 0;
    for (r, row) in matrix.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            // println!("Row: {}, Col: {}, Value: {}", r, c, col);
            let v = matrix[r][c];
            if v == '.' {
                continue;
            }
            let mut neighbour_count = 0;
            for d in Directions {
                let (dr, dc) = d.to_coordinates();
                let new_r = r as i32 + dr;
                let new_c = c as i32 + dc;
                if new_r >= 0 && new_r < rows_len as i32 && new_c >= 0 && new_c < cols_len as i32 {
                    let neighbor_value = matrix[new_r as usize][new_c as usize];
                    if neighbor_value == '@' {
                        neighbour_count += 1;
                    }
                }
            }
            if neighbour_count < 4 {
                result1 += 1;
                new_matrix[r][c] = '.';
            }
        }
    }
    (new_matrix, result1)
}

fn main() -> io::Result<()> {
    let filename = "data/input";
    // let filename = "data/input2";
    let mut matrix = read_file_as_matrix(filename)?;
    let mut result2 = 0;
    let (_, result1) = pick_rolls(&matrix);
    loop {
        let (new_matrix, iteration_result) = pick_rolls(&matrix);
        if iteration_result == 0 {
            break;
        }
        result2 += iteration_result;
        matrix = new_matrix;
    }
    println!("Result part one: {}", result1);
    println!("Result part two: {}", result2);
    Ok(())
}
